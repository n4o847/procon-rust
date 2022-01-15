use proconio::{fastout, input};

#[fastout]
fn main() {
    const M: u64 = 998244353;
    input! {
        n: usize, s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for t in 0..=s {
            dp[i + 1][t] = (dp[i + 1][t] + dp[i][t] * 2) % M;
            if t + a[i] <= s {
                dp[i + 1][t + a[i]] = (dp[i + 1][t + a[i]] + dp[i][t]) % M;
            }
        }
    }
    println!("{}", dp[n][s]);
}
