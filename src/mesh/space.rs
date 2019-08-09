use nalgebra as na;
use vector_spaces::*;

pub struct World;
impl Space for World {
    type Dim = na::U4;
}

pub struct Screen;
impl Space for Screen {
    type Dim = na::U3;
}

pub struct Depth;
impl Space for Depth {
    type Dim = na::U1;
}

pub struct Texture;
impl Space for Texture {
    /* TXPRBLM: IF USING 1D TEXTURES, USE THIS */
    type Dim = na::U1;
    /* TXPRBLM: IF USING 2D TEXTURES, USE THIS */
    // type Dim = na::U2;
}

pub struct Facet;
impl Space for Facet {
    type Dim = na::U3;
}
