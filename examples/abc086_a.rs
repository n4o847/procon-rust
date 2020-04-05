use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64, b: i64,
    }
    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" });
}
