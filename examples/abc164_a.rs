use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: u64, w: u64
    }
    println!("{}", if w >= s { "unsafe" } else { "safe" });
}
