use nalgebra as na;
use vector_spaces::*;

use num_traits::{Inv, One};

use super::{region::Region, space, texture::Texture};

pub struct Facet<A: Space<Dim = na::U3>> {
    embedding: Projective<f64, A, space::World>,
    region: Region<A>,
    texture: Vec<Texture<A>>,
    convex_hull: Vec<Projective<f64, (), A>>,
}

impl Facet<space::Facet> {
    pub fn cube(embedding: na::Matrix5x4<f64>) -> Self {
        Facet {
            embedding: Linear(embedding),
            region: vec![
                Linear(na::RowVector4::new(1., 0., 0., 0.)),
                Linear(na::RowVector4::new(0., 1., 0., 0.)),
                Linear(na::RowVector4::new(0., 0., 1., 0.)),
                Linear(na::RowVector4::new(-1., 0., 0., 1.)),
                Linear(na::RowVector4::new(0., -1., 0., 1.)),
                Linear(na::RowVector4::new(0., 0., -1., 1.)),
            ]
            .into(),
            texture: vec![
                Texture::new([0.05, 0.05, 0.05], [0.95, 0.05, 0.05], [0.05, 0.95, 0.05]),
                Texture::new([0.05, 0.05, 0.05], [0.05, 0.95, 0.05], [0.05, 0.05, 0.95]),
                Texture::new([0.05, 0.05, 0.05], [0.05, 0.05, 0.95], [0.95, 0.05, 0.05]),
                Texture::new([0.95, 0.95, 0.95], [0.05, 0.95, 0.95], [0.95, 0.05, 0.95]),
                Texture::new([0.95, 0.95, 0.95], [0.95, 0.05, 0.95], [0.95, 0.95, 0.05]),
                Texture::new([0.95, 0.95, 0.95], [0.95, 0.95, 0.05], [0.05, 0.95, 0.95]),
            ],
            convex_hull: vec![
                Linear(na::Vector4::new(0., 0., 0., 1.)),
                Linear(na::Vector4::new(1., 0., 0., 1.)),
                Linear(na::Vector4::new(0., 1., 0., 1.)),
                Linear(na::Vector4::new(1., 1., 0., 1.)),
                Linear(na::Vector4::new(0., 0., 1., 1.)),
                Linear(na::Vector4::new(1., 0., 1., 1.)),
                Linear(na::Vector4::new(0., 1., 1., 1.)),
                Linear(na::Vector4::new(1., 1., 1., 1.)),
            ],
        }
    }
}

impl Facet<space::Facet> {
    fn into_screen_depth_space(
        self,
        p: Projective<f64, space::World, (space::Screen, space::Depth)>,
    ) -> (
        Vec<Texture<(space::Screen, space::Depth)>>,
        (
            Region<(space::Screen, space::Depth)>,
            Vec<Projective<f64, (), space::Screen>>,
        ),
    ) {
        let Facet {
            embedding,
            region,
            texture,
            convex_hull,
        } = self;

        let m0: Projective<f64, space::Facet, (space::Screen, space::Depth)> = p * embedding;
        let m1: Projective<f64, (space::Screen, space::Depth), space::Screen> =
            Linear::one().first_output().make_projective();
        let m2: Projective<f64, space::Facet, space::Screen> = m1 * m0;
        let m3: Projective<f64, space::Screen, space::Facet> = m2.inv();
        let m4: Projective<f64, (space::Screen, space::Depth), space::Facet> = m3 * m1;

        let texture = texture.into_iter().map(|t| t.transform(m0)).collect();

        let mut region = region.transform(m4);
        region.add_boundary(region_behind(m0));

        let convex_hull = convex_hull.into_iter().map(|p| m2 * p).collect();

        (texture, (region, convex_hull))
    }
}

