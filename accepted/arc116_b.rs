use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    const M: u64 = 998244353;
    let mut ans = 0;
    a.sort();
    let mut t = 0;
    for &x in a.iter() {
        ans = (ans + t * x % M + x * x % M) % M;
        t = (t * 2 + x) % M;
        // eprintln!("{} {}", t, ans);
    }
    println!("{}", ans);
}
