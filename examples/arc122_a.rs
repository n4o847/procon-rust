use proconio::{fastout, input};

#[fastout]
fn main() {
    const M: i64 = 1000000007;
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![((0, 0), (0, 0)); n + 1];
    dp[0].0 = (a[0], 1);
    for i in 1..n {
        let ((s, t), (u, v)) = dp[i - 1];
        dp[i] = (
            ((s + t * a[i] + u + v * a[i]) % M, (t + v) % M),
            ((s - t * a[i]) % M, t),
        );
    }
    let ans = (dp[n - 1].0).0 + (dp[n - 1].1).0;
    println!("{}", ans.rem_euclid(M));
}
