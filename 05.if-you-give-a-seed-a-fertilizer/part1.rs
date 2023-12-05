use std::{
    collections::{HashMap, HashSet},
    io::Read,
    iter::FromIterator,
};

struct RangeMap {
    dst_start: usize,
    src_start: usize,
    len: usize,
}

impl RangeMap {
    fn map(&self, inp: usize) -> usize {
        if self.src_start <= inp && inp < self.src_start + self.len {
            self.dst_start + inp - self.src_start
        } else {
            inp
        }
    }
}

struct Mapping {
    ranges: Vec<RangeMap>,
}

impl Mapping {
    fn new(mut ranges: Vec<RangeMap>) -> Self {
        ranges.sort_by_key(|c| c.src_start);
        Mapping { ranges }
    }

    fn map(&self, inp: usize) -> usize {
        let idx = self.ranges.partition_point(|x| x.src_start <= inp);
        if idx == 0 {
            inp
        } else {
            self.ranges[idx - 1].map(inp)
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut mappings = HashMap::new();
    let (seed_line, map_lines) = input.split_once("\n\n").unwrap();
    let seed = seed_line
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for mapping_lines in map_lines.split("\n\n") {
        let (name, rest) = mapping_lines.split_once("\n").unwrap();
        let (source, target) = name.split_once(" ").unwrap().0.split_once("-to-").unwrap();
        mappings.entry(source).or_insert(HashMap::new()).insert(
            target,
            Mapping::new(
                rest.split("\n")
                    .filter(|l| !l.is_empty())
                    .map(|l| {
                        let split = l
                            .split_whitespace()
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect::<Vec<_>>();
                        RangeMap {
                            dst_start: split[0],
                            src_start: split[1],
                            len: split[2],
                        }
                    })
                    .collect(),
            ),
        );
    }
    let mut whats_needed = HashMap::new();
    whats_needed.insert("seed", HashSet::from_iter(seed.iter().cloned()));
    let mut changes = HashMap::new();
    changes.insert("seed", seed);
    while !changes.is_empty() {
        let mut new_changes = HashMap::new();
        for (source, changes) in changes {
            if let Some(mappings) = mappings.get(source) {
                for (&target, mapping) in mappings {
                    for &src_elem in &changes {
                        let dst_elem = mapping.map(src_elem);
                        if whats_needed
                            .entry(target)
                            .or_insert(HashSet::new())
                            .insert(dst_elem)
                        {
                            new_changes
                                .entry(target)
                                .or_insert(Vec::new())
                                .push(dst_elem);
                        }
                    }
                }
            }
        }
        changes = new_changes;
    }
    println!("Result: {}", whats_needed["location"].iter().min().unwrap());
}
