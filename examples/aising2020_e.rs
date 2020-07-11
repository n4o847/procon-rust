use proconio::{fastout, input};

use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut klr: [(usize, i64, i64); n],
        }
        let mut ans = 0;
        klr.sort_by_key(|&(k, _, _)| k);
        for (_, l, r) in klr.iter() {
            ans += min(l, r);
        }
        let (a, b): (Vec<_>, Vec<_>) = klr.into_iter().partition(|&(_, l, r)| l >= r);
        let mut s = BinaryHeap::new();
        let mut idx = 0;
        for i in 1..=n {
            while idx < a.len() && a[idx].0 == i {
                let (_, l, r) = a[idx];
                s.push(Reverse(l - r));
                idx += 1;
            }
            while s.len() > i {
                s.pop();
            }
        }
        while let Some(Reverse(lr)) = s.pop() {
            ans += lr;
        }
        let mut idx = b.len();
        for i in 1..=n {
            while idx > 0 && b[idx - 1].0 == n {
                idx -= 1;
            }
            while idx > 0 && b[idx - 1].0 == n - i {
                let (_, l, r) = b[idx - 1];
                s.push(Reverse(r - l));
                idx -= 1;
            }
            while s.len() > i {
                s.pop();
            }
        }
        while let Some(Reverse(rl)) = s.pop() {
            ans += rl;
        }
        println!("{}", ans);
    }
}
