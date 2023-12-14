use std::collections::HashMap;

mod common;

fn cycle(map: &mut [u8], width: isize, height: isize) {
    common::tilt(map, width, height, 0, [width, 1]);
    common::tilt(map, width, height, 0, [1, width]);
    common::tilt(map, width, height, width * height - 1, [-width, -1]);
    common::tilt(map, width, height, width * height - 1, [-1, -width]);
}

fn main() {
    let (mut map, width, height) = common::read_input();
    let mut seen = HashMap::new();
    let n = 1_000_000_000;
    for step in 1..n {
        cycle(&mut map, width, height);
        if let Some(first) = seen.get(&map) {
            if (n - first) % (step - first) == 0 {
                break;
            }
        } else {
            seen.insert(map.clone(), step);
        }
    }
    println!("Result: {}", common::load(&map, width, height));
}
