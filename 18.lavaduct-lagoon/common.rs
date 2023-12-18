pub fn area_from_points(points: Vec<(i64, i64)>) -> i64 {
    let mut count0 = 0;
    let mut count1 = 0;
    for (&(x0, y0), &(x1, y1)) in points.iter().zip(points.iter().skip(1)) {
        count0 += (x0 * y1) - (x1 * y0);
        count1 += (x1 - x0).abs() + (y1 - y0).abs();
    }
    return count0.abs() / 2 + count1 / 2 + 1;
}
