use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut p = n - 1;
    for i in 1..n {
        if a[i - 1] > a[i] {
            p = i - 1;
            break;
        }
    }
    let m = a[p];
    for x in a.iter().filter(|&&x| x != m) {
        print!("{} ", x);
    }
}
