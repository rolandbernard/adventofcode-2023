use std::{collections::HashMap, io::Read, iter::FromIterator};

#[derive(Debug)]
struct Ranges(Vec<(usize, usize)>);

impl Ranges {
    fn empty() -> Self {
        Ranges(Vec::new())
    }

    fn from_iter<I: Iterator<Item = (usize, usize)>>(iter: I) -> Self {
        let mut result = Ranges(Vec::from_iter(iter));
        result.cleanup();
        result
    }

    fn cleanup(&mut self) {
        self.0.sort();
        if !self.0.is_empty() {
            let mut new_len = 0;
            for i in 0..self.0.len() {
                let range = self.0[i];
                if range.1 != 0 {
                    let last;
                    if new_len != 0 && {
                        last = &mut self.0[new_len - 1];
                        last.0 + last.1 >= range.0
                    } {
                        last.1 = range.0 + range.1 - last.0;
                    } else {
                        self.0[new_len] = range;
                        new_len += 1;
                    }
                }
            }
            self.0.truncate(new_len);
        }
    }

    fn merge(mut self, mut other: Self) -> Self {
        self.0.append(&mut other.0);
        self.cleanup();
        self
    }

    fn split(self, at: usize) -> (Self, Self) {
        let mut lower = Vec::new();
        let mut higher = Vec::new();
        for range in self.0 {
            if range.0 + range.1 <= at {
                lower.push(range);
            } else if at <= range.0 {
                higher.push(range);
            } else {
                lower.push((range.0, at - range.0));
                higher.push((at, range.0 + range.1 - at));
            }
        }
        (Ranges(lower), Ranges(higher))
    }

    fn add_sub(mut self, add: usize, sub: usize) -> Self {
        for range in &mut self.0 {
            range.0 = range.0 + add - sub;
        }
        self
    }

    fn limits(&self) -> Option<(usize, usize)> {
        if self.0.is_empty() {
            None
        } else {
            Some((self.0[0].0, self.0.last().map(|x| x.0 + x.1).unwrap()))
        }
    }
}

struct Mapping {
    mappings: Vec<(usize, usize, usize)>,
}

impl Mapping {
    fn new(mut ranges: Vec<(usize, usize, usize)>) -> Self {
        ranges.sort_by_key(|c| c.1);
        Mapping { mappings: ranges }
    }

    fn map(&self, mut inp: Ranges) -> Ranges {
        let mut result = Ranges::empty();
        for mapping in &self.mappings {
            let (before_start, after_start) = inp.split(mapping.1);
            let (in_middle, after_end) = after_start.split(mapping.1 + mapping.2);
            result = result
                .merge(before_start)
                .merge(in_middle.add_sub(mapping.0, mapping.1));
            inp = after_end;
        }
        result.merge(inp)
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut mappings = HashMap::new();
    let (seed_line, map_lines) = input.split_once("\n\n").unwrap();
    let seed_numbers = seed_line
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for mapping_lines in map_lines.split("\n\n") {
        let (name, rest) = mapping_lines.split_once("\n").unwrap();
        let (source, target) = name.split_once(" ").unwrap().0.split_once("-to-").unwrap();
        mappings.insert(
            source,
            (
                target,
                Mapping::new(
                    rest.split("\n")
                        .filter(|l| !l.is_empty())
                        .map(|l| {
                            let split = l
                                .split_whitespace()
                                .map(|n| n.parse::<usize>().unwrap())
                                .collect::<Vec<_>>();
                            (split[0], split[1], split[2])
                        })
                        .collect(),
                ),
            ),
        );
    }
    let mut whats_needed = Ranges::from_iter(seed_numbers.chunks(2).map(|x| (x[0], x[1])));
    let mut last_target = "seed";
    while last_target != "location" {
        let (target, mapping) = &mappings[last_target];
        whats_needed = mapping.map(whats_needed);
        last_target = target;
    }
    println!("Result: {}", whats_needed.limits().unwrap().0);
}
