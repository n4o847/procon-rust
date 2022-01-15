use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut k: i64,
        mut a: [i64; n],
    }
    a.sort();
    a.reverse();
    a.push(0);
    let mut z = 0;
    for i in 0..n {
        let t = (a[i] - a[i + 1]) * (i as i64 + 1);
        let s = (a[i] + 1 + a[i + 1]) * t / 2;
        if k >= t {
            k -= t;
            z += s;
        } else {
            for s in ((a[i + 1] + 1)..=a[i]).rev() {
                let t = i as i64 + 1;
                if k >= t {
                    k -= t;
                    z += s * t;
                } else {
                    z += s * k;
                    // k = 0;
                    break;
                }
            }
            break;
        }
    }
    println!("{}", z);
}
