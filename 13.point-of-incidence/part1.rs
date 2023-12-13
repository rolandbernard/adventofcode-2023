use std::io::Read;

fn find_mirror(pattern: &Vec<Vec<bool>>) -> Option<usize> {
    for i in 1..pattern.len() {
        let mut is_mirror = true;
        for k in 0..i.min(pattern.len() - i) {
            if pattern[i - 1 - k] != pattern[i + k] {
                is_mirror = false;
                break;
            }
        }
        if is_mirror {
            return Some(i);
        }
    }
    return None;
}

fn transpose(pattern: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut result = vec![Vec::with_capacity(pattern.len()); pattern[0].len()];
    for line in pattern {
        for (i, val) in line.into_iter().enumerate() {
            result[i].push(val);
        }
    }
    return result;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut col_sum = 0;
    let mut row_sum = 0;
    for pattern_str in input.split("\n\n").map(str::trim) {
        let pattern = pattern_str
            .split("\n")
            .map(|l| l.chars().map(|x| x == '#').collect())
            .collect::<Vec<Vec<_>>>();
        if let Some(i) = find_mirror(&pattern) {
            row_sum += i;
        }
        let pattern = transpose(pattern);
        if let Some(i) = find_mirror(&pattern) {
            col_sum += i;
        }
    }
    println!("Result: {}", col_sum + 100 * row_sum);
}
