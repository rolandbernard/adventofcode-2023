use std::collections::BTreeSet;

pub fn expanded_distances(n: usize) -> usize {
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
    for (i, j) in &mut galaxies {
        *i = n * *i - (n - 1) * filled_rows.range(..*i).count();
        *j = n * *j - (n - 1) * filled_cols.range(..*j).count();
    }
    let mut sum = 0;
    for (k, &(i0, j0)) in galaxies.iter().enumerate() {
        for &(i1, j1) in galaxies.iter().skip(k + 1) {
            let di = i1.abs_diff(i0);
            let dj = j1.abs_diff(j0);
            sum += di + dj;
        }
    }
    return sum;
}
