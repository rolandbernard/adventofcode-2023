mod common;

fn main() {
    let (mut map, width, height) = common::read_input();
    common::tilt(&mut map, width, height, 0, [width, 1]);
    println!("Result: {}", common::load(&map, width, height));
}
