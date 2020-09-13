use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64, b: i64, c: i64, d: i64,
    }
    let ans = vec![a * c, a * d, b * c, b * d].into_iter().max().unwrap();
    println!("{}", ans);
}
