use proconio::{fastout, input};

use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    let mut ans = 0;
    let mut dp = vec![vec![0; n + 1]; n + 1];
    let mut a = a.iter().enumerate().collect::<Vec<_>>();
    a.sort_by_key(|x| std::cmp::Reverse(x.1));
    let abs = |x, y| if x > y { x - y } else { y - x };
    for i in 0..n {
        for j in 0..=i {
            let k = i - j;
            dp[i + 1][j] = max(
                dp[i + 1][j],
                dp[i][j] + a[i].1 * abs(n - 1 - k, a[i].0) as u64,
            );
            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + a[i].1 * abs(a[i].0, j) as u64);
        }
    }
    for x in dp[n].iter() {
        ans = max(ans, *x);
    }
    println!("{}", ans);
}
