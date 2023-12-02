const COLORS: [&str; 3] = ["red", "green", "blue"];

fn main() {
    let mut cur = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (_, content) = line.split_once(": ").unwrap();
        let mut max = [0, 0, 0];
        for subsets in content.split("; ") {
            for part in subsets.split(", ") {
                let (count, color) = part.split_once(" ").unwrap();
                let idx = COLORS.iter().position(|&x| x == color).unwrap();
                max[idx] = max[idx].max(count.parse::<i64>().unwrap());
            }
        }
        cur += max.iter().product::<i64>();
    }
    println!("Result: {}", cur);
}
