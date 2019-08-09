use super::region::Region;
use nalgebra as na;
use vector_spaces::*;

pub struct Poly<A: Space<Dim = na::U2>> {
    _phantom: core::marker::PhantomData<A>,
    // 0 < v.x < 1
    // 0 < v.y < 1
    poly: Vec<Vec<[f64; 2]>>,
}

fn area(p: &[[f64; 2]]) -> f64 {
    let mut out = 0.;
    for x in p.windows(2) {
        out += x[0][0] * x[1][1] - x[0][1] * x[1][0];
    }
    let x = [p[p.len() - 1], p[0]];
    out += x[0][0] * x[1][1] - x[0][1] * x[1][0];
    out
}

impl<A: Space<Dim = na::U2>> Poly<A> {
    pub fn subtract_region(&mut self, _region: Region<A>) {
        unimplemented!()
    }

    pub fn get_vertices(self) -> impl Iterator<Item = (Projective<f64, (), A>, f64)> {
        self.poly.into_iter().flat_map(|mut polygon| {
            if area(&polygon) < 0. {
                polygon.reverse();
            }
            let v1 = polygon[0];
            polygon[1..]
                .windows(2)
                .flat_map(move |w| {
                    let v2 = w[0];
                    let v3 = w[1];

                    let sign = area(&[v1, v2, v3]).signum();

                    vec![(v1, sign), (v2, sign), (v3, sign)]
                })
                .map(|([x, y], sign)| (Linear(na::Vector3::new(x, y, 1.)), sign))
                .collect::<Vec<_>>()
                .into_iter()
        })
    }

    // Inputs should be from zero to one.
    pub fn square([x1, y1]: [f64; 2], [x2, y2]: [f64; 2]) -> Self {
        Self {
            _phantom: core::marker::PhantomData,
            poly: vec![vec![[x1, y1], [x2, y1], [x2, y2], [x1, y2]]],
        }
    }
}

#[test]
fn test_area() {
    let points = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
    assert_eq!(area(&points), 4);

    let points = [[-1, 1], [1, 1], [1, -1], [-1, -1]];
    assert_eq!(area(&points), -4);
}
