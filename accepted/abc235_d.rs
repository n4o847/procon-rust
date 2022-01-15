use proconio::{fastout, input};
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        a: u64,
        n: u64,
    }
    let mut visited = HashSet::new();
    let mut que = VecDeque::new();
    que.push_back((n, 0));
    visited.insert(n);
    while let Some((x, t)) = que.pop_front() {
        if x == 1 {
            println!("{}", t);
            return;
        }
        if x % a == 0 {
            let y = x / a;
            if !visited.contains(&y) {
                que.push_back((y, t + 1));
                visited.insert(y);
            }
        }
        {
            let mut h = x;
            let mut b = 1;
            while h >= 10 {
                h /= 10;
                b *= 10;
            }
            let y = (x - h * b) * 10 + h;
            if b > 1 {
                let k = x / (b / 10) % 10;
                if k == 0 {
                    continue;
                }
            }
            if !visited.contains(&y) {
                que.push_back((y, t + 1));
                visited.insert(y);
            }
        }
    }
    println!("-1");
}
