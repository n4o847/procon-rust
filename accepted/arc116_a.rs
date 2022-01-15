use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64,
        }
        if n % 4 == 0 {
            println!("Even");
        } else if n % 2 == 0 {
            println!("Same");
        } else {
            println!("Odd");
        }
    }
}
