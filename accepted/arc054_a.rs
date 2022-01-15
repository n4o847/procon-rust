use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u64, x: u64, y: u64, s: u64, d: u64,
    }
    let a = if s <= d { d - s } else { l - s + d };
    let b = if s < d { s + l - d } else { s - d };
    let p = a as f64 / (x + y) as f64;
    if y > x {
        let q = b as f64 / (y - x) as f64;
        println!("{}", if p < q { p } else { q });
    } else {
        println!("{}", p);
    }
}
