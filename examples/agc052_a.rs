use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s0: Chars,
            s1: Chars,
            s2: Chars,
        }
        let mut res = String::new();
        for i in 0..(2 * n + 1) {
            if i < n {
                res.push('0');
            } else if i < 2 * n {
                res.push('1');
            } else {
                res.push('0');
            }
        }
        println!("{}", res);
    }
}
