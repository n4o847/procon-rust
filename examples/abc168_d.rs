use proconio::{fastout, input};

use proconio::marker::Usize1;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! {
            a: Usize1, b: Usize1,
        }
        g[a].push(b);
        g[b].push(a);
    }
    let mut que = VecDeque::new();
    que.push_back(0);
    let mut ans = vec![None; n];
    ans[0] = Some(0);
    while let Some(s) = que.pop_front() {
        for &t in g[s].iter() {
            if ans[t] == None {
                ans[t] = Some(s);
                que.push_back(t);
            }
        }
    }
    println!("Yes");
    for a in ans.iter().skip(1) {
        println!("{}", a.unwrap() + 1);
    }
}
