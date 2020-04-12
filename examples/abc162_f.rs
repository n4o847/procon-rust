use proconio::{fastout, input};

use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [i64; n]
    }
    let mut u = vec![0; n + 1];
    let mut v = vec![0; n + 1];
    let mut w = vec![0; n + 1];
    let mut x = vec![0; n + 1];
    let mut y = vec![0; n + 1];
    let mut z = vec![0; n + 1];
    for i in 0..n {
        u[i + 1] = x[i] + aa[i];
        v[i + 1] = y[i] + aa[i];
        w[i + 1] = z[i] + aa[i];
        x[i + 1] = u[i];
        y[i + 1] = max(x[i], v[i]);
        z[i + 1] = max(y[i], w[i]);
    }
    // println!("{} {} {} {} {} {}", u[n], v[n], w[n], x[n], y[n], z[n]);
    if n % 2 == 1 {
        println!("{}", max(w[n], y[n]));
    } else {
        println!("{}", max(v[n], x[n]));
    }
}
