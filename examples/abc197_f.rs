use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::collections::HashSet;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abc: [(Usize1, Usize1, char); m],
    }
    let mut g = vec![vec![]; n];
    let mut chars = HashSet::new();
    for &(a, b, c) in abc.iter() {
        g[a].push((b, c));
        g[b].push((a, c));
        chars.insert(c);
    }
    const INF: i64 = 1 << 60;
    let mut dp = vec![vec![INF; n]; n];
    dp[0][n - 1] = 0;
    let mut q = VecDeque::new();
    q.push_back((0, n - 1));
    while let Some((a, b)) = q.pop_front() {
        for &(at, ac) in g[a].iter() {
            for &(bt, bc) in g[b].iter() {
                if ac != bc {
                    continue;
                }
                if dp[at][bt] > dp[a][b] + 1 {
                    dp[at][bt] = dp[a][b] + 1;
                    q.push_back((at, bt));
                }
            }
        }
    }
    let mut ans = INF;
    for &(a, b, _) in abc.iter() {
        ans = ans.min(dp[a][b] * 2 + 1);
        ans = ans.min(dp[b][a] * 2 + 1);
    }
    for a in 0..n {
        ans = ans.min(dp[a][a] * 2);
    }
    if ans >= INF {
        ans = -1;
    }
    println!("{}", ans);
}
