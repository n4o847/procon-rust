use proconio::{fastout, input};

#[fastout]
fn main() {
    const M: i64 = 998244353;
    input! {
        a: i64, b: i64, c: i64, d: i64,
    }
    let mut h = vec![vec![None; d as usize + 1]; c as usize + 1];
    fn f(c: i64, d: i64, a: i64, b: i64, h: &mut Vec<Vec<Option<i64>>>) -> i64 {
        if let Some(v) = h[c as usize][d as usize] {
            return v;
        }
        let v = if c == a && d == b {
            1
        } else if c == a {
            f(c, d - 1, a, b, h) * c
        } else if d == b {
            f(c - 1, d, a, b, h) * d
        } else {
            f(c, d - 1, a, b, h) * c + f(c - 1, d, a, b, h) * d
                - f(c - 1, d - 1, a, b, h) * ((c - 1) * (d - 1))
        } % M;
        let v = (v + M) % M;
        h[c as usize][d as usize] = Some(v);
        v
    }
    let ans = f(c, d, a, b, &mut h);
    println!("{}", ans);
}
