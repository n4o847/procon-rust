use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, q: usize,
        abc: [(Usize1, Usize1, u64); m],
        uvw: [(Usize1, Usize1, u64); q],
    }
    let mut edges = vec![];
    for &(a, b, c) in abc.iter() {
        edges.push((c, a, b, None));
    }
    for (i, &(u, v, w)) in uvw.iter().enumerate() {
        edges.push((w, u, v, Some(i)));
    }
    edges.sort();
    let mut ans = vec![false; q];
    let mut uf = UnionFind::new(n);
    for &(c, a, b, i) in edges.iter() {
        match i {
            None => {
                if !uf.same(a, b) {
                    uf.unite(a, b);
                }
            }
            Some(i) => {
                if !uf.same(a, b) {
                    ans[i] = true;
                }
            }
        }
    }
    for &a in ans.iter() {
        println!("{}", if a { "Yes" } else { "No" });
    }
}

use union_find::*;
mod union_find {
    pub struct UnionFind {
        parent: Vec<usize>,
        size: Vec<usize>,
    }
    impl UnionFind {
        #[allow(dead_code)]
        pub fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
                size: vec![1; n],
            }
        }
        #[allow(dead_code)]
        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] == x {
                x
            } else {
                let p = self.find(self.parent[x]);
                self.parent[x] = p;
                p
            }
        }
        #[allow(dead_code)]
        pub fn unite(&mut self, x: usize, y: usize) {
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
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
        #[allow(dead_code)]
        pub fn size(&mut self, x: usize) -> usize {
            let x = self.find(x);
            self.size[x]
        }
        #[allow(dead_code)]
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let n = self.parent.len();
            let mut leader_buf = vec![0; n];
            let mut group_size = vec![0; n];
            for i in 0..n {
                leader_buf[i] = self.find(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); n];
            for i in 0..n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..n {
                result[leader_buf[i]].push(i);
            }
            result.into_iter().filter(|v| !v.is_empty()).collect()
        }
    }
}
