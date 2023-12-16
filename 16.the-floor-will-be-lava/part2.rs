mod common;

fn main() {
    let map = common::read_input();
    let mut max = 0;
    for i in 0..map.len() as isize {
        max = max.max(common::count_energy(&map, (i, 0, 0)));
        max = max.max(common::count_energy(
            &map,
            (i, map[0].len() as isize - 1, 1),
        ));
    }
    for j in 0..map[0].len() as isize {
        max = max.max(common::count_energy(&map, (0, j, 2)));
        max = max.max(common::count_energy(&map, (map.len() as isize - 1, j, 3)));
    }
    println!("Result: {max}");
}
