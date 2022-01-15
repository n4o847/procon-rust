use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, u64); n],
    }
    dcs.sort();
    let mut dp = vec![0; dcs[n - 1].0 + 1];
    for &(d, c, s) in dcs.iter() {
        if c > d {
            continue;
        }
        for a in (0..=(d - c)).rev() {
            dp[a + c] = dp[a + c].max(dp[a] + s);
        }
    }
    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}
