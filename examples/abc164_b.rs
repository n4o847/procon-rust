use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64, c: u64, d: u64
    }
    let ceil = |x, y| x / y + (if x % y == 0 { 0 } else { 1 });
    let ans = if ceil(c, b) <= ceil(a, d) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
