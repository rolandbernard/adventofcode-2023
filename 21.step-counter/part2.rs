use std::collections::HashSet;

mod common;

fn gcd(a: i128, b: i128) -> i128 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

#[derive(Clone, Copy)]
struct Fraction(i128, i128);

impl Fraction {
    fn add(self, other: Self) -> Self {
        let gcd = gcd(self.1.abs(), other.1.abs());
        Fraction(
            self.0 * (other.1 / gcd) + other.0 * (self.1 / gcd),
            (self.1 / gcd) * other.1,
        )
    }

    fn mul(self, other: Self) -> Self {
        let gcd1 = gcd(self.0.abs(), other.1.abs());
        let gcd2 = gcd(other.0.abs(), self.1.abs());
        Fraction(
            (self.0 / gcd1) * (other.0 / gcd2),
            (self.1 / gcd2) * (other.1 / gcd1),
        )
    }

    fn smul(self, other: i128) -> Self {
        let gcd = gcd(other.abs(), self.1.abs());
        Fraction(self.0 * (other / gcd), self.1 / gcd)
    }
}

struct Quad(Fraction, Fraction, Fraction);

impl Quad {
    fn fit(points: &[(i128, i128)]) -> Self {
        fn basis(points: &[(i128, i128)], i: usize) -> Quad {
            let mut res = Quad(Fraction(1, 1), Fraction(0, 1), Fraction(0, 1));
            for j in 0..3 {
                if j != i {
                    let div = points[i].0 - points[j].0;
                    res = res.mul(Quad(
                        Fraction(-points[j].0, div),
                        Fraction(1, div),
                        Fraction(0, 1),
                    ));
                }
            }
            return res;
        }
        basis(points, 0)
            .smul(points[0].1)
            .add(basis(points, 1).smul(points[1].1))
            .add(basis(points, 2).smul(points[2].1))
    }

    fn apply(&self, x: i128) -> i128 {
        let exact = self.0.add(self.1.smul(x)).add(self.2.smul(x * x));
        exact.0 / exact.1
    }

    fn add(self, other: Self) -> Self {
        Quad(
            self.0.add(other.0),
            self.1.add(other.1),
            self.2.add(other.2),
        )
    }

    fn mul(self, other: Self) -> Self {
        Quad(
            self.0.mul(other.0),
            self.0.mul(other.1).add(self.1.mul(other.0)),
            self.0
                .mul(other.2)
                .add(self.1.mul(other.1))
                .add(self.2.mul(other.0)),
        )
    }

    fn smul(self, other: i128) -> Self {
        Quad(self.0.smul(other), self.1.smul(other), self.2.smul(other))
    }
}

fn count_options(map: &Vec<Vec<char>>, start: (i64, i64), steps: usize) -> i64 {
    let mut reachable = HashSet::new();
    let mut count = [0, 0];
    let mut counts = Vec::new();
    let mut front = vec![start];
    for step in 1..=steps {
        let mut new_front = Vec::new();
        for (i, j) in front {
            for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ni, nj) = (i + di, j + dj);
                let (ri, rj) = (
                    ni.rem_euclid(map.len() as i64) as usize,
                    nj.rem_euclid(map[0].len() as i64) as usize,
                );
                if map[ri][rj] == '.' && !reachable.contains(&(step % 2, ni, nj)) {
                    count[step % 2] += 1;
                    reachable.insert((step % 2, ni, nj));
                    new_front.push((ni, nj));
                }
            }
        }
        front = new_front;
        if step % map.len() == steps % map.len() {
            counts.push((step as i128, count[step % 2] as i128));
            if counts.len() >= 4 {
                let poly = Quad::fit(&counts[counts.len() - 4..]);
                if counts.last().unwrap().1 == poly.apply(step as i128) {
                    return poly.apply(steps as i128) as i64;
                }
            }
        }
    }
    return count[steps % 2];
}

fn main() {
    let (map, start) = common::read_input();
    println!("Result: {}", count_options(&map, start, 26501365));
}
