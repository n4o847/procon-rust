use proconio::{fastout, input};

use num::complex::Complex;
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        a: f64, b: f64, h: f64, m: f64,
    }
    let c = a * Complex::new(0.0, (h + m / 60.0) / 12.0 * 2.0 * PI).exp();
    let d = b * Complex::new(0.0, m / 60.0 * 2.0 * PI).exp();
    println!("{}", (c - d).norm());
}
