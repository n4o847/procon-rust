use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut qj = vec![0; n + 1];
    for j in 0..n {
        qj[q[j]] = j;
    }

    let mut ijs = vec![];

    for i in 0..n {
        let mut js = vec![];
        for k in 1.. {
            if p[i] * k > n {
                break;
            }
            let j = qj[p[i] * k];
            js.push(j);
        }
        js.sort();
        js.reverse();
        ijs.extend(js.into_iter());
    }

    let mut l = vec![ijs[0]];
    for &a in ijs.iter() {
        let k = l.lower_bound(&a);
        if k == l.len() {
            l.push(a);
        } else {
            l[k] = a;
        }
    }

    println!("{}", l.len());
}

use binary_search::*;
mod binary_search {
    use std::cmp::Ordering::*;
    use std::ops::Range;
    pub trait BinarySearch<T> {
        fn lower_bound(&self, x: &T) -> usize;
        fn upper_bound(&self, x: &T) -> usize;
        fn equal_range(&self, x: &T) -> Range<usize>;
        fn partition_point<F>(&self, f: F) -> usize
        where
            F: FnMut(&T) -> bool;
    }
    impl<T: Ord> BinarySearch<T> for [T] {
        fn lower_bound(&self, x: &T) -> usize {
            self.partition_point(|y| match y.cmp(x) {
                Less => true,
                Equal | Greater => false,
            })
        }
        fn upper_bound(&self, x: &T) -> usize {
            self.partition_point(|y| match y.cmp(x) {
                Less | Equal => true,
                Greater => false,
            })
        }
        fn equal_range(&self, x: &T) -> Range<usize> {
            self.lower_bound(x)..self.upper_bound(x)
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
}
