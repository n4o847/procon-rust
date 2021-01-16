use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut a: [u64; n],
    }
    a.sort();

    let mut c = vec![0; n];
    for &x in a.iter() {
        c[x as usize] += 1;
    }

    let mut s = 0;
    let mut m = k;
    for i in 0..n {
        if c[i] < m {
            s += (m - c[i]) * i;
            m = c[i];
        }
    }
    println!("{}", s);
}
