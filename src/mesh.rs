mod facet;
mod poly;
mod region;
mod space;
mod texture;

// If I hadn't written this wrapper, I would have made dozens of errors writing this code.
// I would have been unable to debug them all at the same time, and would have given up.
// Instead, I made the compiler catch all my errors as they happened!
use vector_spaces::*;

use nalgebra as na;

pub use crate::render::Vertex;

use facet::Facet;

pub struct Mesh {
    facets: Vec<Facet<space::Facet>>,
}

impl Mesh {
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

impl Mesh {
    pub fn project(self, p: na::Matrix5<f64>) -> Vec<Vertex> {
        facet::do_all_occlusions(self.facets, Linear(p))
            .into_iter()
            .flat_map(|tex| {
                let (embedding, vertices) = tex.get_vertices();
                vertices.map(move |(a, sign)| Vertex {
                    pos: (embedding * a).0,
                    sign,
                    texcoord: a.0,
                })
            })
            .collect()
    }
}
