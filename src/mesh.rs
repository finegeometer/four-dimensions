mod region;
mod space;
mod texture;

mod poly1;
mod poly2;
use poly2 as poly;

// If I hadn't written this wrapper, I would have made dozens of errors writing this code.
// I would have been unable to debug them all at the same time, and would have given up.
// Instead, I made the compiler catch all my errors as they happened!
use vector_spaces::*;

use nalgebra as na;
use num_traits::{Inv, One};

pub use crate::render::Vertex;

pub struct Mesh {
    facets: Vec<Facet<space::Facet>>,
}

impl Mesh {
    pub fn hypercube() -> Self {
        Mesh {
            facets: vec![
                Facet::cube(na::Matrix5x4::new(
                    2., 0., 0., -1., 0., 2., 0., -1., 0., 0., 2., -1., 0., 0., 0., -1., 0., 0., 0.,
                    1.,
                )),
                Facet::cube(na::Matrix5x4::new(
                    2., 0., 0., -1., 0., 2., 0., -1., 0., 0., 0., -1., 0., 0., 2., -1., 0., 0., 0.,
                    1.,
                )),
                Facet::cube(na::Matrix5x4::new(
                    2., 0., 0., -1., 0., 0., 0., -1., 0., 2., 0., -1., 0., 0., 2., -1., 0., 0., 0.,
                    1.,
                )),
                Facet::cube(na::Matrix5x4::new(
                    0., 0., 0., -1., 2., 0., 0., -1., 0., 2., 0., -1., 0., 0., 2., -1., 0., 0., 0.,
                    1.,
                )),
                Facet::cube(na::Matrix5x4::new(
                    -2., 0., 0., 1., 0., -2., 0., 1., 0., 0., -2., 1., 0., 0., 0., 1., 0., 0., 0.,
                    1.,
                )),
                Facet::cube(na::Matrix5x4::new(
                    -2., 0., 0., 1., 0., -2., 0., 1., 0., 0., 0., 1., 0., 0., -2., 1., 0., 0., 0.,
                    1.,
                )),
                Facet::cube(na::Matrix5x4::new(
                    -2., 0., 0., 1., 0., 0., 0., 1., 0., -2., 0., 1., 0., 0., -2., 1., 0., 0., 0.,
                    1.,
                )),
                Facet::cube(na::Matrix5x4::new(
                    0., 0., 0., 1., -2., 0., 0., 1., 0., -2., 0., 1., 0., 0., -2., 1., 0., 0., 0.,
                    1.,
                )),
            ],
        }
    }

    /// Create a mesh from a 16x16x16x16 world.
    pub fn new(blocks: &[[[[bool; 16]; 16]; 16]; 16]) -> Self {
        let mut facets = Vec::new();
        for i1 in 0..16 {
            for i2 in 0..16 {
                for i3 in 0..16 {
                    for i4 in 0..15 {
                        if blocks[i1][i2][i3][i4] ^ blocks[i1][i2][i3][i4 + 1] {
                            facets.push(Facet::cube(na::Matrix5x4::new(
                                1.,
                                0.,
                                0.,
                                i1 as f64,
                                0.,
                                1.,
                                0.,
                                i2 as f64,
                                0.,
                                0.,
                                1.,
                                i3 as f64,
                                0.,
                                0.,
                                0.,
                                i4 as f64 + 1.,
                                0.,
                                0.,
                                0.,
                                1.,
                            )))
                        }

                        if blocks[i2][i3][i4][i1] ^ blocks[i2][i3][i4 + 1][i1] {
                            facets.push(Facet::cube(na::Matrix5x4::new(
                                0.,
                                1.,
                                0.,
                                i2 as f64,
                                0.,
                                0.,
                                1.,
                                i3 as f64,
                                0.,
                                0.,
                                0.,
                                i4 as f64 + 1.,
                                1.,
                                0.,
                                0.,
                                i1 as f64,
                                0.,
                                0.,
                                0.,
                                1.,
                            )))
                        }

                        if blocks[i3][i4][i1][i2] ^ blocks[i3][i4 + 1][i1][i2] {
                            facets.push(Facet::cube(na::Matrix5x4::new(
                                0.,
                                0.,
                                1.,
                                i3 as f64,
                                0.,
                                0.,
                                0.,
                                i4 as f64 + 1.,
                                1.,
                                0.,
                                0.,
                                i1 as f64,
                                0.,
                                1.,
                                0.,
                                i2 as f64,
                                0.,
                                0.,
                                0.,
                                1.,
                            )))
                        }

                        if blocks[i4][i1][i2][i3] ^ blocks[i4 + 1][i1][i2][i3] {
                            facets.push(Facet::cube(na::Matrix5x4::new(
                                0.,
                                0.,
                                0.,
                                i4 as f64 + 1.,
                                1.,
                                0.,
                                0.,
                                i1 as f64,
                                0.,
                                1.,
                                0.,
                                i2 as f64,
                                0.,
                                0.,
                                1.,
                                i3 as f64,
                                0.,
                                0.,
                                0.,
                                1.,
                            )))
                        }
                    }
                }
            }
        }
        Self { facets }
    }
}

