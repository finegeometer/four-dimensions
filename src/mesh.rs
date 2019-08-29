use nalgebra as na;
pub use render_4d::Facet;
pub use render_4d::Mesh;

/// Create a mesh from a 16x16x16x16 world.
pub fn new(blocks: &[[[[bool; 16]; 16]; 16]; 16]) -> Mesh {
    let mut facets = Vec::new();
    for i1 in 0..16 {
        for i2 in 0..16 {
            for i3 in 0..16 {
                for i4 in 0..15 {
                    if blocks[i1][i2][i3][i4] ^ blocks[i1][i2][i3][i4 + 1] {
                        facets.push(Facet::new_cube(na::Matrix5x4::new(
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
                        facets.push(Facet::new_cube(na::Matrix5x4::new(
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
                        facets.push(Facet::new_cube(na::Matrix5x4::new(
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
                        facets.push(Facet::new_cube(na::Matrix5x4::new(
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
    Mesh { facets }
}
