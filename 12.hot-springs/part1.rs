mod common;

fn main() {
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (chars_line, runs_line) = line.split_once(" ").unwrap();
        let chars = chars_line.chars().collect::<Vec<_>>();
        let runs = runs_line
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let options = common::count_options(&chars, &runs);
        sum += options;
    }
    println!("Result: {}", sum);
}
