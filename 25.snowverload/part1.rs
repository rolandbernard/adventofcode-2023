use std::collections::{HashMap, HashSet};

fn min_cut(network: &Vec<HashSet<usize>>) -> (usize, Vec<usize>) {
    let mut mat = vec![vec![0; network.len()]; network.len()];
    for i in 0..network.len() {
        for &j in &network[i] {
            mat[i][j] = 1;
        }
    }
    let mut best = (usize::MAX, Vec::new());
    let mut merged = (0..mat.len()).map(|e| vec![e]).collect::<Vec<_>>();
    let mut remaining = (0..mat.len()).collect::<HashSet<_>>();
    for _ in 1..mat.len() {
        let mut unmerged = remaining.clone();
        unmerged.remove(&0);
        let mut w = mat[0].clone();
        let mut s = 0;
        let mut t = 0;
        for _ in 0..unmerged.len() {
            s = t;
            t = unmerged.iter().cloned().max_by_key(|&i| w[i]).unwrap();
            unmerged.remove(&t);
            for i in 0..mat.len() {
                w[i] += mat[t][i];
            }
        }
        let cur = (w[t] - mat[t][t], merged[t].clone());
        best = best.min(cur);
        let cot_copy = merged[t].clone();
        merged[s].extend(cot_copy);
        for i in 0..mat.len() {
            mat[s][i] += mat[t][i];
        }
        for i in 0..mat.len() {
            mat[i][s] = mat[s][i];
        }
        remaining.remove(&t);
    }
    return best;
}

fn main() {
    let mut names = HashMap::new();
    let mut links = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (name, conn) = line.split_once(": ").unwrap();
        let len = names.len();
        let id = *names.entry(name.to_owned()).or_insert(len);
        for next in conn.split_whitespace() {
            let len = names.len();
            let next_id = *names.entry(next.to_owned()).or_insert(len);
            links.push((id, next_id));
        }
    }
    let mut network = vec![HashSet::new(); names.len()];
    for &(from, to) in &links {
        network[from].insert(to);
        network[to].insert(from);
    }
    let (num, merged) = min_cut(&network);
    assert_eq!(num, 3);
    println!("Result: {}", merged.len() * (network.len() - merged.len()));
}
