fn access(map: &mut [u8], i: isize, j: isize, offset: isize, stride: [isize; 2]) -> &mut u8 {
    &mut map[(offset + stride[0] * i + stride[1] * j) as usize]
}

pub fn tilt(map: &mut [u8], width: isize, height: isize, offset: isize, stride: [isize; 2]) {
    for j in 0..width {
        let mut ni = 0;
        for i in 0..(height) {
            match access(map, i, j, offset, stride) {
                b'O' => {
                    *access(map, i, j, offset, stride) = b'.';
                    *access(map, ni, j, offset, stride) = b'O';
                    ni += 1;
                }
                b'#' => ni = i + 1,
                _ => {}
            }
        }
    }
}

pub fn load(map: &[u8], width: isize, height: isize) -> isize {
    let mut load = 0;
    for i in 0..height {
        for j in 0..width {
            if map[(i * width + j) as usize] == b'O' {
                load += height - i;
            }
        }
    }
    return load;
}

pub fn read_input() -> (Vec<u8>, isize, isize) {
    let mut width = 0;
    let mut height = 0;
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.extend(line.bytes());
        width = line.len() as isize;
        height += 1;
    }
    return (map, width, height);
}
