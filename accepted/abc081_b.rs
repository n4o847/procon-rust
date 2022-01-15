use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }
    println!(
        "{}",
        a.iter()
            .map(|x| x.trailing_zeros())
            .fold(100, std::cmp::min)
    );
}
