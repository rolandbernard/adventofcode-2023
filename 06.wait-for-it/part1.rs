fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let times = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|t| t.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|t| t.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let mut cur = 1;
    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        let det = f64::sqrt(time * time - 4.0 * (distance + 1.0));
        let x1 = (time - det) / 2.0;
        let x2 = (time + det) / 2.0;
        cur *= 1 + (x2.floor().max(0.0) - x1.ceil().max(0.0)).max(0.0) as i64;
    }
    println!("Result: {}", cur);
}
