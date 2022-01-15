use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut a = 0;
    for k in 1..=n {
        a += (n / k) * (n / k + 1) / 2 * k;
    }
    println!("{}", a);
}
