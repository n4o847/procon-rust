use proconio::{fastout, input};

use proconio::marker::Usize1;

struct FenwickTree {
    seg: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            seg: vec![0; n + 1],
        }
    }
    fn len(&self) -> usize {
        self.seg.len() - 1
    }
    fn add(&mut self, i: usize, x: i64) {
        let mut i = i + 1;
        while i <= self.len() {
            self.seg[i] += x;
            i += i & -(i as isize) as usize;
        }
    }
    fn sum(&self, i: usize) -> i64 {
        let mut s = 0;
        let mut i = i;
        while i > 0 {
            s += self.seg[i];
            i -= i & -(i as isize) as usize;
        }
        s
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        cc: [Usize1; n],
        lr: [(Usize1, Usize1); q],
    }
    let mut ft = FenwickTree::new(n);
    let mut ls = vec![None; n];
    let mut ilr = lr.into_iter().enumerate().collect::<Vec<_>>();
    ilr.sort_by_key(|&(_i, (_l, r))| r);
    let mut ans = vec![0; q];
    let mut j = 0;
    for i in 0..n {
        let c = cc[i];
        if let Some(i) = ls[c] {
            ft.add(i, -1);
        }
        ls[c] = Some(i);
        ft.add(i, 1);
        while j < q && (ilr[j].1).1 <= i {
            ans[ilr[j].0] = ft.sum((ilr[j].1).1 + 1) - ft.sum((ilr[j].1).0);
            j += 1;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
