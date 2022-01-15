use std::collections::{BTreeMap, VecDeque};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        mut b: [u64; n],
    }
    for i in 0..n {
        a[i] += i as u64;
        b[i] += i as u64;
    }
    let mut a_ = a.clone();
    a_.sort();
    let mut b_ = b.clone();
    b_.sort();
    if a_ != b_ {
        println!("-1");
        return;
    }
    let mut idx = BTreeMap::new();
    for i in 0..n {
        idx.entry(b[i])
            .or_insert_with(|| VecDeque::new())
            .push_back(i);
    }
    let mut c = vec![0; n];
    for i in 0..n {
        c[i] = idx.get_mut(&a[i]).unwrap().pop_front().unwrap();
    }
    let mut ft = FenwickTree::new(n);
    let mut ans = 0;
    for i in 0..n {
        ans += i - ft.sum(..c[i]);
        ft.add(c[i], 1);
    }
    println!("{}", ans);
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
