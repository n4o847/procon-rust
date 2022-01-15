use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [[i64; n]; n],
    }
    let mut lo = -1;
    let mut hi = 2_000_000_000;
    while hi - lo > 1 {
        let m = (lo + hi) / 2;
        let mut s = vec![vec![0_i64; n + 1]; n + 1];
        for i in 0..n {
            for j in 0..n {
                let v = if a[i][j] <= m { 0 } else { 1 };
                s[i + 1][j + 1] = s[i][j + 1] + s[i + 1][j] - s[i][j] + v;
            }
        }
        let mut cnt = 0;
        for i in 0..(n + 1 - k) {
            for j in 0..(n + 1 - k) {
                let c = s[i + k][j + k] - s[i + k][j] - s[i][j + k] + s[i][j];
                if c <= (k * k / 2) as i64 {
                    cnt += 1;
                }
            }
        }
        if cnt > 0 {
            hi = m;
        } else {
            lo = m;
        }
    }
    println!("{}", hi);
}
