use std::collections::HashSet;

pub type Point = [usize; 3];

pub struct BBox {
    min: Point,
    max: Point,
}

pub fn read_input() -> Vec<BBox> {
    let mut bricks = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (start, end) = line.split_once('~').unwrap();
        let first = start
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<usize>>();
        let second = end
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<usize>>();
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
    return bricks;
}

pub fn build_support_graph(mut boxes: Vec<BBox>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut supports = vec![Vec::new(); boxes.len()];
    let mut supported_by = vec![Vec::new(); boxes.len()];
    let max_x = boxes.iter().map(|b| b.max[0]).max().unwrap();
    let max_y = boxes.iter().map(|b| b.max[1]).max().unwrap();
    let mut height_map = vec![vec![0; max_y + 1]; max_x + 1];
    let mut top_map = vec![vec![0; max_y + 1]; max_x + 1];
    boxes.sort_by_key(|b| b.min[2]);
    for (k, b) in boxes.into_iter().enumerate() {
        let mut d = 0;
        let mut sup = HashSet::new();
        for i in b.min[0]..=b.max[0] {
            for j in b.min[1]..=b.max[1] {
                if height_map[i][j] > d {
                    d = height_map[i][j];
                    sup.clear();
                    sup.insert(top_map[i][j]);
                } else if d != 0 && height_map[i][j] == d {
                    sup.insert(top_map[i][j]);
                }
            }
        }
        for i in b.min[0]..=b.max[0] {
            for j in b.min[1]..=b.max[1] {
                top_map[i][j] = k;
                height_map[i][j] = d + 1 + b.max[2] - b.min[2];
            }
        }
        for s in sup {
            supports[s].push(k);
            supported_by[k].push(s);
        }
    }
    return (supports, supported_by);
}
