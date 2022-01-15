use proconio::{fastout, input};

#[fastout]
fn main() {
    const M: i64 = 1_000_000_007;
    input! {
        n: usize,
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        dp[i] = dp[i - 1] * 4 % M;
        if i >= 3 {
            // -AGC, -ACG, -GAC
            dp[i] = (dp[i] + M - 3 * dp[i - 3] % M) % M;
        }
        if i >= 4 {
            // -AGGC, -ATGC, -AGTC
            dp[i] = (dp[i] + M - 3 * dp[i - 4] % M) % M;
        }
        for j in 1.. {
            if i >= 3 * j + 1 {
                // +G(ACG)+
                dp[i] = (dp[i] + dp[i - (3 * j + 1)]) % M;
            } else {
                break;
            }
            if i >= 3 * j + 2 {
                // +AC(GAC)+
                dp[i] = (dp[i] + dp[i - (3 * j + 2)]) % M;
            } else {
                break;
            }
            if i >= 3 * j + 3 {
                // -GAC(GAC)+, -ACG(ACG)+
                dp[i] = (dp[i] + M - 2 * dp[i - (3 * j + 3)] % M) % M;
            } else {
                break;
            }
        }
    }
    println!("{}", dp[n]);
}
