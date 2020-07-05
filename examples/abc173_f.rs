use proconio::{fastout, input};

use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }
    let mut ans: i64 = 0;
    for k in 0..n {
        ans += (k + 1) as i64 * (n - k) as i64;
    }
    for (u, v) in uv {
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        ans -= (u + 1) as i64 * (n - v) as i64;
    }
    println!("{}", ans);
}
