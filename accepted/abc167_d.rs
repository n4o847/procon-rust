use proconio::{fastout, input};

use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,  k: u64,
        a: [Usize1; n],
    }
    let mut v = vec![std::u64::MAX; n];
    let mut c = 0;
    let mut pos = 0;
    v[pos] = 0;
    loop {
        pos = a[pos];
        c += 1;
        if c == k {
            println!("{}", pos + 1);
            return;
        }
        if v[pos] < std::u64::MAX {
            break;
        }
        v[pos] = c;
    }
    let t = v[pos] + (k - v[pos]) % (c - v[pos]);
    for i in 0..n {
        if t == v[i] {
            println!("{}", i + 1);
        }
    }
}
