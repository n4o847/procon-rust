use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    }
    let mut a: Vec<_> = (0..=9).collect();
    let mut b: Vec<_> = (0..=9).collect();
    let mut c = vec![k; 11];
    for i in 0..4 {
        let d = s[i].to_digit(10).unwrap() as usize;
        a[d] *= 10;
        c[d] -= 1;
        let d = t[i].to_digit(10).unwrap() as usize;
        b[d] *= 10;
        c[d] -= 1;
    }
    let sa: u64 = a.iter().sum();
    let sb: u64 = b.iter().sum();
    let mut sum = 0;
    let mut win = 0;
    for x in 1..=9 {
        if c[x] == 0 {
            continue;
        }
        let cx = c[x];
        c[x] -= 1;
        for y in 1..=9 {
            if c[y] == 0 {
                continue;
            }
            let cy = c[y];
            c[y] -= 1;
            let pat = cx * cy;
            sum += pat;
            if sa + a[x] * 9 > sb + b[y] * 9 {
                win += pat;
            }
            c[y] += 1;
        }
        c[x] += 1;
    }
    let ans = win as f64 / sum as f64;
    println!("{}", ans);
}
