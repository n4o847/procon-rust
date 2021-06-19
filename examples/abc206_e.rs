use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: usize, r: usize,
    }
    let ps = Sieve::generate(r);
    let mut s = 0;
    for g in 2..=r {
        let fs = ps.prime_factorization(g);
        if fs.values().any(|&v| v != 1) {
            continue;
        }
        let c = r / g - (l - 1) / g;
        if fs.len() % 2 == 1 {
            s += c * (c - 1) / 2;
        } else {
            s -= c * (c - 1) / 2;
        }
    }
    for g in l.max(2)..=r {
        s -= r / g - 1;
    }
    println!("{}", s * 2);
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
