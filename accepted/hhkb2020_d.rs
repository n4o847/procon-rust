use proconio::{fastout, input};

#[fastout]
fn main() {
    const M: u64 = 1_000_000_007;
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64, a: u64, b: u64,
        }
        if n >= a + b {
            let z = (n + 2 - a - b) * (n + 1 - a - b) / 2 % M;
            let y = z * 2 % M * (n - a + 1) % M * (n - b + 1) % M;
            let x = z * z % M;
            let ans = (y * 2 % M + M - x * 4 % M) % M;
            println!("{}", ans);
        } else {
            println!("0");
        }
    }
}
