pub fn read_input() -> Vec<([f64; 3], [f64; 3])> {
    let mut paths = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (pos, vel) = line.split_once(" @ ").unwrap();
        let vel = vel
            .split(",")
            .map(|x| x.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        let pos = pos
            .split(",")
            .map(|x| x.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        paths.push(([vel[0], vel[1], vel[2]], [pos[0], pos[1], pos[2]]));
    }
    return paths;
}

pub fn solve_linear(m: &mut [Vec<f64>], b: &mut [f64], x: &mut [f64]) -> bool {
    let mut ok = true;
    let mut perm: Vec<usize> = (0..m.len()).collect();
    for i in 0..m.len() {
        let p = (i..m.len())
            .max_by(|&x, &y| {
                m[perm[x]][i]
                    .abs()
                    .partial_cmp(&m[perm[y]][i].abs())
                    .unwrap()
            })
            .unwrap_or(i);
        perm.swap(i, p);
        let pivot = m[perm[i]][i];
        if pivot != 0.0 {
            for j in i + 1..m.len() {
                let mul = m[perm[j]][i] / pivot;
                for k in i + 1..m.len() {
                    m[perm[j]][k] -= mul * m[perm[i]][k];
                }
                b[perm[j]] -= mul * b[perm[i]];
            }
        } else {
            ok = false;
        }
    }
    for i in (0..x.len()).rev() {
        if m[perm[i]][i] != 0.0 {
            x[i] = b[perm[i]];
            for k in i + 1..x.len() {
                x[i] -= m[perm[i]][k] * x[k];
            }
            x[i] /= m[perm[i]][i];
        } else {
            x[i] = 0.0;
        }
    }
    return ok;
}
