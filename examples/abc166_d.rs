use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i64,
    }
    for a in -200..=200 {
        for b in -200..=200 {
            if a * a * a * a * a - b * b * b * b * b == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
