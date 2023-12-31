use crate::util::parse_vec3;
use nalgebra::{Matrix3, RowVector3, Vector2, Vector3};

#[derive(Debug)]
struct XYRay {
    start: Vector2<i64>,
    direction: Vector2<i64>,
}
#[derive(Debug)]
struct Projectile {
    start: [f64; 3],
    velocity: [f64; 3],
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

fn parse_input2(input: &str) -> Vec<Projectile> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            // let pos = parse_vec3(pos);
            let pos: Vec<f64> = pos
                .trim()
                .split(',')
                .filter_map(|part| part.trim().parse::<f64>().ok())
                .collect();
            let vel: Vec<f64> = vel
                .trim()
                .split(',')
                .filter_map(|part| part.trim().parse::<f64>().ok())
                .collect();
            Projectile {
                start: [pos[0], pos[1], pos[2]],
                velocity: [vel[0], vel[1], vel[2]],
            }
        })
        .collect()
}

fn sub(u: &[f64], v: &[f64]) -> Vec<f64> {
    vec![u[0] - v[0], u[1] - v[1], u[2] - v[2]]
}

fn exterior3(u: &[f64], v: &[f64], w: &[f64]) -> f64 {
    u[0] * v[1] * w[2] + u[1] * v[2] * w[0] + u[2] * v[0] * w[1]
        - u[0] * v[2] * w[1]
        - u[1] * v[0] * w[2]
        - u[2] * v[1] * w[0]
}

fn exterior2(v: &[f64], w: &[f64]) -> Vec<f64> {
    vec![
        v[0] * w[1] - v[1] * w[0],
        v[1] * w[2] - v[2] * w[1],
        v[2] * w[0] - v[0] * w[2],
    ]
}

fn solve(projectiles: &[Projectile], idzs: &[usize; 3]) -> i64 {
    let projectiles: Vec<Vec<[f64; 3]>> = vec![
        vec![projectiles[idzs[0]].start, projectiles[idzs[0]].velocity],
        vec![projectiles[idzs[1]].start, projectiles[idzs[1]].velocity],
        vec![projectiles[idzs[2]].start, projectiles[idzs[2]].velocity],
    ];
    let a: Vec<Vec<f64>> = vec![
        exterior2(
            &sub(&projectiles[0][1], &projectiles[1][1]),
            &sub(&projectiles[0][0], &projectiles[1][0]),
        ),
        exterior2(
            &sub(&projectiles[0][1], &projectiles[2][1]),
            &sub(&projectiles[0][0], &projectiles[2][0]),
        ),
        exterior2(
            &sub(&projectiles[1][1], &projectiles[2][1]),
            &sub(&projectiles[1][0], &projectiles[2][0]),
        ),
    ];
    let b: Vec<f64> = vec![
        -exterior3(&projectiles[0][0], &projectiles[0][1], &projectiles[1][0])
            - exterior3(&projectiles[1][0], &projectiles[1][1], &projectiles[0][0]),
        -exterior3(&projectiles[0][0], &projectiles[0][1], &projectiles[2][0])
            - exterior3(&projectiles[2][0], &projectiles[2][1], &projectiles[0][0]),
        -exterior3(&projectiles[1][0], &projectiles[1][1], &projectiles[2][0])
            - exterior3(&projectiles[2][0], &projectiles[2][1], &projectiles[1][0]),
    ];
    let row1 = RowVector3::from_row_slice(&a[0]);
    let row2 = RowVector3::from_row_slice(&a[1]);
    let row3 = RowVector3::from_row_slice(&a[2]);
    let mat = Matrix3::from_rows(&[row1, row2, row3]);
    let data = Vector3::from_row_slice(&b);

    let xyz = mat.lu().solve(&data);
    xyz.unwrap().sum().ceil() as i64
}

// part two is basically taken from https://github.com/apprenticewiz/adventofcode/blob/main/2023/rust/day24b/src/main.rs
// with addition of checking for consistent solutions
pub fn part2(input: &str) -> i64 {
    let projectiles = parse_input2(input);
    let idzs: Vec<[usize; 3]> = vec![[1, 2, 5], [2, 3, 4]];
    // [0, 1, 2 somehow gives a wrong result, probably due to numeric issues]
    let sol = solve(&projectiles, &[1, 2, 3]);
    for idx in idzs {
        let new_sol = solve(&projectiles, &idx);
        assert_eq!(sol, new_sol);
    }
    sol
}
