use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a].push(b);
        g[b].push(a);
    }
    fn f(
        g: &Vec<Vec<usize>>,
        c: &Vec<usize>,
        d: &mut Vec<usize>,
        u: usize,
        p: usize,
        r: &mut Vec<bool>,
    ) {
        if d[c[u]] == 0 {
            r[u] = true;
        }
        d[c[u]] += 1;
        for &v in g[u].iter() {
            if v == p {
                continue;
            }
            f(g, c, d, v, u, r);
        }
        d[c[u]] -= 1;
    }
    let mut r = vec![false; n];
    let cn = *c.iter().max().unwrap() as usize + 1;
    f(&g, &c, &mut vec![0; cn], 0, n, &mut r);
    for i in 0..n {
        if r[i] {
            println!("{}", i + 1);
        }
    }
}
