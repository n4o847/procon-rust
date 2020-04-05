use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        mut a: [i64; n],
    }
    a.sort();
    println!("{}", a.iter().fold(0, |x, y| y - x));
}
