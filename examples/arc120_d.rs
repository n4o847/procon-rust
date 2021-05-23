use std::collections::{BTreeSet, VecDeque};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; 2*n],
    }
    let mut ai = vec![];
    for i in 0..(2 * n) {
        ai.push((a[i], i));
    }
    ai.sort();
    let mut g = BTreeSet::new();
    for i in n..(2 * n) {
        g.insert(ai[i]);
    }
    let mut t = VecDeque::new();
    let mut s = vec!['_'; 2 * n];
    for i in 0..(2 * n) {
        if t.len() == 0 {
            t.push_back(i);
        } else {
            let j = *t.back().unwrap();
            let gi = g.contains(&(a[i], i));
            let gj = g.contains(&(a[j], j));
            if gi != gj {
                t.pop_back();
                s[j] = '(';
                s[i] = ')';
            } else {
                t.push_back(i);
            }
        }
    }
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
