use std::vec;

use proconio::marker::Usize1;
use proconio::{fastout, input};

fn pow(mut x: u64, mut y: u64, z: u64) -> u64 {
    let mut r = 1;
    while y > 0 {
        if y & 1 == 1 {
            r = r * x % z;
        }
        x = x * x % z;
        y >>= 1;
    }
    r
}

#[fastout]
fn main() {
    const M: u64 = 998244353;
    input! {
        h: usize, w: usize, k: usize,
        hwc: [(Usize1, Usize1, char); k],
    }
    let mut mp = vec![vec![' '; w + 1]; h + 1];
    for &(h, w, c) in hwc.iter() {
        mp[h][w] = c;
    }
    let mut dp = vec![vec![0; w + 1]; h + 1];
    dp[0][0] = 1;
    let tt = 2 * pow(3, M - 2, M) % M;
    for i in 0..h {
        for j in 0..w {
            match mp[i][j] {
                'D' => {
                    dp[i + 1][j] += dp[i][j];
                    dp[i + 1][j] %= M;
                }
                'R' => {
                    dp[i][j + 1] += dp[i][j];
                    dp[i][j + 1] %= M;
                }
                'X' => {
                    dp[i + 1][j] += dp[i][j];
                    dp[i + 1][j] %= M;
                    dp[i][j + 1] += dp[i][j];
                    dp[i][j + 1] %= M;
                }
                _ => {
                    dp[i + 1][j] += dp[i][j] * tt % M;
                    dp[i + 1][j] %= M;
                    dp[i][j + 1] += dp[i][j] * tt % M;
                    dp[i][j + 1] %= M;
                }
            }
        }
    }
    println!(
        "{}",
        dp[h - 1][w - 1] * pow(3, h as u64 * w as u64 - k as u64, M) % M
    );
}
