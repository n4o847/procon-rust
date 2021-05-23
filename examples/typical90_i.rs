use ordered_float::NotNan;
use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut ans = NotNan::new(0.0).unwrap();
    for i in 0..n {
        let mut ps = vec![];
        for j in 0..n {
            if i == j {
                continue;
            }
            let x = (xy[j].0 - xy[i].0) as f64;
            let y = (xy[j].1 - xy[i].1) as f64;
            ps.push(NotNan::new(y.atan2(x)).unwrap());
        }
        ps.sort();
        for j in 0..(n - 1) {
            let pk = if ps[j] > NotNan::new(0.0).unwrap() {
                ps[j] - PI
            } else {
                ps[j] + PI
            };
            let a = match ps.binary_search(&pk) {
                Ok(_) => PI,
                Err(k) => {
                    let l = if k == 0 { ps[k] } else { ps[k - 1] };
                    let r = if k < n - 1 { ps[k] } else { ps[k - 1] };
                    PI - (pk - l).abs().min((r - pk).abs())
                }
            };
            ans = ans.max(NotNan::new(a * 180.0 / PI).unwrap());
        }
    }
    println!("{}", ans);
}
