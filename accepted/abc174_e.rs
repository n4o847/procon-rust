use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: u64,
        aa: [u64; n],
    }
    let mut ok = *aa.iter().max().unwrap();
    let mut ng = 0;
    while ng + 1 < ok {
        let x = (ng + ok) / 2;
        let mut s = 0;
        for &a in aa.iter() {
            s += a / x + (if a % x == 0 { 0 } else { 1 }) - 1;
        }
        if s <= k {
            ok = x;
        } else {
            ng = x;
        }
    }
    println!("{}", ok);
}
