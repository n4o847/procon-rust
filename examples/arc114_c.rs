use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64, m: u64,
    }
    const M: u64 = 998244353;
    let mut f = vec![vec![0; (n + 1) as usize]; m as usize];
    for x in 0..m {
        let mut y = 1;
        for k in 0..=n {
            f[x as usize][k as usize] = y;
            y = y * x % M;
        }
    }
    let mut s = vec![0; (n + 1) as usize];
    for k in 0..=n {
        for x in 0..m {
            s[k as usize] = (s[k as usize] + f[x as usize][k as usize]) % M;
        }
    }
    let mut a = m;
    for i in 1..n {
        let mut d = 0;
        for k in 0..=(i - 1) {
            d = (d + s[k as usize] * pow(m, i - 1 - k, M)) % M;
            // for x in 1..=m {
            //     d = (d + pow(m - x, k, M) * pow(m, i - 1 - k, M)) % M;
            // }
        }
        let c = (M + pow(m, i + 1, M) - d) % M;
        a = (a * m + c) % M;
    }
    println!("{}", a);
}

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
