use nalgebra as na;
pub use render_4d::Mesh;
use render_4d::*;
use std::convert::TryFrom;

const N: usize = 3;

pub struct World {
    blocks: [[[[Block; N]; N]; N]; N],
}

#[derive(Copy, Clone)]
pub enum Block {
    Air,
    Block,
}

impl World {
    pub fn new() -> Self {
        Self {
            blocks: [[[[Block::Block; N]; N]; N]; N],
        }
    }
    pub fn block(&self, pos: [isize; 4]) -> &Block {
        (|| {
            self.blocks
                .get(usize::try_from(pos[0]).ok()?)?
                .get(usize::try_from(pos[1]).ok()?)?
                .get(usize::try_from(pos[2]).ok()?)?
                .get(usize::try_from(pos[3]).ok()?)
        })()
        .unwrap_or(&Block::Air)
    }
    pub fn block_mut(&mut self, pos: [isize; 4]) -> Option<&mut Block> {
        self.blocks
            .get_mut(usize::try_from(pos[0]).ok()?)?
            .get_mut(usize::try_from(pos[1]).ok()?)?
            .get_mut(usize::try_from(pos[2]).ok()?)?
            .get_mut(usize::try_from(pos[3]).ok()?)
    }
}

impl Block {
    fn is_transparent(self) -> bool {
        match self {
            Block::Air => true,
            Block::Block => false,
        }
    }
}

impl World {
    pub fn mesh(&self) -> Mesh {
        let mut facets = Vec::new();

        for &(mut dimensions) in &[[0, 1, 2, 3], [1, 0, 3, 2], [2, 3, 0, 1], [3, 2, 1, 0]] {
            for &dir in &[false, true] {
                if dir {
                    dimensions.swap(0, 1);
                }

                for i3 in 0..=N as isize {
                    let mut embedding = na::Matrix5x4::zeros();
                    embedding[(dimensions[0], 0)] = 1.;
                    embedding[(dimensions[1], 1)] = 1.;
                    embedding[(dimensions[2], 2)] = 1.;
                    embedding[(dimensions[3], 3)] = i3 as f64;
                    embedding[(4, 3)] = 1.;

                    let mut regions = Vec::new();
                    for i0 in 0..N as isize {
                        for i1 in 0..N as isize {
                            for i2 in 0..N as isize {
                                let mut pos = [0, 0, 0, 0];
                                pos[dimensions[0]] = i0;
                                pos[dimensions[1]] = i1;
                                pos[dimensions[2]] = i2;
                                pos[dimensions[3]] = i3;

                                if self.block(pos).is_transparent() ^ dir {
                                    pos[dimensions[3]] -= 1;
                                    if !self.block(pos).is_transparent() ^ dir {
                                        regions.push(vec![
                                            na::RowVector4::new(1., 0., 0., -i0 as f64),
                                            na::RowVector4::new(0., 1., 0., -i1 as f64),
                                            na::RowVector4::new(0., 0., 1., -i2 as f64),
                                            na::RowVector4::new(-1., 0., 0., i0 as f64 + 1.),
                                            na::RowVector4::new(0., -1., 0., i1 as f64 + 1.),
                                            na::RowVector4::new(0., 0., -1., i2 as f64 + 1.),
                                        ])
                                    }
                                }
                            }
                        }
                    }

                    let mut texture = Vec::new();
                    for &dimensions2 in &[[0, 1, 2], [1, 2, 0], [2, 0, 1]] {
                        for &dir2 in &[false, true] {
                            for j2 in 0..N as isize {
                                let mut embedding = na::Matrix4x3::zeros();
                                embedding[(dimensions2[0], 0)] = 1.;
                                embedding[(dimensions2[1], 1)] = 1.;
                                embedding[(dimensions2[2], 2)] =
                                    j2 as f64 + if dir2 { 0.95 } else { 0.05 };
                                embedding[(3, 2)] = 1.;

                                let mut edge_loops = Vec::new();

                                for j0 in 0..N as isize {
                                    for j1 in 0..N as isize {
                                        let mut pos = [0, 0, 0, 0];
                                        pos[dimensions[dimensions2[0]]] = j0;
                                        pos[dimensions[dimensions2[1]]] = j1;
                                        pos[dimensions[dimensions2[2]]] = j2;
                                        pos[dimensions[3]] = i3;

                                        if self.block(pos).is_transparent() ^ dir {
                                            pos[dimensions[3]] -= 1;
                                            if !self.block(pos).is_transparent() ^ dir {
                                                edge_loops.push(vec![
                                                    polygon3::Line::try_from_f64_array([
                                                        1.0,
                                                        0.0,
                                                        -j0 as f64 + 0.05,
                                                    ])
                                                    .unwrap(),
                                                    polygon3::Line::try_from_f64_array([
                                                        0.0,
                                                        1.0,
                                                        -j1 as f64 + 0.05,
                                                    ])
                                                    .unwrap(),
                                                    polygon3::Line::try_from_f64_array([
                                                        -1.0,
                                                        0.0,
                                                        j0 as f64 + 0.95,
                                                    ])
                                                    .unwrap(),
                                                    polygon3::Line::try_from_f64_array([
                                                        0.0,
                                                        -1.0,
                                                        j1 as f64 + 0.95,
                                                    ])
                                                    .unwrap(),
                                                ])
                                            }
                                        }
                                    }
                                }

                                let poly = polygon3::Polygon::try_from_edges(edge_loops).unwrap();
                                texture.push(Texture { embedding, poly });
                            }
                        }
                    }
                    facets.push(Facet {
                        embedding,
                        regions,
                        texture,
                    });
                }
            }
        }

        Mesh { facets }
    }
}
