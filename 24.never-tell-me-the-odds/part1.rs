mod common;

const MIN: f64 = 200_000_000_000_000.0;
const MAX: f64 = 400_000_000_000_000.0;

fn find_intersection(
    vel1: [f64; 3],
    vel2: [f64; 3],
    pos1: [f64; 3],
    pos2: [f64; 3],
) -> Option<[f64; 2]> {
    let mut m = [vec![vel1[0], -vel2[0]], vec![vel1[1], -vel2[1]]];
    let mut b = [pos2[0] - pos1[0], pos2[1] - pos1[1]];
    let mut t = [0.0, 0.0];
    if common::solve_linear(&mut m, &mut b, &mut t) {
        if t[0] >= 0.0 && t[1] >= 0.0 {
            return Some([pos1[0] + vel1[0] * t[0], pos1[1] + vel1[1] * t[0]]);
        }
    }
    return None;
}

fn main() {
    let paths = common::read_input();
    let mut count = 0;
    for i in 0..paths.len() {
        for j in i + 1..paths.len() {
            let (vel1, pos1) = paths[i];
            let (vel2, pos2) = paths[j];
            if let Some([ix, iy]) = find_intersection(vel1, vel2, pos1, pos2) {
                if MIN <= ix && ix <= MAX && MIN <= iy && iy <= MAX {
                    count += 1;
                }
            }
        }
    }
    println!("Result: {count}");
}
