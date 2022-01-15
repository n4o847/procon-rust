use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, d: i64,
        xy: [(i64, i64); n],
    }
    let ans = xy.iter().filter(|&(x, y)| x * x + y * y <= d * d).count();
    println!("{}", ans);
}
