use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: Chars,
        m: u64,
    }
    let x: Vec<_> = x
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    fn ok(x: &Vec<u64>, m: u64, b: u64) -> bool {
        let mut s = 0_u64;
        for &a in x.iter() {
            s = s.saturating_mul(b).saturating_add(a);
            if s > m {
                return false;
            }
        }
        true
    }
    let d = x.iter().max().unwrap();
    if !ok(&x, m, d + 1) {
        println!("0");
        return;
    }
    if x.len() == 1 {
        println!("1");
        return;
    }
    let mut lo = d + 1;
    let mut hi = std::u64::MAX / 2;
    while lo + 1 < hi {
        let b = (lo + hi) / 2;
        if ok(&x, m, b) {
            lo = b;
        } else {
            hi = b;
        }
    }
    let ans = lo - d;
    println!("{}", ans);
}
