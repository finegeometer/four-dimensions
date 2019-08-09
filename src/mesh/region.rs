use nalgebra as na;
use vector_spaces::*;

type Hyperplane<A> = Projective<f64, A, ()>;

pub struct Region<A: Space>
where
    A::Dim: na::DimNameAdd<na::U1>,
    na::DefaultAllocator: na::allocator::Allocator<f64, na::U1, <Homogeneous<A> as Space>::Dim>,
{
    boundaries: Vec<Hyperplane<A>>,
}

impl<A: Space> From<Vec<Hyperplane<A>>> for Region<A>
where
    A::Dim: na::DimNameAdd<na::U1>,
    na::DefaultAllocator: na::allocator::Allocator<f64, na::U1, <Homogeneous<A> as Space>::Dim>,
{
    fn from(boundaries: Vec<Hyperplane<A>>) -> Self {
        Region { boundaries }
    }
}

impl<A: Space> Into<Vec<Hyperplane<A>>> for Region<A>
where
    A::Dim: na::DimNameAdd<na::U1>,
    na::DefaultAllocator: na::allocator::Allocator<f64, na::U1, <Homogeneous<A> as Space>::Dim>,
{
    fn into(self) -> Vec<Hyperplane<A>> {
        self.boundaries
    }
}

/// Return hyperplanes such that the intersection of their positive regions is the region.
impl<A: Space> IntoIterator for Region<A>
where
    A::Dim: na::DimNameAdd<na::U1>,
    na::DefaultAllocator: na::allocator::Allocator<f64, na::U1, <Homogeneous<A> as Space>::Dim>,
{
    type Item = Hyperplane<A>;
    type IntoIter = <Vec<Hyperplane<A>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        let boundaries: Vec<Hyperplane<A>> = self.into();
        boundaries.into_iter()
    }
}

impl<A: Space> Region<A>
where
    A::Dim: na::DimNameAdd<na::U1>,
    na::DefaultAllocator: na::allocator::Allocator<f64, na::U1, <Homogeneous<A> as Space>::Dim>,
{
    pub fn transform<B: Space>(&self, p: Projective<f64, B, A>) -> Region<B>
    where
        B::Dim: na::DimNameAdd<na::U1>,
        na::DefaultAllocator: na::allocator::Allocator<f64, na::U1, <Homogeneous<B> as Space>::Dim>,
        na::DefaultAllocator: na::allocator::Allocator<
            f64,
            <Homogeneous<A> as Space>::Dim,
            <Homogeneous<B> as Space>::Dim,
        >,
        Projective<f64, A, ()>: Copy,
        Projective<f64, B, A>: Copy,
    {
        Region {
            boundaries: self.boundaries.iter().map(|&h| h * p).collect(),
        }
    }

    pub fn add_boundary(&mut self, h: Hyperplane<A>) {
        self.boundaries.push(h)
    }
}

impl<A: Space> core::fmt::Debug for Region<A>
where
    A::Dim: na::DimNameAdd<na::U1>,
    na::DefaultAllocator: na::allocator::Allocator<f64, na::U1, <Homogeneous<A> as Space>::Dim>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.boundaries.fmt(f)
    }
}
