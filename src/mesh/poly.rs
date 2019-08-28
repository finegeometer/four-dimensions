use super::region::Region;
use nalgebra as na;
use std::convert::TryInto;
use vector_spaces::*;

pub struct Poly<A: Space<Dim = na::U2>> {
    _phantom: core::marker::PhantomData<A>,
    poly: polygon3::Polygon,
}

fn square() -> Vec<polygon3::Line> {
    vec![
        [1, 0, 0].try_into().unwrap(),
        [0, 1, 0].try_into().unwrap(),
        [-1, 0, 1].try_into().unwrap(),
        [0, -1, 1].try_into().unwrap(),
    ]
}

fn region_to_polygon<A: Space<Dim = na::U2>>(region: Region<A>) -> Option<polygon3::Polygon> {
    let boundaries = region
        .into_iter()
        .filter_map(|h| polygon3::Line::try_from_f64_array(h.0.into()))
        .chain(square());

    let convex_polygon = polygon3::ConvexPolygon::from_boundaries(boundaries)?;

    Some(convex_polygon.try_into().unwrap())
}

impl<A: Space<Dim = na::U2>> Poly<A> {
    pub fn subtract_regions(&mut self, region: impl IntoIterator<Item = Region<A>>) {
        self.poly = self.poly.difference(
            &region
                .into_iter()
                .filter_map(region_to_polygon)
                .collect::<Vec<_>>(),
        );
    }

    pub fn get_vertices(self) -> impl Iterator<Item = (Projective<f64, (), A>, f64)> {
        self.poly.vertices().into_iter().flat_map(|mut polygon| {
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
                    let sign = area(&[v1, v2, v3]).signum();

                    vec![(v1, sign), (v2, sign), (v3, sign)]
                })
                .map(|(v, sign)| (Linear(v.to_f64_array().into()), sign))
                .collect::<Vec<_>>()
                .into_iter()
        })
    }

    // Inputs should be from zero to one.
    pub fn square([x1, y1]: [f64; 2], [x2, y2]: [f64; 2]) -> Self {
        Self {
            _phantom: core::marker::PhantomData,
            poly: polygon3::Polygon::try_from_edges(vec![vec![
                polygon3::Line::try_from_f64_array([1., 0., -x1]).unwrap(),
                polygon3::Line::try_from_f64_array([0., 1., -y1]).unwrap(),
                polygon3::Line::try_from_f64_array([-1., 0., x2]).unwrap(),
                polygon3::Line::try_from_f64_array([0., -1., y2]).unwrap(),
            ]])
            .unwrap(),
        }
    }
}

fn area(p: &[polygon3::Point]) -> f64 {
    let n = p.len();
    let mut out = 0.;
    for i in 0..n {
        let j = (i + 1) % n;

        let [mut x1, mut y1, z1] = p[i].to_f64_array();
        x1 /= z1;
        y1 /= z1;
        let [mut x2, mut y2, z2] = p[j].to_f64_array();
        x2 /= z2;
        y2 /= z2;

        out += x1 * y2 - y1 * x2;
    }
    out
}
