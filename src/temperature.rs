pub fn f_to_celsius(f: f64) -> f64 {
    (f - 32.) * 5. / 9.
}

pub fn c_to_fahrenheit(c: f64) -> f64 {
    c * 9. / 5. + 32.
}