use num::integer::gcd;
use proconio::{fastout, input};

use std::cmp::Ordering;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
    fn partition_point<F>(&self, f: F) -> usize
    where
        F: FnMut(&T) -> bool;
}
impl<T: Ord> BinarySearch<T> for Vec<T> {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn partition_point<F>(&self, mut f: F) -> usize
    where
        F: FnMut(&T) -> bool,
    {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            if f(&self[mid]) {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        a: [i64; n],
        s: [i64; q]
    }
    let mut d = 0;
    let b = a
        .iter()
        .map(|x| {
            d = gcd(d, *x);
            d
        })
        .collect::<Vec<_>>();
    for x in s.iter() {
        let i = b.partition_point(|y| gcd(*x, *y) > 1);
        println!(
            "{}",
            if i < b.len() {
                (i + 1) as i64
            } else {
                gcd(d, *x)
            }
        );
    }
}
