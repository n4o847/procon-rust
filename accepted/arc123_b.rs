use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        mut b: [u64; n],
        mut c: [u64; n],
    }
    a.sort();
    b.sort();
    c.sort();
    let mut i = 0;
    let mut k = 0;
    let mut t = 0;
    for j in 0..n {
        if a[i] >= b[j] {
            continue;
        }
        while k < n && b[j] >= c[k] {
            k += 1;
        }
        if k == n {
            break;
        }
        i += 1;
        k += 1;
        t += 1;
    }
    println!("{}", t);
}
