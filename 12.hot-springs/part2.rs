use std::{collections::HashMap, iter};

fn count_options(
    line: &[char],
    runs: &[i32],
    lpos: usize,
    rpos: usize,
    crun: i32,
    cache: &mut HashMap<(usize, usize, i32), i64>,
) -> i64 {
    if line.len() == lpos {
        if (rpos == runs.len() && crun == 0) || (rpos == runs.len() - 1 && runs[rpos] == crun) {
            1
        } else {
            0
        }
    } else if cache.contains_key(&(lpos, rpos, crun)) {
        cache[&(lpos, rpos, crun)]
    } else {
        let char = line[lpos];
        let mut res = 0;
        if char == '.' || char == '?' {
            if crun == 0 || (rpos < runs.len() && runs[rpos] == crun) {
                res += count_options(
                    line,
                    runs,
                    lpos + 1,
                    if crun == 0 { rpos } else { rpos + 1 },
                    0,
                    cache,
                );
            }
        }
        if char == '#' || char == '?' {
            if rpos < runs.len() && crun < runs[rpos] {
                res += count_options(line, runs, lpos + 1, rpos, crun + 1, cache);
            }
        }
        cache.insert((lpos, rpos, crun), res);
        res
    }
}

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
        let options = count_options(&chars, &runs, 0, 0, 0, &mut HashMap::new());
        sum += options;
    }
    println!("Result: {}", sum);
}
