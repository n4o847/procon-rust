use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
        m: usize,
    }
    // ax + by + c
    let mut a = vec![0; m + 1];
    let mut b = vec![0; m + 1];
    let mut c = vec![0; m + 1];
    // dx + ey + f
    let mut d = vec![0; m + 1];
    let mut e = vec![0; m + 1];
    let mut f = vec![0; m + 1];
    a[0] = 1;
    e[0] = 1;
    for i in 0..m {
        input! { op: usize }
        match op {
            1 => {
                a[i + 1] = d[i];
                b[i + 1] = e[i];
                c[i + 1] = f[i];
                d[i + 1] = -a[i];
                e[i + 1] = -b[i];
                f[i + 1] = -c[i];
            }
            2 => {
                a[i + 1] = -d[i];
                b[i + 1] = -e[i];
                c[i + 1] = -f[i];
                d[i + 1] = a[i];
                e[i + 1] = b[i];
                f[i + 1] = c[i];
            }
            3 => {
                input! { p: i64 }
                a[i + 1] = -a[i];
                b[i + 1] = -b[i];
                c[i + 1] = -c[i] + 2 * p;
                d[i + 1] = d[i];
                e[i + 1] = e[i];
                f[i + 1] = f[i];
            }
            4 => {
                input! { p: i64 }
                a[i + 1] = a[i];
                b[i + 1] = b[i];
                c[i + 1] = c[i];
                d[i + 1] = -d[i];
                e[i + 1] = -e[i];
                f[i + 1] = -f[i] + 2 * p;
            }
            _ => {}
        }
    }
    input! {
        q: usize,
        ab: [(usize, Usize1); q],
    }
    for &(g, h) in ab.iter() {
        println!(
            "{} {}",
            a[g] * xy[h].0 + b[g] * xy[h].1 + c[g],
            d[g] * xy[h].0 + e[g] * xy[h].1 + f[g]
        );
    }
}
