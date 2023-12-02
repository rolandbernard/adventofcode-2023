const COLORS: [&str; 3] = ["red", "green", "blue"];
const MAX: [i32; 3] = [12, 13, 14];

fn main() {
    let mut cur = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (header, content) = line.split_once(": ").unwrap();
        let game_id = header.split_once(" ").unwrap().1.parse::<i32>().unwrap();
        let mut max = [0, 0, 0];
        for subsets in content.split("; ") {
            for part in subsets.split(", ") {
                let (count, color) = part.split_once(" ").unwrap();
                let idx = COLORS.iter().position(|&x| x == color).unwrap();
                max[idx] = max[idx].max(count.parse::<i32>().unwrap());
            }
        }
        if MAX.iter().zip(max.iter()).all(|(x, y)| x >= y) {
            cur += game_id;
        }
    }
    println!("Result: {}", cur);
}
