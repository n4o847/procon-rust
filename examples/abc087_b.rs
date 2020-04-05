use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        x: i64,
    }
    let mut cnt = 0;
    for p in 0..=a {
        for q in 0..=b {
            for r in 0..=c {
                if 500 * p + 100 * q + 50 * r == x {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
