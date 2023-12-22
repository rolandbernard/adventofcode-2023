mod common;

fn main() {
    let boxes = common::read_input();
    let (supports, supported_by) = common::build_support_graph(boxes);
    let count = supports
        .into_iter()
        .filter(|x| x.iter().all(|&i| supported_by[i].len() > 1))
        .count();
    println!("Result: {count}");
}
