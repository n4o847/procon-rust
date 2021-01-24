use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, k: usize,
        a: [usize; k],
    }
    let mut dp = vec![(0.0, 0.0); n + 1];
    let mut l = (0.0, 0.0);
    for i in (0..n).rev() {
        if !a.contains(&i) {
            dp[i] = (l.0 / m as f64, 1.0 + l.1 / m as f64);
        } else {
            dp[i] = (1.0, 0.0);
        }
        l.0 += dp[i].0;
        l.1 += dp[i].1;
        if i + m <= n {
            l.0 -= dp[i + m].0;
            l.1 -= dp[i + m].1;
        }
    }
    let ans = if (1.0 - dp[0].0).abs() < 1e-6 {
        -1.0
    } else {
        dp[0].1 / (1.0 - dp[0].0)
    };
    println!("{}", ans);
}
