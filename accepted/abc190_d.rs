use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
    }
    let mut d = vec![];
    let mut i = 1;
    let m = n * 2;
    while i * i <= m {
        if m % i == 0 {
            d.push(i);
            if i * i < m {
                d.push(m / i);
            }
        }
        i += 1;
    }
    let mut ans = 0;
    for &l in d.iter() {
        // (2a + l - 1)
        let k = m / l;
        let b = k + 1 - l;
        if b % 2 == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
