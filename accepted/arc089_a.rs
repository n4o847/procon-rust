use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize }
    let (mut t0, mut x0, mut y0) = (0, 0, 0);
    let ans = (0..n).all(|_| {
        input! { t1: i64, x1: i64, y1: i64 }
        let d = (x1 - x0).abs() + (y1 - y0).abs();
        let t = t1 - t0;
        t0 = t1;
        x0 = x1;
        y0 = y1;
        d <= t && d % 2 == t % 2
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
