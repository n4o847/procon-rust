use cargo_snippet::snippet;

#[snippet("use primes")]
#[snippet(prefix = "use primes::*;")]
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
