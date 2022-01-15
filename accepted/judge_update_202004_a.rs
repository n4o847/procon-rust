use num::clamp;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: i64, l: i64, r: i64
    }
    println!("{}", clamp(s, l, r));
}
