use proconio::{fastout, input};

use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n - 1]
    }
    let mut b = vec![0; n];
    for x in a.iter() {
        b[*x] += 1;
    }
    for i in 0..n {
        println!("{}", b[i]);
    }
}
