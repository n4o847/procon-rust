use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut ans = a.iter().fold(0, |acc, x| acc | x);
    for s in 0..(1 << (n - 1)) {
        let mut v = 0;
        let mut u = 0;
        for i in 0..n {
            u |= a[i];
            if (s >> i) & 1 == 1 || i == n - 1 {
                v ^= u;
                u = 0;
            }
        }
        ans = ans.min(v);
    }
    println!("{}", ans);
}
