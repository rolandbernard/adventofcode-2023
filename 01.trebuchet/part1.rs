fn main() {
    let mut cur = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            if c.is_digit(10) {
                if first.is_none() {
                    first = Some(c.to_digit(10).unwrap());
                }
                last = Some(c.to_digit(10).unwrap());
            }
        }
        cur += 10 * first.unwrap() + last.unwrap();
    }
    println!("Result: {}", cur);
}
