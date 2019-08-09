use super::region::Region;
use nalgebra as na;
use vector_spaces::*;

/// A one-dimensional (multi-)polychoron
pub struct Poly<A: Space<Dim = na::U1>> {
    _phantom: core::marker::PhantomData<A>,
    lines: Vec<[f64; 2]>,
}

/// A convex region of a one-dimensional space is either empty or a line segment, ray, or line.
/// Returns the endpoints of that line segment as Some([smaller_endpoint, larger_endpoint]),
/// or returns None if the region is empty.
/// In the case of a ray or line, infinite ends are replaced by std::f64::{MIN, MAX}.
fn region_to_line<A: Space<Dim = na::U1>>(region: Region<A>) -> Option<[f64; 2]> {
    let mut a: f64 = std::f64::MIN;
    let mut b: f64 = std::f64::MAX;

    for hyperplane in region {
        let x = hyperplane.0[0];
        let y = hyperplane.0[1];

        match x.partial_cmp(&0.) {
            Some(std::cmp::Ordering::Greater) => {
                a = a.max(-y / x);
            }
            Some(std::cmp::Ordering::Less) => {
                b = b.min(-y / x);
            }
            Some(std::cmp::Ordering::Equal) => {
                if y < 0. {
                    return None;
                }
            }
            None => {}
        }
    }

    if a >= b {
        None
    } else {
        Some([a, b])
    }
}

impl<A: Space<Dim = na::U1>> Poly<A> {
    /// Cut a region out of a Poly, returning what is left over.
    pub fn subtract_region(&mut self, region: Region<A>) {
        if let Some([x, y]) = region_to_line(region) {
            self.lines = self
                .lines
                .iter()
                .flat_map(|&[a, b]| match (x <= a, b <= y) {
                    (true, true) => vec![],
                    (true, false) => vec![[a.max(y), b]],
                    (false, true) => vec![[a, b.min(x)]],
                    (false, false) => vec![[a, x], [y, b]],
                })
                .collect();
        }
    }

    pub fn get_vertices(self) -> impl Iterator<Item = (Projective<f64, (), A>, f64)> {
        self.lines.into_iter().flat_map(|[a, b]| {
            vec![
                (Linear(na::Vector2::new(a, 1.)), 1.),
                (Linear(na::Vector2::new(b, 1.)), 1.),
            ]
        })
    }

    pub fn new(a: f64, b: f64) -> Self {
        Self {
            _phantom: core::marker::PhantomData,
            lines: vec![[a, b]],
        }
    }
}

#[test]
fn test() {
    pub struct TestSpace;
    impl Space for TestSpace {
        type Dim = na::U1;
    }

    let region: Region<TestSpace> = {
        let a = Linear(na::RowVector2::new(4., -1.));
        let b = Linear(na::RowVector2::new(-2., 1.));

        vec![a, b].into()
    };

    let mut poly: Poly<TestSpace> = Poly::new(0., 1.);

    poly.subtract_region(region);

    assert_eq!(poly.lines, vec![[0., 0.25], [0.5, 1.]]);

    //

    let region: Region<TestSpace> = {
        let a = Linear(na::RowVector2::new(4., -1.));

        vec![a].into()
    };

    let mut poly: Poly<TestSpace> = Poly::new(0., 1.);

    poly.subtract_region(region);

    assert_eq!(poly.lines, vec![[0., 0.25]]);

    //

    let region: Region<TestSpace> = {
        let b = Linear(na::RowVector2::new(-2., 1.));

        vec![b].into()
    };

    let mut poly: Poly<TestSpace> = Poly::new(0., 1.);

    poly.subtract_region(region);

    assert_eq!(poly.lines, vec![[0.5, 1.]]);

    //

    let region: Region<TestSpace> = { <Vec<Projective<f64, TestSpace, ()>>>::new().into() };

    let mut poly: Poly<TestSpace> = Poly::new(0., 1.);

    poly.subtract_region(region);

    assert_eq!(poly.lines, <Vec<[f64; 2]>>::new());

    //

    let region: Region<TestSpace> = {
        let a = Linear(na::RowVector2::new(1., -4.));
        let b = Linear(na::RowVector2::new(-1., 2.));

        vec![a, b].into()
    };

    let mut poly: Poly<TestSpace> = Poly::new(0., 1.);

    poly.subtract_region(region);

    assert_eq!(poly.lines, vec![[0., 1.]]);

    //

    let region: Region<TestSpace> = {
        let a = Linear(na::RowVector2::new(1., 4.));
        let b = Linear(na::RowVector2::new(-1., -2.));

        vec![a, b].into()
    };

    let mut poly: Poly<TestSpace> = Poly::new(0., 1.);

    poly.subtract_region(region);

    assert_eq!(poly.lines, vec![[0., 1.]]);
}
