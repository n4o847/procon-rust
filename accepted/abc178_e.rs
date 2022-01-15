use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }
    xy.sort();
    let mut a = vec![];
    let mut b = vec![];
    for &(x, y) in xy.iter() {
        a.push(x + y);
        b.push(x - y);
    }
    let a = a.iter().max().unwrap() - a.iter().min().unwrap();
    let b = b.iter().max().unwrap() - b.iter().min().unwrap();
    println!("{}", std::cmp::max(a, b));
}
