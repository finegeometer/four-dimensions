use nalgebra as na;
use render_4d::Facet;
pub use render_4d::Mesh;
use std::convert::{identity, TryFrom};

pub struct World {
    blocks: [[[[Block; 3]; 3]; 3]; 3],
}

#[derive(Copy, Clone)]
pub enum Block {
    Air,
    Block,
}

impl World {
    pub fn new() -> Self {
        Self {
            blocks: [[[[Block::Block; 3]; 3]; 3]; 3],
        }
    }
    pub fn block(&self, pos: [isize; 4]) -> Option<&Block> {
        self.blocks
            .get(usize::try_from(pos[0]).ok()?)?
            .get(usize::try_from(pos[1]).ok()?)?
            .get(usize::try_from(pos[2]).ok()?)?
            .get(usize::try_from(pos[3]).ok()?)
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

#[rustfmt::skip]
impl World {
    pub fn mesh(&self) -> render_4d::Mesh {
    	Mesh {
    		facets: (0..3).flat_map(|i| (0..3).flat_map(move |j| (0..3).flat_map(move |k| (-1..3).flat_map(move |l| {
	    		let f1 = if !self.block([i,j,k,l]).unwrap_or(&Block::Air).is_transparent() && self.block([i,j,k,l+1]).unwrap_or(&Block::Air).is_transparent() {
	                Some(Facet::new_cube(na::Matrix5x4::new(
	                    1., 0., 0., i as f64,
	                    0., 1., 0., j as f64,
	                    0., 0., 1., k as f64,
	                    0., 0., 0., l as f64 + 1.,
	                    0., 0., 0., 1.,
	                )))
	    		} else {
	    			None
	    		};
	    		let f2 = if !self.block([j,i,l,k]).unwrap_or(&Block::Air).is_transparent() && self.block([j,i,l+1,k]).unwrap_or(&Block::Air).is_transparent() {
	                Some(Facet::new_cube(na::Matrix5x4::new(
	                    0., 1., 0., j as f64,
	                    1., 0., 0., i as f64,
	                    0., 0., 0., l as f64 + 1.,
	                    0., 0., 1., k as f64,
	                    0., 0., 0., 1.,
	                )))
	    		} else {
	    			None
	    		};
	    		let f3 = if !self.block([k,l,i,j]).unwrap_or(&Block::Air).is_transparent() && self.block([k,l+1,i,j]).unwrap_or(&Block::Air).is_transparent() {
	                Some(Facet::new_cube(na::Matrix5x4::new(
	                    0., 0., 1., k as f64,
	                    0., 0., 0., l as f64 + 1.,
	                    1., 0., 0., i as f64,
	                    0., 1., 0., j as f64,
	                    0., 0., 0., 1.,
	                )))
	    		} else {
	    			None
	    		};
	    		let f4 = if !self.block([l,k,j,i]).unwrap_or(&Block::Air).is_transparent() && self.block([l+1,k,j,i]).unwrap_or(&Block::Air).is_transparent() {
	                Some(Facet::new_cube(na::Matrix5x4::new(
	                    0., 0., 0., l as f64 + 1.,
	                    0., 0., 1., k as f64,
	                    0., 1., 0., j as f64,
	                    1., 0., 0., i as f64,
	                    0., 0., 0., 1.,
	                )))
	    		} else {
	    			None
	    		};

	    		let f5 = if self.block([j,i,k,l]).unwrap_or(&Block::Air).is_transparent() && !self.block([j,i,k,l+1]).unwrap_or(&Block::Air).is_transparent() {
	                Some(Facet::new_cube(na::Matrix5x4::new(
	                    0., 1., 0., j as f64,
	                    1., 0., 0., i as f64,
	                    0., 0., 1., k as f64,
	                    0., 0., 0., l as f64 + 1.,
	                    0., 0., 0., 1.,
	                )))
	    		} else {
	    			None
	    		};
	    		let f6 = if self.block([i,j,l,k]).unwrap_or(&Block::Air).is_transparent() && !self.block([i,j,l+1,k]).unwrap_or(&Block::Air).is_transparent() {
	                Some(Facet::new_cube(na::Matrix5x4::new(
	                    1., 0., 0., i as f64,
	                    0., 1., 0., j as f64,
	                    0., 0., 0., l as f64 + 1.,
	                    0., 0., 1., k as f64,
	                    0., 0., 0., 1.,
	                )))
	    		} else {
	    			None
	    		};
	    		let f7 = if self.block([k,l,j,i]).unwrap_or(&Block::Air).is_transparent() && !self.block([k,l+1,j,i]).unwrap_or(&Block::Air).is_transparent() {
	                Some(Facet::new_cube(na::Matrix5x4::new(
	                    0., 0., 1., k as f64,
	                    0., 0., 0., l as f64 + 1.,
	                    0., 1., 0., j as f64,
	                    1., 0., 0., i as f64,
	                    0., 0., 0., 1.,
	                )))
	    		} else {
	    			None
	    		};
	    		let f8 = if self.block([l,k,i,j]).unwrap_or(&Block::Air).is_transparent() && !self.block([l+1,k,i,j]).unwrap_or(&Block::Air).is_transparent() {
	                Some(Facet::new_cube(na::Matrix5x4::new(
	                    0., 0., 0., l as f64 + 1.,
	                    0., 0., 1., k as f64,
	                    1., 0., 0., i as f64,
	                    0., 1., 0., j as f64,
	                    0., 0., 0., 1.,
	                )))
	    		} else {
	    			None
	    		};


	    		vec![f1, f2, f3, f4, f5, f6, f7, f8].into_iter().filter_map(identity)
	    	})))).collect()
    	}
    }
}
