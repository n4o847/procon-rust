use proconio::{fastout, input};

use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    let mut a = a;
    for _ in 0..k {
        let mut s = vec![0isize; n + 1];
        for i in 0..n {
            s[max(a[i], i) - a[i]] += 1;
            s[min(n, i + a[i] + 1)] -= 1;
        }
        let mut b = vec![0; n];
        let mut t = 0;
        let mut m = n;
        for i in 0..n {
            t += s[i];
            b[i] = t as usize;
            m = min(m, t as usize);
        }
        a = b;
        if m == n {
            break;
        }
    }
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}
