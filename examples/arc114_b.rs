use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        f: [Usize1; n],
    }
    const M: usize = 998244353;
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        uf.unite(i, f[i]);
    }
    let mut s = HashSet::new();
    for i in 0..n {
        s.insert(uf.find(i));
    }
    let mut a = 1;
    for _ in 0..s.len() {
        a = a * 2 % M;
    }
    println!("{}", (a + M - 1) % M);
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    #[allow(dead_code)]
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![0; n],
        }
    }
    #[allow(dead_code)]
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
            p
        }
    }
    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        self.size[x] += self.size[y];
    }
    #[allow(dead_code)]
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
