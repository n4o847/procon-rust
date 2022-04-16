use proconio::{fastout, input};

#[fastout]
fn main() {
    const M: u64 = 998244353;
    input! {
        n: usize, m: usize, k: usize,
    }
    let mut dp = vec![vec![0; k + 1]; n];
    for ai in 1..=m {
        dp[0][ai] = 1;
    }
    for i in 1..n {
        for j in 0..=k {
            for ai in 1..=m {
                if ai > j {
                    break;
                }
                dp[i][j] = (dp[i][j] + dp[i - 1][j - ai]) % M;
            }
        }
    }
    let mut s = 0;
    for j in 0..=k {
        s = (s + dp[n - 1][j]) % M;
    }
    println!("{:?}", s);
}
