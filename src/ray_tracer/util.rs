
pub fn f64_is_equal(a: f64, b: f64) -> bool {
    let epsilon = 0.00001;
    (a - b).abs() < epsilon
}
