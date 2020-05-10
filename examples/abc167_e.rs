use proconio::{fastout, input};

fn pow(mut x: u64, mut y: u64, z: u64) -> u64 {
    let mut a = 1;
    while y > 0 {
        if y & 1 == 1 {
            a = a * x % z;
        }
        x = x * x % z;
        y = y >> 1;
    }
    a
}

fn inv(x: u64, m: u64) -> u64 {
    pow(x, m - 2, m)
}

#[fastout]
fn main() {
    input! {
        n: u64, m: u64, k: u64,
    }
    const M: u64 = 998244353;
    let mut fact = vec![0; 200_100];
    fact[0] = 1;
    for i in 1..fact.len() {
        fact[i] = i as u64 * fact[i - 1] % M;
    }
    let mut ans = 0;
    for i in 0..=k {
        let cnt = m * pow(m - 1, n - 1 - i, M) % M;
        let nn = (n - i) as usize;
        let rr = i as usize;
        let nr = nn + rr - 1;
        let comb = fact[nr] * inv(fact[rr], M) % M * inv(fact[nr - rr], M) % M;
        ans = (ans + cnt * comb % M) % M;
    }
    println!("{}", ans);
}
