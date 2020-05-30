use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h1: i64, m1: i64, h2: i64, m2: i64, k: i64,
    }
    let ans = std::cmp::max((h2 * 60 + m2) - (h1 * 60 + m1) - k, 0);
    println!("{}", ans);
}
