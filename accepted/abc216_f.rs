use proconio::{fastout, input};

#[fastout]
fn main() {
    const M: u64 = 998244353;
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }
    let mut ab: Vec<_> = a.into_iter().zip(b.into_iter()).collect();
    ab.sort();
    let (a, b): (Vec<_>, Vec<_>) = ab.into_iter().unzip();
    let m = *a.iter().max().unwrap() as usize;
    let mut c = vec![vec![0; m + 1]; n + 1];
    let mut s = vec![vec![0; m + 1]; n + 1];
    c[0][0] = 1;
    for i in 0..n {
        for s in 0..c[i].len() {
            c[i + 1][s] += c[i][s];
            c[i + 1][s] %= M;
            let t = s + b[i] as usize;
            if t < c[i + 1].len() {
                c[i + 1][t] += c[i][s];
                c[i + 1][t] %= M;
            }
        }
    }
    for i in 0..n {
        s[i][0] = c[i][0];
        for j in 1..c[i].len() {
            s[i][j] = (s[i][j - 1] + c[i][j]) % M;
        }
    }
    let mut z = 0;
    for i in 0..n {
        if a[i] >= b[i] {
            let t = s[i][(a[i] - b[i]) as usize];
            z += t;
            z %= M;
        }
    }
    println!("{}", z);
}