fn aabb(hull: &[Projective<f64, (), space::Screen>]) -> [[f64; 3]; 2] {
    let mut max_x = std::f64::MIN;
    let mut max_y = std::f64::MIN;
    let mut max_z = std::f64::MIN;
    let mut min_x = std::f64::MAX;
    let mut min_y = std::f64::MAX;
    let mut min_z = std::f64::MAX;

    for pt in hull {
        let [mut x, mut y, mut z, w]: [f64; 4] = pt.0.into();
        x /= w;
        y /= w;
        z /= w;

        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        min_z = min_z.min(z);
    }

    [[min_x, min_y, min_z], [max_x, max_y, max_z]]
}

/// Return pairs of (textures_on_one_facet, regions_that_may_occlude_that_facet)
// TODO: optimize this to decrease number of possible occlusions returned
fn find_occlusions(
    facets: Vec<Facet<space::Facet>>,
    p: Projective<f64, space::World, (space::Screen, space::Depth)>,
) -> Vec<(
    Vec<Texture<(space::Screen, space::Depth)>>,
    Vec<Region<(space::Screen, space::Depth)>>,
)> {
    let (textures, regions_and_hulls): (Vec<Vec<_>>, Vec<_>) = facets
        .into_iter()
        .map(|f| {
            let (t, r) = f.into_screen_depth_space(p);
            (t, r)
        })
        .unzip();

    let mut out = Vec::new();

    let aabbs: Vec<[[f64; 3]; 2]> = regions_and_hulls.iter().map(|(_r, h)| aabb(h)).collect();

    for (i, tex) in textures.into_iter().enumerate() {
        let mut regions = Vec::new();

        for (j, (region, _)) in regions_and_hulls.iter().enumerate() {
            if i == j {
                continue;
            }

            let [min1, max1] = aabbs[i];
            let [min2, max2] = aabbs[j];

            if min1[0] > max2[0] {
                continue;
            }
            if min1[1] > max2[1] {
                continue;
            }
            if min1[2] > max2[2] {
                continue;
            }
            if min2[0] > max1[0] {
                continue;
            }
            if min2[1] > max1[1] {
                continue;
            }
            if min2[2] > max1[2] {
                continue;
            }

            regions.push(region.clone())
        }

        out.push((tex, regions))
    }

    out
    // textures
    //     .into_iter()
    //     .enumerate()
    //     .map(|(i, t)| {
    //         (
    //             t,
    //             regions
    //                 .iter()
    //                 .enumerate()
    //                 .filter_map(|(j, (region, hull))| {
    //                     if i == j { return None; }
    //                     if region.into_iter().any(|hp| hull.iter)
    //                     Some(region.clone())
    //                 })
    //                 .collect(),
    //         )
    //     })
    //     .collect()
}

pub fn do_all_occlusions(
    facets: Vec<Facet<space::Facet>>,
    p: Projective<f64, space::World, (space::Screen, space::Depth)>,
) -> Vec<Texture<space::Screen>> {
    let mut out = Vec::new();
    for (textures, regions) in find_occlusions(facets, p) {
        for mut t in textures {
            t.subtract_regions(&regions);
            out.push(t.transform(Linear::one().first_output().make_projective()));
        }
    }
    out
}

fn region_behind<A: Space<Dim = na::U3>>(
    embedding: Projective<f64, A, (space::Screen, space::Depth)>,
) -> Projective<f64, (space::Screen, space::Depth), ()> {
    // This is a cross product.
    let x1 = embedding.0.remove_row(0).determinant();
    let x2 = -embedding.0.remove_row(1).determinant();
    let x3 = embedding.0.remove_row(2).determinant();
    let x4 = -embedding.0.remove_row(3).determinant();
    let x5 = embedding.0.remove_row(4).determinant();
    let hyperplane = Linear(na::RowVector5::new(x1, x2, x3, x4, x5));

    // Make sure the positive (included in region) direction is in the positive depth direction.
    if x4 < 0. {
        -hyperplane
    } else {
        hyperplane
    }
}
