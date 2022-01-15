use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64, c: i64,
        s: String,
    }
    println!("{} {}", a + b + c, s);
}
