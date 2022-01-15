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
    fn mkdist(g: &Vec<Vec<usize>>, u: usize, p: usize, dist: &mut Vec<u64>) {
        for &v in g[u].iter() {
            if v == p {
                continue;
            }
            dist[v] = dist[u] + 1;
            mkdist(g, v, u, dist);
        }
    }
    let mut dist = vec![0; n];
    mkdist(&g, 0, n, &mut dist);
    let (u, _) = dist.iter().enumerate().max_by_key(|&(_, d)| d).unwrap();
    let mut dist = vec![0; n];
    mkdist(&g, u, n, &mut dist);
    let (_, d) = dist.iter().enumerate().max_by_key(|&(_, d)| d).unwrap();
    println!("{}", d + 1);
}
