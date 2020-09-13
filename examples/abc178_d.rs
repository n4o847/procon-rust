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
    let inv = |x| pow(x, M - 2, M);
    input! {
        s: u64,
    }
    const M: u64 = 1_000_000_007;
    let mut f = vec![1; 10_000];
    for i in 1..f.len() {
        f[i] = f[i - 1] * i as u64 % M;
    }
    let mut ans = 0;
    for k in 1..=s / 3 {
        let t = s - k * 3;
        let sub = f[(t + (k - 1)) as usize] * inv(f[(k - 1) as usize]) % M * inv(f[t as usize]) % M;
        ans += sub;
        ans %= M;
    }
    println!("{}", ans);
}
