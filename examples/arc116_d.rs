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

fn inv(x: u64, m: u64) -> u64 {
    pow(x, m - 2, m)
}

#[fastout]
fn main() {
    input! {
        n: u64, mut m: u64,
    }
    const M: u64 = 998244353;

    let mut s = 0;
    {
        let mut k = m;
        while k > 0 {
            k >>= 1;
            s += 1;
        }
    }

    let mut f = vec![1; n as usize + 1];
    for i in 1..f.len() {
        f[i] = f[i - 1] * (i as u64) % M;
    }

    let comb = |n: u64, r: u64| {
        f[n as usize] * inv(f[r as usize], M) % M * inv(f[(n - r) as usize], M) % M
    };

    let mut c = vec![0; n as usize + 1];
    for l in 0..=n as usize {
        c[l] = comb(n, l as u64);
    }

    let mut dp = vec![vec![0; m as usize + 1]; s + 1];
    dp[s - 1][0] = 1;
    dp[s - 1][1 << (s - 1)] = 0;
    for i in (0..(s - 1)).rev() {
        for j in 0..=m as usize {
            let mut l = 0;
            loop {
                if l > n as usize {
                    break;
                }
                if j + (1 << i) * l > m as usize {
                    break;
                }
                dp[i][j + (1 << i) * l] += c[l] * dp[i + 1][j] % M;
                dp[i][j + (1 << i) * l] %= M;
                l += 2;
            }
        }
    }
    println!("{}", dp[0][m as usize]);
}
