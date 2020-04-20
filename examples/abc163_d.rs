use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64, k: u64
    }
    const M: u64 = 1_000_000_007;
    let mut ans = 0;
    for i in k..=n + 1 {
        let a = i * (i - 1) / 2;
        let b = n * i - a;
        ans += b - a + 1;
        ans %= M;
    }
    println!("{}", ans);
}
