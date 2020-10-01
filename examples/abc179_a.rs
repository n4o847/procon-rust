use proconio::{fastout, input};

use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    print!("{}", s.iter().collect::<String>());
    if s.last() == Some(&'s') {
        println!("es");
    } else {
        println!("s");
    }
}
