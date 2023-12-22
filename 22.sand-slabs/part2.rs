use std::collections::VecDeque;

mod common;

fn simulate_fall(
    supports: &Vec<Vec<usize>>,
    supported_by: &Vec<Vec<usize>>,
    destroyed: usize,
) -> usize {
    let mut will_fall = vec![false; supports.len()];
    will_fall[destroyed] = true;
    let mut queue = VecDeque::new();
    queue.push_back(destroyed);
    while let Some(i) = queue.pop_front() {
        for &j in &supports[i] {
            if !will_fall[j] && supported_by[j].iter().all(|&k| will_fall[k]) {
                will_fall[j] = true;
                queue.push_back(j);
            }
        }
    }
    return will_fall.into_iter().filter(|&x| x).count() - 1;
}

fn main() {
    let boxes = common::read_input();
    let (supports, supported_by) = common::build_support_graph(boxes);
    let mut count = 0;
    for i in 0..supports.len() {
        count += simulate_fall(&supports, &supported_by, i);
    }
    println!("Result: {count}");
}
