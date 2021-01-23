use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut ans = 0;
    for l in 0..n {
        let mut x = a[l];
        for r in l..n {
            x = std::cmp::min(x, a[r]);
            ans = std::cmp::max(ans, (r - l + 1) as u64 * x);
        }
    }
    println!("{}", ans);
}
