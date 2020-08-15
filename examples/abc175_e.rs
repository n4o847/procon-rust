use proconio::{fastout, input};

use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        r: usize, c: usize, k: usize,
        rcv: [(Usize1, Usize1, u64); k],
    }
    let mut w = vec![vec![0; c]; r];
    let mut dp = vec![vec![vec![0; c + 1]; r + 1]; 4];
    for (i, j, v) in rcv {
        w[i][j] = v;
    }
    for i in 0..r {
        for j in 0..c {
            dp[0][i + 1][j + 1] = *[
                dp[0][i][j + 1],
                dp[1][i][j + 1],
                dp[2][i][j + 1],
                dp[3][i][j + 1],
                dp[0][i + 1][j],
            ]
            .iter()
            .max()
            .unwrap();
            dp[1][i + 1][j + 1] = *[
                dp[0][i][j + 1] + w[i][j],
                dp[1][i][j + 1] + w[i][j],
                dp[2][i][j + 1] + w[i][j],
                dp[3][i][j + 1] + w[i][j],
                dp[0][i + 1][j] + w[i][j],
                dp[1][i + 1][j],
                //
            ]
            .iter()
            .max()
            .unwrap();
            dp[2][i + 1][j + 1] = *[
                dp[1][i + 1][j] + w[i][j],
                dp[2][i + 1][j],
                //
            ]
            .iter()
            .max()
            .unwrap();
            dp[3][i + 1][j + 1] = *[
                dp[2][i + 1][j] + w[i][j],
                dp[3][i + 1][j],
                //
            ]
            .iter()
            .max()
            .unwrap();
        }
    }
    println!(
        "{}",
        [
            dp[0][r][c],
            dp[1][r][c],
            dp[2][r][c],
            dp[3][r][c],
            //
        ]
        .iter()
        .max()
        .unwrap()
    );
}
