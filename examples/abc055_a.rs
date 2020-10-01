use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let ans = 800 * n - 200 * (n / 15);
    println!("{}", ans);
}
