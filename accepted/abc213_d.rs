use proconio::{fastout, input, marker::Usize1};

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
    for u in 0..n {
        g[u].sort();
    }
    fn f(g: &[Vec<usize>], u: usize, p: usize) {
        print!("{} ", u + 1);
        for &v in g[u].iter() {
            if v == p {
                continue;
            }
            f(g, v, u);
            print!("{} ", u + 1);
        }
    }
    f(&mut g, 0, n);
    println!();
}
