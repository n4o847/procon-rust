use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64, b: i64, c: i64,
    }
    let x = b - a;
    let y = c - b;
    let ans = if x > y {
        x - y
    } else {
        (y - x) / 2 + (y - x) % 2 * 2
    };
    println!("{}", ans);
}
