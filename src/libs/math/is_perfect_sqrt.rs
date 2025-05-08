pub fn is_perfect_square(n: i32) -> bool {
    if n < 0 {}
    let sqrt = (n as f64).sqrt() as i32;
    sqrt * sqrt == n
}
