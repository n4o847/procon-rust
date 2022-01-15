use proconio::{fastout, input};
use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u128, a: u128, b: u128, c: u128, d: u128,
        }
        let mut que = BinaryHeap::new();
        que.push(Reverse((0, n)));
        let mut used = BTreeSet::new();
        while let Some(Reverse((z, m))) = que.pop() {
            if used.contains(&m) {
                continue;
            }
            if m == 0 {
                println!("{}", z);
                break;
            }
            if m == 1 {
                println!("{}", z + d);
                break;
            }
            if m % 2 == 0 {
                que.push(Reverse((z + a, m / 2)));
            } else {
                que.push(Reverse((z + min(a + d, (m - m / 2 - 1) * d), m / 2 + 1)));
                que.push(Reverse((z + min(a + d, (m - m / 2) * d), m / 2)));
            }
            if m % 3 == 0 {
                que.push(Reverse((z + min(b, (m - m / 3) * d), m / 3)));
            } else {
                que.push(Reverse((
                    z + min(b + d * (3 - m % 3), (m - m / 3 - 1) * d),
                    m / 3 + 1,
                )));
                que.push(Reverse((z + min(b + d * (m % 3), (m - m / 3) * d), m / 3)));
            }
            if m % 5 == 0 {
                que.push(Reverse((z + min(c, (m - m / 5) * d), m / 5)));
            } else {
                que.push(Reverse((
                    z + min(c + d * (5 - m % 5), (m - m / 5 - 1) * d),
                    m / 5 + 1,
                )));
                que.push(Reverse((z + min(c + d * (m % 5), (m - m / 5) * d), m / 5)));
            }
            used.insert(m);
        }
    }
}
