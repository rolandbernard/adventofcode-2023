fn count_options(line: &[char], runs: &[i32], lpos: usize, rpos: usize, crun: i32) -> i64 {
    if line.len() == lpos {
        if (rpos == runs.len() && crun == 0) || (rpos == runs.len() - 1 && runs[rpos] == crun) {
            1
        } else {
            0
        }
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
                );
            }
        }
        if char == '#' || char == '?' {
            if rpos < runs.len() && crun < runs[rpos] {
                res += count_options(line, runs, lpos + 1, rpos, crun + 1);
            }
        }
        res
    }
}

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
        let options = count_options(&chars, &runs, 0, 0, 0);
        sum += options;
    }
    println!("Result: {}", sum);
}
