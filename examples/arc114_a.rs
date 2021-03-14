use num::integer::gcd;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [u64; n],
    }
    let ps = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut ans = vec![];
    for b in 0..(1 << 15) {
        let mut y = 1;
        for i in 0..15 {
            if (b >> i) & 1 == 1 {
                y *= ps[i];
            }
        }
        if x.iter().all(|&x| gcd(x, y) != 1) {
            ans.push(y);
        }
    }
    println!("{}", ans.iter().min().unwrap());
}
