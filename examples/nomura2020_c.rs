use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n + 1],
    }
    let mut b = vec![];
    let mut m = 1;
    for i in 0..n + 1 {
        if a[i] <= m {
            m -= a[i];
            b.push(m);
        } else {
            println!("-1");
            return;
        }
        m *= 2;
    }
    let mut l = 0;
    for i in (0..n + 1).rev() {
        if b[i] > l {
            b[i] = l;
        } else {
            break;
        }
        l = b[i] + a[i];
    }
    let mut ans = 0;
    for i in 0..n + 1 {
        ans += a[i] + b[i];
    }
    println!("{}", ans);
}
