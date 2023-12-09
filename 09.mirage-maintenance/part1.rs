fn main() {
    let mut cur = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut numbers = Vec::new();
        numbers.push(
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );
        loop {
            let last = numbers.last().unwrap();
            let mut all_zero = true;
            let mut new = Vec::with_capacity(last.len() - 1);
            for (l, n) in last.iter().zip(last.iter().skip(1)) {
                let diff = n - l;
                new.push(diff);
                if diff != 0 {
                    all_zero = false;
                }
            }
            numbers.push(new);
            if all_zero {
                break;
            }
        }
        numbers.last_mut().unwrap().push(0);
        for i in (0..numbers.len() - 1).rev() {
            let next = numbers[i].last().unwrap() + numbers[i + 1].last().unwrap();
            numbers[i].push(next);
        }
        cur += numbers[0].last().unwrap();
    }
    println!("Result: {}", cur);
}