impl Facet<space::Facet> {
    fn cube(embedding: na::Matrix5x4<f64>) -> Self {
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
            // texture: {
            //     let mut out = vec![];

            //     for &x in &[0.05, 0.2, 0.35, 0.5, 0.65, 0.8, 0.95] {
            //         for &y in &[0.05, 0.95] {
            //             out.push(texture::Texture::new([0.05, x, y], [0.95, x, y]));
            //             out.push(texture::Texture::new([0.05, y, x], [0.95, y, x]));
            //             out.push(texture::Texture::new([x, y, 0.05], [x, y, 0.95]));
            //             out.push(texture::Texture::new([y, x, 0.05], [y, x, 0.95]));
            //             out.push(texture::Texture::new([y, 0.05, x], [y, 0.95, x]));
            //             out.push(texture::Texture::new([x, 0.05, y], [x, 0.95, y]));
            //         }
            //     }

            //     out
            // }
            texture: vec![
                /* TXPRBLM: IF USING 1D TEXTURES, USE THESE */
                // texture::Texture::new([0.05, 0.05, 0.05], [0.95, 0.05, 0.05]),
                // texture::Texture::new([0.05, 0.05, 0.05], [0.05, 0.95, 0.05]),
                // texture::Texture::new([0.05, 0.05, 0.05], [0.05, 0.05, 0.95]),
                // texture::Texture::new([0.95, 0.95, 0.05], [0.05, 0.95, 0.05]),
                // texture::Texture::new([0.95, 0.95, 0.05], [0.95, 0.05, 0.05]),
                // texture::Texture::new([0.95, 0.95, 0.05], [0.95, 0.95, 0.95]),
                // texture::Texture::new([0.95, 0.05, 0.95], [0.05, 0.05, 0.95]),
                // texture::Texture::new([0.95, 0.05, 0.95], [0.95, 0.95, 0.95]),
                // texture::Texture::new([0.95, 0.05, 0.95], [0.95, 0.05, 0.05]),
                // texture::Texture::new([0.05, 0.95, 0.95], [0.95, 0.95, 0.95]),
                // texture::Texture::new([0.05, 0.95, 0.95], [0.05, 0.05, 0.95]),
                // texture::Texture::new([0.05, 0.95, 0.95], [0.05, 0.95, 0.05]),
                /* TXPRBLM: IF USING 2D TEXTURES, USE THESE */
                texture::Texture::new([0.05, 0.05, 0.05], [0.95, 0.05, 0.05], [0.05, 0.95, 0.05]),
                texture::Texture::new([0.05, 0.05, 0.05], [0.05, 0.95, 0.05], [0.05, 0.05, 0.95]),
                texture::Texture::new([0.05, 0.05, 0.05], [0.05, 0.05, 0.95], [0.95, 0.05, 0.05]),
                texture::Texture::new([0.95, 0.95, 0.95], [0.05, 0.95, 0.95], [0.95, 0.05, 0.95]),
                texture::Texture::new([0.95, 0.95, 0.95], [0.95, 0.05, 0.95], [0.95, 0.95, 0.05]),
                texture::Texture::new([0.95, 0.95, 0.95], [0.95, 0.95, 0.05], [0.05, 0.95, 0.95]),
            ],
        }
    }
}

struct Facet<A: Space<Dim = na::U3>> {
    embedding: Projective<f64, A, space::World>,
    region: region::Region<A>,
    texture: Vec<texture::Texture<A>>,
}

impl Mesh {
    pub fn project(mut self, p: na::Matrix5<f64>) -> Vec<Vertex> {
        let p: Projective<f64, space::World, (space::Screen, space::Depth)> = Linear(p);

        // This line does the occlusion.
        iterate_pairs(&mut self.facets, |f1, f2| occlude(p, f1, f2));

        self.facets
            .into_iter()
            .flat_map(
                |Facet {
                     embedding, texture, ..
                 }| {
                    texture.into_iter().flat_map(move |tex| {
                        let (e, vs) = tex.get_vertices();
                        let m: Projective<f64, space::Texture, space::Screen> =
                            Linear::one().first_output().make_projective() * p * embedding * e;
                        vs.map(move |(a, sign)| Vertex {
                            pos: (m * a).0,
                            sign,
                            /* TXPRBLM: IF USING 1D TEXTURES, USE THIS */
                            // texcoord: a.0.insert_row(1, 0.5),
                            /* TXPRBLM: IF USING 2D TEXTURES, USE THIS */
                            texcoord: a.0,
                        })
                    })
                },
            )
            .collect()
    }
}

fn occlude<OccluderSpace: Space<Dim = na::U3>, OccludeeSpace: Space<Dim = na::U3>>(
    p: Projective<f64, space::World, (space::Screen, space::Depth)>,
    occluder: &Facet<OccluderSpace>,
    occludee: &mut Facet<OccludeeSpace>,
) {
    let world_to_screen: Projective<f64, space::World, space::Screen> =
        Linear::one().first_output().make_projective() * p;

    // Calculate occluded region in Screen
    let screen_region: region::Region<space::Screen> = occluder
        .region
        .transform((world_to_screen * occluder.embedding).inv());

    // Calculate occluded region in (Screen, Depth)
    let mut screen_depth_region: region::Region<(space::Screen, space::Depth)> =
        screen_region.transform(Linear::one().first_output().make_projective());
    screen_depth_region.add_boundary(region_behind(p * occluder.embedding));

    // Occlude.
    for tex in occludee.texture.iter_mut() {
        tex.subtract_region(screen_depth_region.transform::<OccludeeSpace>(p * occludee.embedding))
    }
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

/// Iterate through all distict pairs of elements.
fn iterate_pairs<T>(slice: &mut [T], mut f: impl FnMut(&mut T, &mut T)) {
    for i in 1..slice.len() {
        let (v, w) = slice.split_at_mut(i);
        for mut b in w.iter_mut() {
            f(&mut v[i - 1], &mut b);
        }
        for mut a in v.iter_mut() {
            f(&mut w[0], &mut a);
        }
    }
}

#[test]
fn iterate_pairs_test() {
    let mut vec = vec![];

    iterate_pairs(&mut [1, 2, 3], |a, b| vec.push((*a, *b)));

    assert_eq!(vec, vec![(1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2)]);
}
