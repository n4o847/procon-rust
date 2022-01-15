use proconio::{fastout, input};

use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        a: usize, b: usize,
        s: Chars,
    }
    if a <= s.len() && s.len() <= b {
        println!("YES");
    } else {
        println!("NO");
    }
}
