use proconio::{fastout, input};

use proconio::marker::Bytes;

#[fastout]
fn main() {
    input! {
        s: Bytes
    }
    let mut a = vec![0; 2019];
    a[0] = 1;
    let mut o = 0;
    let mut e = 1;
    let mut ans = 0 as i64;
    for b in s.iter() {
        let d = (b - 48) as i64;
        o = ((o - d * e) % 2019 + 2019) % 2019;
        ans += a[o as usize];
        a[o as usize] += 1;
        e = e * 202 % 2019;
    }
    println!("{}", ans);
}
