const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let mut cur = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut digits = Vec::new();
        for i in 1..=9 {
            for pat in [DIGITS[i].to_owned(), i.to_string()] {
                if let Some(idx) = line.find(&pat) {
                    digits.push((idx, i));
                }
                if let Some(idx) = line.rfind(&pat) {
                    digits.push((idx, i));
                }
            }
        }
        cur += 10 * digits.iter().min_by_key(|(i, _)| i).unwrap().1
            + digits.iter().max_by_key(|(i, _)| i).unwrap().1;
    }
    println!("Result: {}", cur);
}
