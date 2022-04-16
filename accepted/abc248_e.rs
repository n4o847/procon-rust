use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        xy: [(i64, i64); n],
    }
    if k == 1 {
        println!("Infinity");
        return;
    }
    let mut h = HashMap::new();
    for i in 0..n {
        let (xi, yi) = xy[i];
        for j in (i + 1)..n {
            let (xj, yj) = xy[j];
            let mut x = xj - xi;
            let mut y = yj - yi;
            let g = num::integer::gcd(x, y);
            x = x / g;
            y = y / g;
            if x < 0 {
                x *= -1;
                y *= -1;
            } else if x == 0 && y < 0 {
                y *= -1;
            }
            let (cx, cy) = if x == 0 {
                (xi, 0)
            } else {
                (xi.rem_euclid(x), yi - y * xi.div_euclid(x))
            };
            *h.entry((x, y, cx, cy)).or_insert(0) += 1;
        }
    }
    let mut ans = 0;
    for &v in h.values() {
        if v >= k * (k - 1) / 2 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
