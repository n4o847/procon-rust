use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        cp: [(Usize1, u64); n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }
    let mut t0 = FenwickTree::new(n);
    let mut t1 = FenwickTree::new(n);
    for i in 0..n {
        let (c, p) = cp[i];
        match c {
            0 => t0.add(i, p),
            1 => t1.add(i, p),
            _ => (),
        }
    }
    for &(l, r) in lr.iter() {
        let s0 = t0.sum(l..=r);
        let s1 = t1.sum(l..=r);
        println!("{} {}", s0, s1);
    }
}

use fenwick_tree::*;
mod fenwick_tree {
    use num::traits::Zero;
    use std::ops::{AddAssign, Bound::*, RangeBounds, Sub};
    pub struct FenwickTree<T> {
        data: Vec<T>,
    }
    impl<T> FenwickTree<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + Zero,
    {
        #[allow(dead_code)]
        pub fn new(n: usize) -> Self {
            Self {
                data: vec![T::zero(); n],
            }
        }
        #[allow(dead_code)]
        pub fn add(&mut self, mut i: usize, x: T) {
            i += 1;
            while i <= self.data.len() {
                self.data[i - 1] += x;
                i += i & i.wrapping_neg();
            }
        }
        #[allow(dead_code)]
        pub fn cumsum(&self, mut i: usize) -> T {
            let mut s = T::zero();
            while i > 0 {
                s += self.data[i - 1];
                i -= i & i.wrapping_neg();
            }
            s
        }
        #[allow(dead_code)]
        pub fn sum(&self, index: impl RangeBounds<usize>) -> T {
            let l = match index.start_bound() {
                Included(&i) => i,
                Excluded(&i) => i + 1,
                Unbounded => 0,
            };
            let r = match index.end_bound() {
                Included(&i) => i + 1,
                Excluded(&i) => i,
                Unbounded => self.data.len() - 1,
            };
            self.cumsum(r) - self.cumsum(l)
        }
    }
}
