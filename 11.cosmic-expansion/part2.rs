use std::collections::BTreeSet;

const N: usize = 1000000;

fn main() {
    let mut filled_rows = BTreeSet::new();
    let mut filled_cols = BTreeSet::new();
    let mut galaxies = Vec::new();
    for (i, line) in std::io::stdin().lines().enumerate() {
        for (j, char) in line.unwrap().chars().enumerate() {
            if char == '#' {
                filled_rows.insert(i);
                filled_cols.insert(j);
                galaxies.push((i, j));
            }
        }
    }
    let mut sum = 0;
    for (k, &(i0, j0)) in galaxies.iter().enumerate() {
        for &(i1, j1) in galaxies.iter().skip(k + 1) {
            let di =
                N * i1.abs_diff(i0) - (N - 1) * filled_rows.range(i0.min(i1)..i0.max(i1)).count();
            let dj =
                N * j1.abs_diff(j0) - (N - 1) * filled_cols.range(j0.min(j1)..j0.max(j1)).count();
            sum += di + dj;
        }
    }
    println!("Result: {}", sum);
}
