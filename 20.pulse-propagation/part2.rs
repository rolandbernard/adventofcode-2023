use std::{collections::HashMap, io::Read};

mod common;

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_r, old_s, old_t, s, t)
}

fn find_offset(o: i64, p: i64, s: i64, r: i64) -> Option<i64> {
    let (gcd, x, y, zx, zy) = extended_gcd(p, r);
    if (s - o) % gcd != 0 {
        return None;
    }
    let off;
    if o > s {
        off = o + ((s - o) / gcd * x).rem_euclid(zx) * p;
    } else {
        off = s + ((o - s) / gcd * y).rem_euclid(zy) * r;
    }
    return Some(off);
}

fn lcm(a: i64, b: i64) -> i64 {
    a / extended_gcd(a, b).0 * b
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let (mut network, reverse) = common::parse_input(&input);
    let prev = &reverse["rx"];
    let specials = &reverse[prev[0]];
    let mut last_special = HashMap::new();
    for press in 1.. {
        if common::evaluate_press(&mut network, |_, to, high| {
            if !high {
                if let Some(to) = specials.iter().find(|&&e| e == to) {
                    if let Some((last, _)) = last_special.get(to) {
                        last_special.insert(to, (press, press - last));
                    } else {
                        last_special.insert(to, (press, 0));
                    }
                }
            }
            last_special.len() == specials.len() && last_special.values().all(|x| x.1 != 0)
        }) {
            break;
        }
    }
    let mut step = 0;
    let mut rep = 1;
    for (offset, period) in last_special.into_values() {
        (step, rep) = (
            find_offset(step, rep, offset, period).unwrap(),
            lcm(rep, period),
        );
    }
    println!("Result: {step}");
}
