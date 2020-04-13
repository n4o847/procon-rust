use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String
    }
    let s = s.chars().collect::<Vec<_>>();
    let r = s.iter().filter(|&c| *c == 'R').count();
    let g = s.iter().filter(|&c| *c == 'G').count();
    let b = s.iter().filter(|&c| *c == 'B').count();
    let mut ans = r * g * b;
    for d in 1.. {
        if d + d >= n {
            break;
        }
        for i in 0..n {
            let j = i + d;
            let k = j + d;
            if k >= n {
                break;
            }
            if s[i] != s[j] && s[j] != s[k] && s[i] != s[k] {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}
