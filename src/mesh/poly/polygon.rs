#![deny(unsafe_code)]
#![allow(dead_code)]



use core::ops::Bound::*;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Clone)]
pub struct Polygon(pub Vec<Vec<[f64; 2]>>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PolygonEdge {
    poly: usize,
    edge: usize,
}

impl PolygonEdge {
    fn new(poly: usize, edge: usize) -> Self {
        Self { poly, edge }
    }
}

impl Polygon {
    fn edges<'r>(&'r self) -> impl Iterator<Item = (PolygonEdge, [[f64; 2]; 2])> + 'r {
        self.0.iter().enumerate().flat_map(|(i, poly)| {
            (0..poly.len()).map(move |j| {
                (
                    PolygonEdge::new(i, j),
                    [poly[j], poly[(j + 1) % poly.len()]],
                )
            })
        })
    }

    pub fn contains(&self, p: [f64; 2]) -> bool {
        let mut out = false;
        for e in self.edges() {
            if intersect(e.1, [p, [1e6, 1.618_033_988e6]]).is_some() {
                out ^= true;
            }
        }
        out
    }

    pub fn perturb(&mut self) {
        for p in self.0.iter_mut() {
            for [x, y] in p.iter_mut() {
                *x += 1e-5 * js_sys::Math::random();
                *y += 1e-5 * js_sys::Math::random();
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Alpha(u64);

struct TryToAlphaError;

impl TryFrom<f64> for Alpha {
    type Error = TryToAlphaError;
    // Should be monotonic
    fn try_from(x: f64) -> Result<Self, TryToAlphaError> {
        let out = x.to_bits();

        let min = (0f64).to_bits();
        let max = (1f64).to_bits();

        if (min..=max).contains(&out) {
            Ok(Self(out))
        } else {
            Err(TryToAlphaError)
        }
    }
}

fn intersect(e1: [[f64; 2]; 2], e2: [[f64; 2]; 2]) -> Option<([f64; 2], [Alpha; 2])> {
    let alpha_2 = {
        let measure = |[x, y]: [f64; 2]| x * (e1[1][1] - e1[0][1]) + y * (e1[0][0] - e1[1][0]);
        let a = measure(e2[0]);
        let b = measure(e2[1]);
        let c = measure(e1[0]);
        (a - c) / (a - b)
    };

    let alpha_1 = {
        let measure = |[x, y]: [f64; 2]| x * (e2[1][1] - e2[0][1]) + y * (e2[0][0] - e2[1][0]);
        let a = measure(e1[0]);
        let b = measure(e1[1]);
        let c = measure(e2[0]);
        (a - c) / (a - b)
    };

    let coords = [
        e1[0][0] + alpha_1 * (e1[1][0] - e1[0][0]),
        e1[0][1] + alpha_1 * (e1[1][1] - e1[0][1]),
    ];

    Some((coords, [alpha_1.try_into().ok()?, alpha_2.try_into().ok()?]))
}

#[derive(Debug)]
struct PolygonIntersectionInfo {
    /// Which edge of the other polygon is hit.
    other_edge: PolygonEdge,
    /// How far along the other edge is it hit?
    other_alpha: Alpha,
    /// Am I entering or exiting the other polygon?
    entering_other: bool,
    /// Coordinates of intersection
    coords: [f64; 2],
}

#[derive(Debug)]
struct PolygonEdgeInfo {
    index: PolygonEdge,
    coords: [[f64; 2]; 2],
    intersections: BTreeMap<Alpha, PolygonIntersectionInfo>,
}

impl Polygon {
    pub fn difference(&self, other: &Self) -> Self {
        // Set up edge lists

        let mut poly1: Vec<Vec<PolygonEdgeInfo>> = Vec::new();

        for (i, poly) in self.0.iter().enumerate() {
            let mut out = Vec::new();

            let n = poly.len();

            for j in 0..n {
                out.push(PolygonEdgeInfo {
                    index: PolygonEdge::new(i, j),
                    coords: [poly[j], poly[(j + 1) % n]],
                    intersections: BTreeMap::new(),
                })
            }

            poly1.push(out);
        }

        let mut poly2: Vec<Vec<PolygonEdgeInfo>> = Vec::new();

        for (i, poly) in other.0.iter().enumerate() {
            let mut out = Vec::new();

            let n = poly.len();

            for j in 0..n {
                out.push(PolygonEdgeInfo {
                    index: PolygonEdge::new(i, j),
                    coords: [poly[j], poly[(j + 1) % n]],
                    intersections: BTreeMap::new(),
                })
            }

            poly2.push(out);
        }

        // Mark intersections

        for p1 in &mut poly1 {
            for e1 in p1 {
                for p2 in &mut poly2 {
                    for e2 in p2 {
                        if let Some((coords, [alpha_1, alpha_2])) = intersect(e1.coords, e2.coords)
                        {
                            e1.intersections.insert(
                                alpha_1,
                                PolygonIntersectionInfo {
                                    other_edge: e2.index,
                                    other_alpha: alpha_2,
                                    entering_other: false,
                                    coords,
                                },
                            );
                            e2.intersections.insert(
                                alpha_2,
                                PolygonIntersectionInfo {
                                    other_edge: e1.index,
                                    other_alpha: alpha_1,
                                    entering_other: false,
                                    coords,
                                },
                            );
                        }
                    }
                }
            }
        }

        // Mark entry/exit

        for p in &mut poly1 {
            let mut side = other.contains(p[0].coords[0]);
            for e in p {
                for i in e.intersections.values_mut() {
                    side ^= true;
                    i.entering_other = side;
                }
            }
        }

        for p in &mut poly2 {
            let mut side = self.contains(p[0].coords[0]);
            for e in p {
                for i in e.intersections.values_mut() {
                    side ^= true;
                    i.entering_other = side;
                }
            }
        }

        // Construct output
        let mut out: Vec<Vec<[f64; 2]>> = Vec::new();

        let mut to_visit: BTreeSet<(PolygonEdge, Alpha)> = poly1
            .iter()
            .flat_map(|p| {
                p.iter()
                    .flat_map(|e| e.intersections.keys().map(move |k| (e.index, *k)))
            })
            .collect();

        // TODO: Prove this halts
        while let Some(&info) = to_visit.iter().next() {
            let mut out_poly = Vec::new();

            let (mut edge, mut alpha) = info;

            let mut intersection: &PolygonIntersectionInfo = poly1[edge.poly][edge.edge]
                .intersections
                .get(&alpha)
                .unwrap();

            loop {
                // Delete from to_visit
                to_visit.remove(&(edge, alpha));

                // Hop to polygon2
                edge = intersection.other_edge;
                alpha = intersection.other_alpha;
                intersection = poly2[edge.poly][edge.edge]
                    .intersections
                    .get(&alpha)
                    .unwrap();

                // Walk to next intersection
                if intersection.entering_other {
                    // go forwards

                    if let Some(info) = poly2[edge.poly][edge.edge]
                        .intersections
                        .range((Excluded(alpha), Unbounded))
                        .next()
                    {
                        alpha = *info.0;
                        intersection = info.1;
                    } else {
                        loop {
                            // Push vertex
                            out_poly.push(poly2[edge.poly][edge.edge].coords[1]);

                            edge.edge = (edge.edge + 1) % poly2[edge.poly].len();
                            if let Some(info) =
                                poly2[edge.poly][edge.edge].intersections.range(..).next()
                            {
                                alpha = *info.0;
                                intersection = info.1;
                                break;
                            }
                        }
                    }
                } else {
                    // go backwards

                    if let Some(info) = poly2[edge.poly][edge.edge]
                        .intersections
                        .range((Unbounded, Excluded(alpha)))
                        .rev()
                        .next()
                    {
                        alpha = *info.0;
                        intersection = info.1;
                    } else {
                        loop {
                            // Push vertex
                            out_poly.push(poly2[edge.poly][edge.edge].coords[0]);

                            edge.edge = edge
                                .edge
                                .checked_sub(1)
                                .unwrap_or(poly2[edge.poly].len() - 1);
                            if let Some(info) = poly2[edge.poly][edge.edge]
                                .intersections
                                .range(..)
                                .rev()
                                .next()
                            {
                                alpha = *info.0;
                                intersection = info.1;
                                break;
                            }
                        }
                    }
                }

                // Hop to polygon1
                edge = intersection.other_edge;
                alpha = intersection.other_alpha;
                intersection = poly1[edge.poly][edge.edge]
                    .intersections
                    .get(&alpha)
                    .unwrap();

                // Delete from to_visit
                to_visit.remove(&(edge, alpha));

                // Push vertex
                out_poly.push(intersection.coords);

                // Walk to next intersection
                if !intersection.entering_other {
                    // go forwards

                    if let Some(info) = poly1[edge.poly][edge.edge]
                        .intersections
                        .range((Excluded(alpha), Unbounded))
                        .next()
                    {
                        alpha = *info.0;
                        intersection = info.1;
                    } else {
                        loop {
                            // Push vertex
                            out_poly.push(poly1[edge.poly][edge.edge].coords[1]);

                            edge.edge = (edge.edge + 1) % poly1[edge.poly].len();
                            if let Some(info) =
                                poly1[edge.poly][edge.edge].intersections.range(..).next()
                            {
                                alpha = *info.0;
                                intersection = info.1;
                                break;
                            }
                        }
                    }
                } else {
                    // go backwards

                    if let Some(info) = poly1[edge.poly][edge.edge]
                        .intersections
                        .range((Unbounded, Excluded(alpha)))
                        .rev()
                        .next()
                    {
                        alpha = *info.0;
                        intersection = info.1;
                    } else {
                        loop {
                            // Push vertex
                            out_poly.push(poly1[edge.poly][edge.edge].coords[0]);

                            edge.edge = edge
                                .edge
                                .checked_sub(1)
                                .unwrap_or(poly1[edge.poly].len() - 1);
                            if let Some(info) = poly1[edge.poly][edge.edge]
                                .intersections
                                .range(..)
                                .rev()
                                .next()
                            {
                                alpha = *info.0;
                                intersection = info.1;
                                break;
                            }
                        }
                    }
                }

                // Push vertex
                out_poly.push(intersection.coords);

                if info == (edge, alpha) {
                    break;
                }
            }

            out.push(out_poly);
        }

        if out.is_empty() {
            Self(
                self.0
                    .iter()
                    .cloned()
                    .filter(|p| !other.contains(p[0]))
                    .chain(other.0.iter().cloned().filter(|p| self.contains(p[0])))
                    .collect(),
            )
        } else {
            Self(out)
        }
    }
}

#[test]
fn test_intersection() {
    let l = [[1., 1.], [5., 3.]];
    let m = [[2., 0.], [4., 4.]];
    assert_eq!(intersect(l, m).unwrap().0, [3., 2.]);
}

#[test]
fn test() {
    let square = Polygon(vec![vec![[-1., -1.], [-1., 1.], [1., 1.], [1., -1.]]]);
    let bowtie = Polygon(vec![vec![[-2., -3.], [2., 3.], [2., -3.], [-2., 3.]]]);
    let big_square = Polygon(vec![vec![[-4., -4.], [-4., 4.], [4., 4.], [4., -4.]]]);

    assert_eq!(
        square.difference(&bowtie),
        Polygon(vec![vec![
            [0.6666666666666667, -1.0],
            [-0.6666666666666667, -1.0],
            [0.6666666666666667, 1.0],
            [-0.6666666666666667, 1.0]
        ]])
    );

    assert_eq!(
        bowtie.difference(&square),
        Polygon(vec![
            vec![
                [-1.0, -1.0],
                [-1.0, 1.0],
                [-0.6666666666666665, 1.0],
                [-2.0, 3.0],
                [-2.0, -3.0],
                [-0.6666666666666667, -1.0]
            ],
            vec![
                [1.0, 1.0],
                [1.0, -1.0],
                [0.6666666666666667, -1.0],
                [2.0, -3.0],
                [2.0, 3.0],
                [0.6666666666666665, 1.0]
            ]
        ])
    );

    assert_eq!(square.difference(&big_square), Polygon(vec![]));

    assert_eq!(
        big_square.difference(&square),
        Polygon(vec![
            vec![[-4.0, -4.0], [-4.0, 4.0], [4.0, 4.0], [4.0, -4.0]],
            vec![[-1.0, -1.0], [-1.0, 1.0], [1.0, 1.0], [1.0, -1.0]]
        ])
    );
}
