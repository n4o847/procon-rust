use proconio::{fastout, input};

use proconio::marker::Chars;
use std::cmp::min;

#[fastout]
fn main() {
    const M: usize = 998244353;
    input! {
        s: Chars, k: usize,
    }
    let n = s.len();
    let k = min(k, s.len());
    let mut a = vec![0];
    for c in s {
        if c == '0' {
            a.push(0);
        }
        if c == '1' {
            if let Some(last) = a.last_mut() {
                *last += 1;
            }
        }
    }
    let mut h = vec![vec![vec![None; k + 1]; n]; a.len() + 1];
    fn f(
        i: usize,
        j: usize,
        k: usize,
        a: &Vec<usize>,
        h: &mut Vec<Vec<Vec<Option<usize>>>>,
    ) -> usize {
        if i == 0 {
            return 1;
        }
        if let Some(v) = h[i][j][k] {
            return v;
        }
        let mut s = 0;
        for l in 0..=min(k, a[i]) {
            s += f(i - 1, j + l, k - l, a, h);
            s %= M;
        }
        for l in 1..=j {
            s += f(i - 1, j - l, k, a, h);
            s %= M;
        }
        h[i][j][k] = Some(s);
        s
    }
    let ans = f(a.len() - 1, 0, k, &a, &mut h);
    println!("{}", ans);
}
