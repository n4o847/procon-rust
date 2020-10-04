use proconio::{fastout, input};

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut xy: [(u64, u64); n],
        mut aa: [u64; m],
    }
    xy.sort();
    aa.sort();
    let mut xy = VecDeque::from(xy);
    let mut c = 0;
    let mut h = BinaryHeap::new();
    for &a in aa.iter() {
        while let Some(&(x, y)) = xy.front() {
            if x <= a {
                h.push(Reverse((y, x)));
                xy.pop_front();
            } else {
                break;
            }
        }
        while let Some(Reverse((y, _x))) = h.pop() {
            if a <= y {
                c += 1;
                break;
            }
        }
    }
    println!("{}", c);
}
