use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut xyz: [(Usize1, Usize1, usize); m],
    }
    let mut cns = vec![vec![]; n];
    for &(x, y, z) in xyz.iter() {
        cns[x].push((x, y, z));
    }
    let mut dp = vec![vec![0_u64; 1 << n]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for s in 0..(1 << n) {
            for a in 0..n {
                if s & (1 << a) != 0 {
                    continue;
                }
                if cns[i].iter().all(|&(_x, y, z)| {
                    let cnt = (0..=y)
                        .filter(|k| (s | (1 << a)) & (1_usize << k) != 0)
                        .count();
                    cnt <= z
                }) {
                    dp[i + 1][s | (1 << a)] += dp[i][s];
                }
            }
        }
    }
    println!("{}", &dp[n][(1 << n) - 1]);
}
