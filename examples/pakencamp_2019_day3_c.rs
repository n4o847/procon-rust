use proconio::{fastout, input};

use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [[u64; m]; n],
    }
    let mut ans = 0;
    for t1 in 0..m {
        for t2 in 0..t1 {
            let mut sum = 0;
            for i in 0..n {
                sum += max(a[i][t1], a[i][t2]);
            }
            ans = max(ans, sum);
        }
    }
    print!("{}", ans);
}
