use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, p: u64,
    }
    let mut dp = vec![vec![(0, 0); n + 4]; n + 1];
    dp[0][0] = (0, 1);
    dp[0][1] = (1, 0);
    for i in 0..n {
        for j in 0..n {
            // dp[i + 1][j].0 = (dp[i + 1][j].0 + 0) % p;
            dp[i + 1][j].1 = (dp[i + 1][j].1 + dp[i][j].0 + dp[i][j].1) % p;
            dp[i + 1][j + 1].0 = (dp[i + 1][j + 1].0 + dp[i][j].0 * 1) % p;
            dp[i + 1][j + 1].1 = (dp[i + 1][j + 1].1 + dp[i][j].1 * 3) % p;
            dp[i + 1][j + 2].0 = (dp[i + 1][j + 2].0 + dp[i][j].1 * 2) % p;
            // dp[i + 1][j + 2].1 = (dp[i + 1][j + 2].1 + 0) % p;
            // dp[i + 1][j + 3].0 = (dp[i + 1][j + 3].0 + 0) % p;
            // dp[i + 1][j + 3].1 = (dp[i + 1][j + 3].1 + 0) % p;
        }
    }
    for j in 1..n {
        println!("{}", dp[n - 1][j].1);
    }
}
