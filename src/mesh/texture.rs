use nalgebra as na;
use vector_spaces::*;

pub struct Texture<A: Space>
where
    A::Dim: na::DimNameAdd<na::U1>,
    na::DefaultAllocator: na::allocator::Allocator<f64, <Homogeneous<A> as Space>::Dim, na::U3>,
{
    embedding: Projective<f64, super::space::Texture, A>,
    poly: super::poly::Poly<super::space::Texture>,
}

impl<A: Space> Texture<A>
where
    A::Dim: na::DimNameAdd<na::U1>,
    na::DefaultAllocator: na::allocator::Allocator<f64, <Homogeneous<A> as Space>::Dim, na::U3>,
{
    pub fn transform<B: Space>(self, p: Projective<f64, A, B>) -> Texture<B>
    where
        B::Dim: na::DimNameAdd<na::U1>,
        na::DefaultAllocator: na::allocator::Allocator<f64, <Homogeneous<B> as Space>::Dim, na::U3>,
        na::DefaultAllocator: na::allocator::Allocator<
            f64,
            <Homogeneous<B> as Space>::Dim,
            <Homogeneous<A> as Space>::Dim,
        >,
    {
        let Texture { embedding, poly } = self;
        Texture {
            embedding: p * embedding,
            poly,
        }
    }

    pub fn subtract_region(&mut self, region: super::region::Region<A>)
    where
        na::DefaultAllocator: na::allocator::Allocator<f64, na::U1, <Homogeneous<A> as Space>::Dim>,
        Projective<f64, super::space::Texture, A>: Copy,
        Projective<f64, A, ()>: Copy,
    {
        let region: super::region::Region<super::space::Texture> =
            region.transform::<super::space::Texture>(self.embedding);
        self.poly.subtract_region(region)
    }

    pub fn get_vertices(
        self,
    ) -> (
        Projective<f64, super::space::Texture, A>,
        impl Iterator<Item = (Projective<f64, (), super::space::Texture>, f64)>,
    ) {
        let Self { embedding, poly } = self;
        (embedding, poly.get_vertices())
    }
}

impl Texture<super::space::Facet> {
    pub fn new([a1, a2, a3]: [f64; 3], [b1, b2, b3]: [f64; 3], [c1, c2, c3]: [f64; 3]) -> Self {
        Texture {
            embedding: Linear(na::Matrix4x3::new(
                b1 - a1,
                c1 - a1,
                a1,
                b2 - a2,
                c2 - a2,
                a2,
                b3 - a3,
                c3 - a3,
                a3,
                0.,
                0.,
                1.,
            )),
            poly: super::poly::Poly::square([0., 0.], [1., 1.]),
        }
    }
}
