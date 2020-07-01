use proconio::{fastout, input};

use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 0;
    for a in 0..=9 {
        let mut i = 0;
        while i < n && s[i].to_digit(10) != Some(a) {
            i += 1;
        }
        if i == n {
            continue;
        }
        for b in 0..=9 {
            let mut j = i + 1;
            while j < n && s[j].to_digit(10) != Some(b) {
                j += 1;
            }
            if j == n {
                continue;
            }
            for c in 0..=9 {
                let mut k = j + 1;
                while k < n && s[k].to_digit(10) != Some(c) {
                    k += 1;
                }
                if k == n {
                    continue;
                }
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
