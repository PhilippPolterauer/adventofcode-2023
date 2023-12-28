use crate::util::parse_vec3;
use nalgebra::{Matrix2, Point2, Vector2, Vector3};

#[derive(Debug)]
struct XYRay {
    start: Vector2<i64>,
    direction: Vector2<i64>,
}

fn line(xy: Vector2<i64>, v: Vector2<i64>) -> (f64, f64) {
    let m = v.y as f64 / v.x as f64;
    let q = -m * xy.x as f64 + xy.y as f64;
    (m, q)
}

impl XYRay {
    fn from_pos_vel(pos: &Vector3<i64>, vel: &Vector3<i64>) -> Self {
        Self {
            start: pos.xy().cast(),
            direction: vel.xy().cast(),
        }
    }
    fn intersect(&self, other: &Self) -> Option<Vector2<f64>> {
        let v1 = self.direction;
        let v2 = other.direction;
        let xy1 = self.start;
        let xy2 = other.start;
        let vx1 = v1.x;
        let vx2 = v2.x;
        let vy1 = v1.y;
        let vy2 = v2.y;

        let det = vx1 * vy2 - vy1 * vx2;
        if det == 0 {
            return None;
        }

        let (m1, q1) = line(xy1, v1);
        let (m2, q2) = line(xy2, v2);

        let xc = (q2 - q1) / (m1 - m2);
        let yc = m1 * xc + q1;

        let tc1 = (xc - xy1.x as f64) / vx1 as f64;
        let tc2 = (xc - xy2.x as f64) / vx2 as f64;
        if tc1 < 0. || tc2 < 0. {
            return None;
        }
        Some(Vector2::<f64>::from_column_slice(&[xc, yc]))
    }
}
fn parse_input(input: &str) -> Vec<(Vector3<i64>, Vector3<i64>)> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            let pos = parse_vec3(pos);
            let vel = parse_vec3(vel);
            (pos, vel)
        })
        .collect()
}
const MIN: f64 = 200000000000000f64;
const MAX: f64 = 400000000000000f64;
pub fn part1(input: &str) -> i64 {
    let data = parse_input(input);
    let rays: Vec<XYRay> = data
        .iter()
        .map(|(pos, vel)| XYRay::from_pos_vel(pos, vel))
        .collect();
    let n = rays.len();
    let mut solution = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if i != j {
                if let Some(inter) = rays[i].intersect(&rays[j]) {
                    if inter.x >= MIN && inter.y >= MIN && inter.x <= MAX && inter.y <= MAX {
                        solution += 1;
                    }
                }
            }
        }
    }
    solution
}
pub fn part2(_input: &str) -> i64 {
    0
}
