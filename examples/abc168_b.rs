use proconio::{fastout, input};

use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        k: usize,
        s: Chars,
    }
    if s.len() <= k {
        println!("{}", s.iter().collect::<String>());
    } else {
        println!("{}...", s.iter().take(k).collect::<String>());
    }
}
