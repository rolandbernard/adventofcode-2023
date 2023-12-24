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
    fn new(num: i128, den: i128) -> Self {
        let gcd = gcd(num, den);
        Fraction(num / gcd, den / gcd)
    }

    fn add(self, other: Self) -> Self {
        let gcd = gcd(self.1.abs(), other.1.abs());
        Fraction(
            self.0 * (other.1 / gcd) + other.0 * (self.1 / gcd),
            (self.1 / gcd) * other.1,
        )
    }

    fn smul(self, other: i128) -> Self {
        let gcd = gcd(other.abs(), self.1.abs());
        Fraction(self.0 * (other / gcd), self.1 / gcd)
    }

    fn is_between(self, min: i128, max: i128) -> bool {
        self.1 * min <= self.0 && self.1 * max >= self.0
    }

    fn is_negative(self) -> bool {
        self.0.is_negative() != self.1.is_negative()
    }
}

const MIN: i128 = 200000000000000;
const MAX: i128 = 400000000000000;

fn determinant(mat: [[i128; 2]; 2]) -> i128 {
    mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
}

fn find_intersection(
    vel1: [i128; 2],
    vel2: [i128; 2],
    pos1: [i128; 2],
    pos2: [i128; 2],
) -> Option<[Fraction; 2]> {
    let a = vel1;
    let b = [-vel2[0], -vel2[1]];
    let c = [pos1[0] - pos2[0], pos1[1] - pos2[1]];
    let det = determinant([a, b]);
    if det == 0 {
        None
    } else {
        let det1 = determinant([b, c]);
        let det2 = determinant([c, a]);
        let mul1 = Fraction::new(det1, det);
        let mul2 = Fraction::new(det2, det);
        if mul1.is_negative() || mul2.is_negative() {
            None
        } else {
            Some([
                Fraction(pos1[0], 1).add(mul1.smul(vel1[0])),
                Fraction(pos1[1], 1).add(mul1.smul(vel1[1])),
            ])
        }
    }
}

fn main() {
    let mut paths = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (pos, vel) = line.split_once(" @ ").unwrap();
        let vel = vel
            .split(",")
            .map(|x| x.trim().parse::<i128>().unwrap())
            .collect::<Vec<_>>();
        let pos = pos
            .split(",")
            .map(|x| x.trim().parse::<i128>().unwrap())
            .collect::<Vec<_>>();
        paths.push(([vel[0], vel[1]], [pos[0], pos[1]]));
    }
    let mut count = 0;
    for i in 0..paths.len() {
        for j in i + 1..paths.len() {
            let (vel1, pos1) = paths[i];
            let (vel2, pos2) = paths[j];
            if let Some([ix, iy]) = find_intersection(vel1, vel2, pos1, pos2) {
                if ix.is_between(MIN, MAX) && iy.is_between(MIN, MAX) {
                    count += 1;
                }
            }
        }
    }
    println!("Result: {count}");
}
