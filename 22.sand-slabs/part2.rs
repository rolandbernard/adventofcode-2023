use std::collections::{BTreeMap, VecDeque};

type Point = [i64; 3];

struct BBox {
    min: Point,
    max: Point,
}

impl BBox {
    fn fall(&self, to: i64) -> BBox {
        BBox {
            min: [self.min[0], self.min[1], to],
            max: [self.max[0], self.max[1], self.max[2] - (self.min[2] - to)],
        }
    }

    fn intersects(&self, other: &BBox) -> bool {
        (self.min[0] <= other.max[0] && other.min[0] <= self.max[0])
            && (self.min[1] <= other.max[1] && other.min[1] <= self.max[1])
            && (self.min[2] <= other.max[2] && other.min[2] <= self.max[2])
    }
}

fn simulate_fall(
    supports: &Vec<Vec<usize>>,
    supported_by: &Vec<Vec<usize>>,
    destroyed: usize,
) -> usize {
    let mut will_fall = vec![false; supports.len()];
    will_fall[destroyed] = true;
    let mut queue = VecDeque::new();
    queue.push_back(destroyed);
    while let Some(i) = queue.pop_front() {
        for &j in &supports[i] {
            if !will_fall[j] && supported_by[j].iter().all(|&k| will_fall[k]) {
                will_fall[j] = true;
                queue.push_back(j);
            }
        }
    }
    return will_fall.into_iter().filter(|&x| x).count() - 1;
}

fn main() {
    let mut bricks = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (start, end) = line.split_once('~').unwrap();
        let first = start
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i64>>();
        let second = end
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i64>>();
        bricks.push(BBox {
            min: [
                first[0].min(second[0]),
                first[1].min(second[1]),
                first[2].min(second[2]),
            ],
            max: [
                first[0].max(second[0]),
                first[1].max(second[1]),
                first[2].max(second[2]),
            ],
        });
    }
    bricks.sort_by_key(|b| b.min[2]);
    let mut supports = vec![Vec::new(); bricks.len()];
    let mut supported_by = vec![Vec::new(); bricks.len()];
    let mut sorted = BTreeMap::new();
    sorted.insert(0, Vec::new());
    for i in 0..bricks.len() {
        for (d, boxes) in sorted.iter().rev() {
            let fallen = bricks[i].fall(*d);
            let mut supported = false;
            for &b in boxes {
                if fallen.intersects(&bricks[b]) {
                    supported = true;
                    supported_by[i].push(b);
                    let test: &mut Vec<usize> = &mut supports[b];
                    test.push(i);
                }
            }
            if supported || *d == 0 {
                let fallen = fallen.fall(*d + 1);
                sorted.entry(fallen.max[2]).or_insert(Vec::new()).push(i);
                bricks[i] = fallen;
                break;
            }
        }
    }
    let mut count = 0;
    for i in 0..bricks.len() {
        count += simulate_fall(&supports, &supported_by, i);
    }
    println!("Result: {count}");
}
