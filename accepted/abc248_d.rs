use std::collections::{HashMap, VecDeque};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        q: usize,
        queries: [(Usize1, Usize1, Usize1); q],
    }
    let mut v = vec![];
    for &(l, r, x) in queries.iter() {
        v.push((l, x));
        v.push((r + 1, x));
    }
    v.sort();
    let mut v = VecDeque::from(v);
    let mut m = HashMap::new();
    let mut cnt = vec![0; n];
    for i in 0..=n {
        while v.front().map(|p| p.0) == Some(i) {
            let (_, x) = v.pop_front().unwrap();
            m.insert((i, x), cnt[x]);
        }
        if i < n {
            cnt[a[i]] += 1;
        }
    }
    for &(l, r, x) in queries.iter() {
        println!("{}", m[&(r + 1, x)] - m[&(l, x)]);
    }
}
