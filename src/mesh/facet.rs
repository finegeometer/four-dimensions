use nalgebra as na;
use vector_spaces::*;

use num_traits::{Inv, One};

use super::{region::Region, space, texture::Texture};

pub struct Facet<A: Space<Dim = na::U3>> {
    embedding: Projective<f64, A, space::World>,
    region: Region<A>,
    texture: Vec<Texture<A>>,
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
        }
    }
}

impl Facet<space::Facet> {
    fn into_screen_depth_space(
        self,
        p: Projective<f64, space::World, (space::Screen, space::Depth)>,
    ) -> (
        Vec<Texture<(space::Screen, space::Depth)>>,
        Region<(space::Screen, space::Depth)>,
    ) {
        let Facet {
            embedding,
            region,
            texture,
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

        (texture, region)
    }
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
    let (textures, regions): (Vec<Vec<_>>, Vec<_>) = facets
        .into_iter()
        .map(|f| {
            let (t, r) = f.into_screen_depth_space(p);
            (t, r)
        })
        .unzip();

    textures
        .into_iter()
        .enumerate()
        .map(|(i, t)| {
            (
                t,
                regions
                    .iter()
                    .enumerate()
                    .filter_map(|(j, r)| if i == j { None } else { Some(r.clone()) })
                    .collect(),
            )
        })
        .collect()
}

pub fn do_all_occlusions(
    facets: Vec<Facet<space::Facet>>,
    p: Projective<f64, space::World, (space::Screen, space::Depth)>,
) -> Vec<Texture<space::Screen>> {
    let mut out = Vec::new();
    for (textures, regions) in find_occlusions(facets, p) {
        for mut t in textures {
            for r in &regions {
                t.subtract_region(r);
            }
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
