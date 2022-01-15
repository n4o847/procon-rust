use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![0; n + 1];
    for x in 1..=100 {
        for y in 1..=100 {
            for z in 1..=100 {
                let r = x * x + y * y + z * z + x * y + y * z + z * x;
                if r <= n {
                    a[r] += 1;
                }
            }
        }
    }
    for i in 1..=n {
        println!("{}", a[i]);
    }
}
