use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    const M: u64 = 998244353;
    input! {
        n: usize, l: u64,
        s: [Chars; n],
    }
    let mut z = vec![0; n];
    for i in 0..n {
        for &c in s[i].iter() {
            z[i] |= 1 << (c as usize - 'a' as usize);
        }
    }
    let mut ans = 0;
    for b in 1..(1 << n) {
        let mut bc = 0;
        let mut p = (1 << 26) - 1;
        for i in 0..n {
            if (b >> i) & 1 == 1 {
                p &= z[i];
                bc += 1;
            }
        }
        let mut c = 0;
        for k in 0..26 {
            if (p >> k) & 1 == 1 {
                c += 1;
            }
        }
        if bc & 1 == 1 {
            ans = (ans + pow_mod(c, l, M)) % M;
        } else {
            ans = (ans + (M - pow_mod(c, l, M))) % M;
        }
    }
    println!("{}", ans);
}

#[allow(dead_code)]
fn pow_mod(mut x: u64, mut n: u64, m: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = r * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    r
}
