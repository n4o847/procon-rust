use proconio::{fastout, input};

use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! { u: Usize1, v: Usize1 }
        g[u].push(v);
        g[v].push(u);
    }
    let mut ans: Vec<usize> = vec![0; n];
    let mut lis: Vec<u64> = vec![a[0]];
    fn f(
        s: usize,
        p: usize,
        n: usize,
        a: &Vec<u64>,
        g: &Vec<Vec<usize>>,
        ans: &mut Vec<usize>,
        lis: &mut Vec<u64>,
    ) {
        ans[s] = lis.len();
        for t in g[s].iter() {
            if *t == p {
                continue;
            }
            let i = lis.lower_bound(&a[*t]);
            if i < lis.len() {
                let prev = lis[i];
                lis[i] = a[*t];
                f(*t, s, n, a, g, ans, lis);
                lis[i] = prev;
            } else {
                lis.push(a[*t]);
                f(*t, s, n, a, g, ans, lis);
                lis.pop();
            }
        }
    };
    f(0, n, n, &a, &g, &mut ans, &mut lis);
    for x in ans.iter() {
        println!("{}", *x);
    }
}

use std::cmp::Ordering;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
    fn partition_point<F>(&self, f: F) -> usize
    where
        F: FnMut(&T) -> bool;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        self.partition_point(|y| match y.cmp(x) {
            Ordering::Less => true,
            Ordering::Equal | Ordering::Greater => false,
        })
    }
    fn upper_bound(&self, x: &T) -> usize {
        self.partition_point(|y| match y.cmp(x) {
            Ordering::Less | Ordering::Equal => true,
            Ordering::Greater => false,
        })
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
