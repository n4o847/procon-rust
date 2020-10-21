use proconio::{fastout, input};

use proconio::marker::Chars;

fn zalgo(s: &[char]) -> usize {
    let n = s.len();
    let mut z = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        if i + z[i - j] < j + z[j] {
            z[i] = z[i - j];
        } else {
            let mut k = std::cmp::max(i, j + z[j]) - i;
            while i + k < n && s[k] == s[i + k] {
                k += 1;
            }
            z[i] = k;
            j = i;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if z[i] <= i {
            ans = std::cmp::max(ans, z[i]);
        }
    }
    ans
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 0;
    for i in 0..n {
        ans = std::cmp::max(ans, zalgo(&s[i..]));
    }
    println!("{}", ans);
}
