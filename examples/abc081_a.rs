use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: i64,
    }
    println!("{}", s % 9);
}
