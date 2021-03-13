use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64,
    }
    let ps = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];
    let mut t = vec![0; (b - a + 1) as usize];
    for x in a..=b {
        for i in 0..20 {
            if x % ps[i] == 0 {
                t[(x - a) as usize] |= 1 << i;
            }
        }
    }
    let mut dp = vec![0; 1 << 20];
    dp[0] = 1;
    for k in 0..(b - a + 1) as usize {
        for b in 0..(1 << 20) {
            if b & t[k] != 0 {
                continue;
            }
            dp[b | t[k]] += dp[b];
        }
    }
    let mut ans: u64 = dp.iter().sum();
    println!("{}", ans);
}
