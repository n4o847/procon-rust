use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    const M: u64 = 1_000_000_007;
    let mut dp = vec![0; 7];
    for i in 0..n {
        match s[i] {
            'a' => dp[0] = (dp[0] + 1) % M,
            't' => dp[1] = (dp[1] + dp[0]) % M,
            'c' => dp[2] = (dp[2] + dp[1]) % M,
            'o' => dp[3] = (dp[3] + dp[2]) % M,
            'd' => dp[4] = (dp[4] + dp[3]) % M,
            'e' => dp[5] = (dp[5] + dp[4]) % M,
            'r' => dp[6] = (dp[6] + dp[5]) % M,
            _ => (),
        }
    }
    println!("{}", dp[6]);
}
