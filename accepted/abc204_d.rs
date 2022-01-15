use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    let s: usize = t.iter().sum();
    let mut dp = vec![false; s + 1];
    dp[0] = true;
    for i in 0..n {
        for u in (0..=(s - t[i])).rev() {
            dp[u + t[i]] |= dp[u];
        }
    }
    let mut ans = s;
    for u in 0..=s {
        if dp[u] {
            ans = ans.min(u.max(s - u));
        }
    }
    println!("{}", ans);
}
