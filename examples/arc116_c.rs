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
        n: u64, m: u64,
    }
    const M: u64 = 998244353;
    let mut f = vec![1; 200020];
    for i in 1..f.len() {
        f[i] = f[i - 1] * (i as u64) % M;
    }

    let comb = |n: u64, r: u64| {
        f[n as usize] * inv(f[r as usize], M) % M * inv(f[(n - r) as usize], M) % M
    };

    let multicomb = |n: u64, r: u64| comb(n + r - 1, r);

    let mut mc = vec![0; 20];
    for e in 0..20 {
        mc[e] = multicomb(n, e as u64);
    }

    let ps = Sieve::generate(m as _);

    let mut ans = 0;
    for k in 1..=m {
        let mut prod = 1;
        for (_p, e) in ps.prime_factorization(k as _) {
            prod = prod * mc[e] % M;
        }
        ans = (ans + prod) % M;
    }
    println!("{}", ans);
}

use primes::*;
mod primes {
    use std::collections::BTreeMap;
    pub struct Sieve {
        spf: Vec<usize>,
    }
    impl Sieve {
        #[allow(dead_code)]
        pub fn generate(n: usize) -> Self {
            let mut spf: Vec<_> = (0..=n).collect();
            let mut i = 2;
            while i * i <= n {
                if spf[i] == i {
                    let mut j = i * i;
                    while j <= n {
                        if spf[j] == j {
                            spf[j] = i;
                        }
                        j += i;
                    }
                }
                i += 1;
            }
            Self { spf }
        }
        #[allow(dead_code)]
        pub fn prime_factorization(&self, n: usize) -> BTreeMap<usize, usize> {
            let mut map = BTreeMap::new();
            let mut n = n;
            while n != 1 {
                *map.entry(self.spf[n]).or_insert(0) += 1;
                n /= self.spf[n];
            }
            map
        }
    }
}
