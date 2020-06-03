use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let prod = a.into_iter().fold(1_i64, |acc, x| acc.saturating_mul(x));
    let ans = if prod > 1_000_000_000_000_000_000 {
        -1
    } else {
        prod
    };
    println!("{}", ans);
}
