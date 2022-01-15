use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut s = 0;
    for &x in a.iter() {
        s ^= x;
    }
    for &x in a.iter() {
        print!("{} ", s ^ x);
    }
}
