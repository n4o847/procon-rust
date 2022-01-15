use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        b: i64, c: i64,
    }
    let (d, e, f, g) = if b >= 0 {
        let d = b + (c - 2) / 2;
        let e = b - c / 2;
        let f = -b + (c - 1) / 2;
        let g = -b - (c - 1) / 2;
        (d, e, f, g)
    } else {
        let d = -b + (c - 1) / 2;
        let e = -b - (c - 1) / 2;
        let f = b + (c - 2) / 2;
        let g = b - c / 2;
        (d, e, f, g)
    };
    let ans = if e > f {
        (d - e + 1) + (f - g + 1)
    } else {
        max(d, f) - min(e, g) + 1
    };
    println!("{}", ans);
}
