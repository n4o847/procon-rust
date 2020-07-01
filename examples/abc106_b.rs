use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut c = 0;
    for k in 1..=n {
        if k % 2 == 0 {
            continue;
        }
        if (1..=k).filter(|l| k % l == 0).count() == 8 {
            c += 1;
        };
    }
    println!("{}", c);
}
