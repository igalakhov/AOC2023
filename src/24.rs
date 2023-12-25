use aoc2023::{run_problem, Problem};
use itertools::Itertools;
use std::fmt::Display;

struct Problem24 {
    stones: Vec<((f64, f64, f64), (f64, f64, f64))>,
}

fn intersect_2d(
    ((x1, y1, _), (vx1, vy1, _)): ((f64, f64, f64), (f64, f64, f64)),
    ((x2, y2, _), (vx2, vy2, _)): ((f64, f64, f64), (f64, f64, f64)),
) -> Option<(f64, f64)> {
    let (m1, b1) = (vy1 / vx1, -x1 * (vy1 / vx1) + y1);
    let (m2, b2) = (vy2 / vx2, -x2 * (vy2 / vx2) + y2);

    if m1 == m2 {
        return None;
    }

    let x = (b2 - b1) / (m1 - m2);
    let y = m1 * x + b1;

    let t1 = (x - x1) / vx1;
    let t2 = (x - x2) / vx2;

    return if t1 >= 0.0 && t2 >= 0.0 {
        Some((x, y))
    } else {
        None
    };
}

fn determinant(matrix: Vec<Vec<f64>>) -> f64 {
    if matrix.len() == 1 {
        return matrix[0][0];
    }

    let mut ret = 0.0;

    for j in 0..matrix.len() {
        let subdet = matrix[0][j].clone()
            * determinant(
                matrix[1..]
                    .into_iter()
                    .map(|v| {
                        v.clone()
                            .iter()
                            .enumerate()
                            .filter(|(idx, _)| idx != &j)
                            .map(|t| t.1.clone())
                            .collect_vec()
                    })
                    .collect_vec(),
            );

        if j % 2 == 0 {
            ret += subdet;
        } else {
            ret -= subdet;
        }
    }
    ret
}

fn make_problem(
    ((x1, y1, z1), (vx1, vy1, vz1)): ((f64, f64, f64), (f64, f64, f64)),
    ((x2, y2, z2), (vx2, vy2, vz2)): ((f64, f64, f64), (f64, f64, f64)),
    ((x3, y3, z3), (vx3, vy3, vz3)): ((f64, f64, f64), (f64, f64, f64)),
) -> (Vec<Vec<f64>>, Vec<f64>) {
    (
        vec![
            vec![0.0, vz1 - vz2, -vy1 + vy2, 0.0, -z1 + z2, y1 - y2],
            vec![-vz1 + vz2, 0.0, vx1 - vx2, z1 - z2, 0.0, -x1 + x2],
            vec![vy1 - vy2, -vx1 + vx2, 0.0, -y1 + y2, x1 - x2, 0.0],
            vec![0.0, vz2 - vz3, -vy2 + vy3, 0.0, -z2 + z3, y2 - y3],
            vec![-vz2 + vz3, 0.0, vx2 - vx3, z2 - z3, 0.0, -x2 + x3],
            vec![vy2 - vy3, -vx2 + vx3, 0.0, -y2 + y3, x2 - x3, 0.0],
        ],
        vec![
            -vy1 * z1 + vy2 * z2 + vz1 * y1 - vz2 * y2,
            vx1 * z1 - vx2 * z2 - vz1 * x1 + vz2 * x2,
            -vx1 * y1 + vx2 * y2 + vy1 * x1 - vy2 * x2,
            -vy2 * z2 + vy3 * z3 + vz2 * y2 - vz3 * y3,
            vx2 * z2 - vx3 * z3 - vz2 * x2 + vz3 * x3,
            -vx2 * y2 + vx3 * y3 + vy2 * x2 - vy3 * x3,
        ],
    )
}

fn solve_cramer(m: Vec<Vec<f64>>, b: Vec<f64>) -> Vec<f64> {
    let d = determinant(m.clone());
    (0..m.len())
        .map(|j| {
            let mut col = m.clone();
            (0..m.len()).for_each(|i| {
                col[i][j] = b[i];
            });
            determinant(col) / d
        })
        .collect_vec()
}

impl Problem for Problem24 {
    fn solve<F1, F2>(&self, report_first: F1, report_second: F2) -> ()
    where
        F1: FnOnce(&dyn Display) -> (),
        F2: FnOnce(&dyn Display) -> (),
    {
        report_first(
            &(0..self.stones.len())
                .flat_map(|i| (i + 1..self.stones.len()).map(move |j| (i, j)))
                .filter(|(i, j)| {
                    const MI: f64 = 200000000000000.0;
                    const MA: f64 = 400000000000000.0;
                    intersect_2d(self.stones[*i].into(), self.stones[*j].into())
                        .map_or_else(|| false, |(x, y)| x >= MI && y >= MI && x <= MA && y <= MA)
                })
                .count(),
        );

        let (m, b) = make_problem(self.stones[0], self.stones[1], self.stones[2]);

        let sol = solve_cramer(m, b);

        report_second(&sol.iter().take(3).sum::<f64>());
    }

    fn parse(lines: Vec<String>) -> Self {
        Self {
            stones: lines
                .into_iter()
                .map(|line| {
                    let (pos, vel) = line.split(" @ ").collect_tuple().unwrap();
                    let parse = |p: &str| {
                        p.replace(" ", "")
                            .split(",")
                            .map(|x| x.parse::<f64>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    };
                    (parse(pos), parse(vel))
                })
                .collect(),
        }
    }
}

fn main() {
    run_problem::<Problem24>("inputs/24.txt");
}
