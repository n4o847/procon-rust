use proconio::{fastout, input};

use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        h: [u64; n],
    }
    let mut g = vec![true; n];
    for _ in 0..m {
        input! { a: Usize1, b: Usize1 }
        if h[a] <= h[b] {
            g[a] = false;
        }
        if h[b] <= h[a] {
            g[b] = false;
        }
    }
    let ans = g.iter().filter(|&&x| x).count();
    println!("{}", ans);
}
