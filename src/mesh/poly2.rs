use super::region::Region;
use nalgebra as na;
use vector_spaces::*;

mod polygon;

pub struct Poly<A: Space<Dim = na::U2>> {
    _phantom: core::marker::PhantomData<A>,
    poly: polygon::Polygon,
}

fn region_to_polygon<A: Space<Dim = na::U2>>(region: Region<A>) -> polygon::Polygon {
    let mut out = polygon::Polygon(vec![vec![
        [-0.5, -0.5],
        [1.5, -0.5],
        [1.5, 1.5],
        [-0.5, 1.5],
    ]]);

    for hyperplane in region {
        let [a, b, c]: [f64; 3] = hyperplane.0.into();

        let d = 1. / (a * a + b * b).sqrt();

        if d > 1e6 {
            if c <= 0.0 {
                return polygon::Polygon(Vec::new());
            } else {
                continue;
            }
        }

        let a = a * d;
        let b = b * d;
        let c = c * d;

        let mut clip = polygon::Polygon(vec![vec![
            [-a * c + 5. * b, -b * c - 5. * a],
            [-a * c + 5. * b - 5. * a, -b * c - 5. * a - 5. * b],
            [-a * c - 5. * b - 5. * a, -b * c + 5. * a - 5. * b],
            [-a * c - 5. * b, -b * c + 5. * a],
        ]]);
        clip.perturb();
        out = out.difference(&clip);
    }

    out
}

impl<A: Space<Dim = na::U2>> Poly<A> {
    pub fn subtract_region(&mut self, region: Region<A>) {
        let mut clip = region_to_polygon(region);
        clip.perturb();

        let out = self.poly.difference(&clip);

        self.poly = out;
    }

    pub fn get_vertices(self) -> impl Iterator<Item = (Projective<f64, (), A>, f64)> {
        self.poly.0.into_iter().flat_map(|mut polygon| {
            if area(&polygon) < 0. {
                polygon.reverse();
            }
            let v1 = polygon[0];
            polygon[1..]
                .windows(2)
                .flat_map(move |w| {
                    let v2 = w[0];
                    let v3 = w[1];

                    // // WRONG in case of polygons with holes
                    // let sign = area(&[v1, v2, v3]).signum();

                    let sign = 1.;

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
            poly: polygon::Polygon(vec![vec![[x1, y1], [x2, y1], [x2, y2], [x1, y2]]]),
        }
    }
}

// pub struct Poly<A: Space<Dim = na::U2>> {
//     _phantom: core::marker::PhantomData<A>,
//     // 0 < v.x < 1
//     // 0 < v.y < 1
//     poly: Vec<Vec<[f64; 2]>>,
// }

fn area(p: &[[f64; 2]]) -> f64 {
    let mut out = 0.;
    for x in p.windows(2) {
        out += x[0][0] * x[1][1] - x[0][1] * x[1][0];
    }
    let x = [p[p.len() - 1], p[0]];
    out += x[0][0] * x[1][1] - x[0][1] * x[1][0];
    out
}

// impl<A: Space<Dim = na::U2>> Poly<A> {
//     pub fn subtract_region(&mut self, _region: Region<A>) {
//         unimplemented!()
//     }

//     pub fn get_vertices(self) -> impl Iterator<Item = (Projective<f64, (), A>, f64)> {
//         self.poly.into_iter().flat_map(|mut polygon| {
//             if area(&polygon) < 0. {
//                 polygon.reverse();
//             }
//             let v1 = polygon[0];
//             polygon[1..]
//                 .windows(2)
//                 .flat_map(move |w| {
//                     let v2 = w[0];
//                     let v3 = w[1];

//                     let sign = area(&[v1, v2, v3]).signum();

//                     vec![(v1, sign), (v2, sign), (v3, sign)]
//                 })
//                 .map(|([x, y], sign)| (Linear(na::Vector3::new(x, y, 1.)), sign))
//                 .collect::<Vec<_>>()
//                 .into_iter()
//         })
//     }

//     // Inputs should be from zero to one.
//     pub fn square([x1, y1]: [f64; 2], [x2, y2]: [f64; 2]) -> Self {
//         Self {
//             _phantom: core::marker::PhantomData,
//             poly: vec![vec![[x1, y1], [x2, y1], [x2, y2], [x1, y2]]],
//         }
//     }
// }

// #[test]
// fn test_area() {
//     let points = [[1, -1], [1, 1], [-1, 1], [-1, -1]];
//     assert_eq!(area(&points), 4);

//     let points = [[-1, 1], [1, 1], [1, -1], [-1, -1]];
//     assert_eq!(area(&points), -4);
// }
