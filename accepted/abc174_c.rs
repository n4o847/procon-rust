use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: u64,
    }
    let mut a = 7;
    let mut m = 0;
    for i in 0..2_000_000 {
        m += a;
        m %= k;
        if m == 0 {
            println!("{}", i + 1);
            return;
        }
        a *= 10;
        a %= k;
    }
    println!("-1");
}
