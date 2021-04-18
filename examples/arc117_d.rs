use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a].push(b);
        g[b].push(a);
    }
    fn farthest(g: &Vec<Vec<usize>>, u: usize, p: usize) -> (u64, usize) {
        let mut res = (0, u);
        for &v in g[u].iter() {
            if v == p {
                continue;
            }
            let a = farthest(g, v, u);
            if a.0 + 1 > res.0 {
                res = (a.0 + 1, a.1);
            }
        }
        res
    }
    let (_, p) = farthest(&g, 0, n);
    let (_, q) = farthest(&g, p, n);
    // eprintln!("{} - {}", p, q);
    let mut dist_q = vec![0; n];
    fn make_dist(g: &Vec<Vec<usize>>, u: usize, p: usize, d: u64, dist: &mut Vec<u64>) {
        dist[u] = d;
        for &v in g[u].iter() {
            if v == p {
                continue;
            }
            make_dist(g, v, u, d + 1, dist);
        }
    }
    make_dist(&g, q, n, 0, &mut dist_q);
    let mut ans = vec![std::u64::MAX; n];
    fn dfs(
        g: &Vec<Vec<usize>>,
        u: usize,
        p: usize,
        mut e: u64,
        dist_q: &Vec<u64>,
        ans: &mut Vec<u64>,
    ) -> u64 {
        ans[u] = ans[u].min(e);
        e += 1;
        let mut a: Vec<_> = g[u].iter().cloned().collect();
        a.sort_by_key(|&v| dist_q[v]);
        a.reverse();
        for &v in a.iter() {
            if v == p {
                continue;
            }
            e = dfs(g, v, u, e, dist_q, ans);
        }
        e + 1
    }
    let _ = dfs(&g, p, n, 1, &dist_q, &mut ans);
    for &e in ans.iter() {
        print!("{} ", e);
    }
}
