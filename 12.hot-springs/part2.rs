use std::iter;

mod common;

fn main() {
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (chars_line, runs_line) = line.split_once(" ").unwrap();
        let chars = chars_line
            .chars()
            .chain(
                iter::repeat(iter::once('?').chain(chars_line.chars()))
                    .take(4)
                    .flatten(),
            )
            .collect::<Vec<_>>();
        let runs = iter::repeat(runs_line.split(',').map(|x| x.parse::<i32>().unwrap()))
            .take(5)
            .flatten()
            .collect::<Vec<_>>();
        let options = common::count_options(&chars, &runs);
        sum += options;
    }
    println!("Result: {}", sum);
}
