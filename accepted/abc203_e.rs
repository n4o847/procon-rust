use std::collections::BTreeSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut xy: [(usize, usize); m],
    }
    xy.sort();
    let mut l = 0;
    let mut a = vec![];
    for &(x, y) in xy.iter() {
        if l != x {
            a.push(vec![]);
            l = x;
        }
        a.last_mut().unwrap().push(y);
    }
    let mut s = BTreeSet::new();
    s.insert(n);
    for l in a.iter() {
        let mut c = vec![];
        for &y in l.iter() {
            let p = if y == 0 { false } else { s.contains(&(y - 1)) };
            let r = if y == 2 * n {
                false
            } else {
                s.contains(&(y + 1))
            };
            if p || r {
                c.push(1);
            } else {
                c.push(0);
            }
        }
        for i in 0..l.len() {
            let y = l[i];
            if c[i] == 1 {
                s.insert(y);
            } else {
                s.remove(&y);
            }
        }
    }
    let ans: usize = s.len();
    println!("{}", ans);
}
