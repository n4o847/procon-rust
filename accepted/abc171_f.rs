use proconio::{fastout, input};

use proconio::marker::Chars;

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
    const M: u64 = 1_000_000_007;
    input! {
        k: u64,
        s: Chars,
    }
    let n = s.len() as u64;
    let mut f = vec![1_u64];
    for i in 0_u64..2000000 {
        f.push(f[i as usize] * (i + 1) % M);
    }
    fn c(n: u64, r: u64, f: &Vec<u64>) -> u64 {
        f[n as usize] * pow(f[r as usize], M - 2, M) % M * pow(f[(n - r) as usize], M - 2, M) % M
    }
    fn h(n: u64, r: u64, f: &Vec<u64>) -> u64 {
        c(n + r - 1, r, f)
    }
    let mut t = 0;
    for i in 0..=k {
        t += h(n, i, &f) * pow(25, i, M) % M * pow(26, k - i, M) % M;
        t %= M;
    }
    println!("{}", t);
}
