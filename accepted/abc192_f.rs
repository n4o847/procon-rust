use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize, x: i64,
        a: [i64; n],
    }
    let mut ans = std::i64::MAX;
    for k in 1..=n {
        // dp[i][r] = i 個選んで余りが r のときの最大値
        let mut dp = vec![vec![None; k]; k + 1];
        dp[0][0] = Some(0);
        for &a in a.iter() {
            for i in (0..k).rev() {
                for r in 0..k {
                    dp[i + 1][((r as i64 + a) % k as i64) as usize] = max(
                        dp[i + 1][((r as i64 + a) % k as i64) as usize],
                        dp[i][r].map(|x| x + a),
                    );
                }
            }
        }
        if let Some(m) = dp[k][(x % k as i64) as usize] {
            ans = min(ans, (x - m) / k as i64);
        }
    }
    println!("{}", ans);
}
