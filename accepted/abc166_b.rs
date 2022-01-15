use proconio::{fastout, input};

use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
    }
    let mut b = vec![0; n];
    for _ in 0..k {
        input! {
            d: usize,
            a: [Usize1; d],
        }
        for &x in a.iter() {
            b[x] += 1;
        }
    }
    let ans = b.iter().filter(|&&x| x == 0).count();
    println!("{}", ans);
}
