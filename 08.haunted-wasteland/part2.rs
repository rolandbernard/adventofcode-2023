use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_r, old_s, old_t, s, t)
}

fn find_offset(o: i64, p: i64, s: i64, r: i64) -> Option<i64> {
    let (gcd, x, y, zx, zy) = extended_gcd(p, r);
    if (s - o) % gcd != 0 {
        return None;
    }
    let off;
    if o > s {
        off = o + ((s - o) / gcd * x).rem_euclid(zx) * p;
    } else {
        off = s + ((o - s) / gcd * y).rem_euclid(zy) * r;
    }
    return Some(off);
}

fn lcm(a: i64, b: i64) -> i64 {
    a / extended_gcd(a, b).0 * b
}

fn compute_steps(patterns: &Vec<(Vec<i64>, Vec<(i64, i64)>)>) -> i64 {
    let mut options = HashSet::<i64>::new();
    for (single, _) in patterns {
        options.extend(single.iter().copied());
    }
    let mut periods = Vec::new();
    for (single, repeats) in patterns {
        options.retain(|step| {
            single.contains(step) || (repeats.iter().any(|(s, r)| (step - s) % r == 0))
        });
        if periods.is_empty() {
            periods.extend(repeats.iter().copied());
        } else {
            let mut new_periods = Vec::new();
            for &(s, r) in repeats {
                new_periods.extend(
                    periods
                        .iter()
                        .map(|&(o, p)| find_offset(o, p, s, r).map(|x| (x, lcm(p, r))))
                        .filter(Option::is_some)
                        .map(Option::unwrap),
                );
            }
            periods = new_periods;
        }
    }
    return i64::min(
        periods.into_iter().min().unwrap_or((i64::MAX, 0)).0,
        options.into_iter().min().unwrap_or(i64::MAX),
    );
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (instr, map_lines) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::new();
    for line in map_lines.lines() {
        let (node, conn) = line.split_once(" = ").unwrap();
        let (left, right) = conn
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();
        map.insert(node, (left, right));
    }
    let starts = map
        .iter()
        .map(|(k, _)| *k)
        .filter(|e| e.ends_with('A'))
        .collect::<Vec<_>>();
    let mut patterns = vec![vec![(vec![], vec![]); starts.len()]; instr.len()];
    for (j, mut pos) in starts.into_iter().enumerate() {
        let mut step = 0i64;
        let mut seen = HashMap::new();
        let mut dup = HashSet::new();
        for (i, instr) in instr.chars().enumerate().cycle() {
            if instr == 'L' {
                pos = map[pos].0;
            } else {
                pos = map[pos].1;
            }
            step += 1;
            if dup.contains(&(i, pos)) {
                break;
            }
            if let Some(stp) = seen.get(&(i, pos)) {
                if pos.ends_with('Z') {
                    patterns[i][j].1.push((*stp, step - stp));
                }
                dup.insert((i, pos));
            }
            if pos.ends_with('Z') {
                patterns[i][j].0.push(step);
            }
            seen.insert((i, pos), step);
        }
    }
    let min_steps = (0..instr.len())
        .filter(|&i| {
            patterns[i]
                .iter()
                .all(|(single, reps)| !single.is_empty() || !reps.is_empty())
        })
        .map(|i| compute_steps(&patterns[i]))
        .min()
        .unwrap();
    if min_steps == i64::MAX {
        println!("Result: NOT FOUND");
    } else {
        println!("Result: {}", min_steps);
    }
}
