use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        mut x: Chars,
    }
    s.reverse();
    x.reverse();
    let mut a = vec![false; 7];
    a[0] = true;
    let mut b = 1 as usize;
    for i in 0..n {
        let d = s[i].to_digit(10).unwrap() as usize;
        if x[i] == 'T' {
            let mut c = vec![false; 7];
            for k in 0..7 {
                c[k] |= a[k];
                c[k] |= a[(k + d * b) % 7];
            }
            a = c;
        } else {
            let mut c = vec![true; 7];
            for k in 0..7 {
                c[k] &= a[k];
                c[k] &= a[(k + d * b) % 7];
            }
            a = c;
        }
        b = (b * 3) % 7;
    }
    if a[0] {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
