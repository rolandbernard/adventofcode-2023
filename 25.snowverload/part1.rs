use std::collections::{BinaryHeap, HashMap, HashSet};

fn min_cut(network: &[Vec<usize>]) -> (usize, usize) {
    let mut weighted = vec![HashMap::new(); network.len()];
    for i in 0..network.len() {
        for &next in &network[i] {
            weighted[i].insert(next, 1);
        }
    }
    let mut best = (usize::MAX, 0);
    let mut merge_count = vec![1; weighted.len()];
    let mut remaining = (0..weighted.len()).collect::<HashSet<_>>();
    for _ in 1..weighted.len() {
        let mut s = *remaining.iter().next().unwrap();
        let mut t = s;
        let mut w = weighted[t].clone();
        let mut merged = HashSet::from([t]);
        let mut w_sorted = BinaryHeap::new();
        for (&next, &c) in &w {
            w_sorted.push((c, next));
        }
        for _ in 1..remaining.len() {
            w.remove(&t);
            s = t;
            t = w_sorted.pop().unwrap().1;
            while merged.contains(&t) {
                t = w_sorted.pop().unwrap().1;
            }
            merged.insert(t);
            for (&n, &c) in &weighted[t] {
                if !merged.contains(&n) {
                    *w.entry(n).or_insert(0) += c;
                    w_sorted.push((w[&n], n));
                }
            }
        }
        best = best.min((w[&t], merge_count[t]));
        merge_count[s] += merge_count[t];
        remaining.remove(&t);
        for &i in &remaining {
            if let Some(&w) = weighted[i].get(&t) {
                *weighted[i].entry(s).or_insert(0) += w;
                weighted[i].remove(&t);
            }
        }
        for &i in &remaining {
            if let Some(&w) = weighted[t].get(&i) {
                *weighted[s].entry(i).or_insert(0) += w;
            }
        }
        weighted[s].remove(&s);
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
    let mut network = vec![Vec::new(); names.len()];
    for &(from, to) in &links {
        network[from].push(to);
        network[to].push(from);
    }
    let (num, merged) = min_cut(&network);
    assert_eq!(num, 3);
    println!("Result: {}", merged * (network.len() - merged));
}
