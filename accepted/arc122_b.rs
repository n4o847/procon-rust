use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    let a: Vec<_> = a.into_iter().map(|a| a as f64).collect();
    let m = a.iter().sum::<f64>() / n as f64;
    let mut ans = m;
    let mut s = 0.0;
    for i in 0..n {
        let x = a[i] / 2.0;
        s += a[i];
        ans = ans.min(x + m - (s + x * 2.0 * (n - 1 - i) as f64) / n as f64);
    }
    println!("{}", ans);
}
