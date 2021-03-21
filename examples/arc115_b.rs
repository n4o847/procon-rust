use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [[i64; n]; n],
    }
    let mut d = vec![vec![0; n]; n - 1];
    for i in 0..(n - 1) {
        for j in 0..n {
            d[i][j] = c[i + 1][j] - c[i][j];
        }
    }
    for i in 0..(n - 1) {
        let a = d[i][0];
        for j in 1..n {
            if d[i][j] != a {
                println!("No");
                return;
            }
        }
    }
    let mut m = 0;
    let mut s = 0;
    for i in 0..(n - 1) {
        s += d[i][0];
        m = m.min(s);
    }
    let mut a = vec![0; n];
    a[0] = -m;
    for i in 0..(n - 1) {
        a[i + 1] = a[i] + d[i][0];
    }
    let mut b = vec![0; n];
    b[0] = c[0][0] - a[0];
    for j in 1..n {
        b[j] = c[0][j] - a[0];
        if b[j] < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
    for i in 0..n {
        print!("{} ", b[i]);
    }
    println!();
}
