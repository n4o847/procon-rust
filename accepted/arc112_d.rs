use proconio::marker::Chars;
use proconio::{fastout, input};
use std::{
    cmp::{max, min},
    collections::HashSet,
    mem::swap,
};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let mut ni = vec![1_i64; h];
    let mut nj = vec![1_i64; w];
    ni[0] = 0;
    ni[h - 1] = 0;
    nj[0] = 0;
    nj[w - 1] = 0;
    let mut uf = UF::new(h + w);
    uf.unite(0, h - 1);
    uf.unite(0, h);
    uf.unite(0, h + w - 1);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                ni[i] = 0;
                nj[j] = 0;
                uf.unite(i, h + j);
            }
        }
    }
    let mut s = HashSet::new();
    for i in 0..h {
        let p = uf.find(i);
        if p != i {
            s.insert(p);
        }
    }
    for j in 0..w {
        let p = uf.find(h + j);
        if p != h + j {
            s.insert(p);
        }
    }
    let nh: i64 = ni.iter().sum();
    let nw: i64 = nj.iter().sum();
    let ans = min(nh, nw) + s.len() as i64 - 1;
    println!("{}", ans);
}

struct UF {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> UF {
        UF {
            par: (0..n).collect(),
            size: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let p = self.find(self.par[x]);
            self.par[x] = p;
            p
        }
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }
        if self.size[x] < self.size[y] {
            swap(&mut x, &mut y);
        }
        self.par[y] = x;
        self.size[x] += self.size[y];
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
