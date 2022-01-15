use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    const M: u64 = 1_000_000_007;
    let mut a = 1;
    let mut b = 1;
    let mut c = 1;
    for _ in 0..n {
        a = a * 10 % M;
        b = b * 9 % M;
        c = c * 8 % M;
    }
    let ans = (a + (M - b) * 2 + c) % M;
    println!("{}", ans);
}
