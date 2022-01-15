use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }
    let (mut a, mut b): (Vec<_>, Vec<_>) = ab.into_iter().unzip();
    a.sort();
    b.sort();
    let ans = if n % 2 == 1 {
        b[n / 2] - a[n / 2] + 1
    } else {
        (b[n / 2 - 1] + b[n / 2]) - (a[n / 2 - 1] + a[n / 2]) + 1
    };
    println!("{}", ans);
}
