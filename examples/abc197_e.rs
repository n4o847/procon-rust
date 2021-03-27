use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xc: [(i64, Usize1); n],
    }
    let mut pos: Vec<Option<(i64, i64)>> = vec![None; n];
    for &(x, c) in xc.iter() {
        if let Some((a, b)) = pos[c] {
            pos[c] = Some((a.min(x), b.max(x)));
        } else {
            pos[c] = Some((x, x));
        }
    }
    let mut ls: i64 = 0;
    let mut lx: i64 = 0;
    let mut rs: i64 = 0;
    let mut rx: i64 = 0;
    for item in pos.into_iter() {
        if let Some((a, b)) = item {
            let lsn =
                (ls + (b - lx).abs() + (a - b).abs()).min(rs + (b - rx).abs() + (a - b).abs());
            let rsn =
                (ls + (a - lx).abs() + (b - a).abs()).min(rs + (a - rx).abs() + (b - a).abs());
            ls = lsn;
            rs = rsn;
            lx = a;
            rx = b;
        }
    }
    let ans = (ls + lx.abs()).min(rs + rx.abs());
    println!("{}", ans);
}
