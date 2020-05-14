use proconio::{fastout, input};

use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }
    let mut p = vec![];
    for s in ss {
        let mut r = 0;
        let mut m = 0;
        for c in s {
            match c {
                '(' => {
                    r -= 1;
                }
                ')' => {
                    r += 1;
                    if r > m {
                        m = r
                    }
                }
                _ => {}
            }
        }
        p.push((m, m - r));
    }
    let (mut first, mut second): (Vec<(i32, i32)>, Vec<(i32, i32)>) =
        p.iter().partition(|&(a, b)| b - a >= 0);
    first.sort_by_key(|&(a, _)| a);
    second.sort_by_key(|&(_, b)| std::cmp::Reverse(b));
    let mut r = 0;
    for &(a, b) in first.iter().chain(second.iter()) {
        r -= a;
        if r < 0 {
            println!("No");
            return;
        }
        r += b;
    }
    if r == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
