use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut b = vec![];
    let mut c = vec![];
    b.push(a[0]);
    c.push(0);
    for i in 1..n {
        if a[i] < a[i - 1] {
            b.push(b[i - 1]);
            c.push(c[i - 1] + a[i - 1] - a[i]);
        } else {
            b.push(b[i - 1] + a[i] - a[i - 1]);
            c.push(c[i - 1]);
        }
    }
    let mut s = vec![0];
    let mut t = vec![0];
    for i in 0..n {
        s.push(s[i] + b[i]);
        t.push(t[i] + c[i]);
    }
    let mut ans = std::i64::MAX;
    let f = |i, x| x * i as i64 - s[i] + (s[n] - s[i]) - x * (n - i) as i64;
    let g = |i, x| x * i as i64 - t[i] + (t[n] - t[i]) - x * (n - i) as i64;
    for i in 0..n {
        let x = b[i];
        let j = match c.binary_search(&x) {
            Ok(j) => j,
            Err(j) => j,
        };
        ans = ans.min(f(i, x) + g(j, x));
    }
    for j in 0..n {
        let x = c[j];
        let i = match b.binary_search(&x) {
            Ok(i) => i,
            Err(i) => i,
        };
        ans = ans.min(f(i, x) + g(j, x));
    }
    println!("{}", ans);
}
