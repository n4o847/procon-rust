use proconio::{fastout, input};

use proconio::marker::Usize1;
use std::collections::{BTreeMap, BTreeSet};

#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        ab: [(u64, Usize1); n],
        cd: [(Usize1, Usize1); q],
    }
    let mut rt = vec![0; n];
    let mut at = vec![0; n];
    let mut kgs = BTreeMap::new();
    let mut tp = BTreeSet::new();
    for i in 0..n {
        let (a, b) = ab[i];
        rt[i] = a;
        at[i] = b;
        let kg = kgs.entry(b).or_insert_with(|| BTreeSet::new());
        kg.insert((a, i));
    }
    for (_k, kg) in kgs.iter() {
        let i = kg.iter().next_back().unwrap().1;
        tp.insert((rt[i], i));
    }
    // dbg!(&kgs, &tp);
    for (c, d) in cd {
        let kg = kgs.get_mut(&at[c]).unwrap();
        kg.remove(&(rt[c], c));
        if tp.contains(&(rt[c], c)) {
            tp.remove(&(rt[c], c));
            if let Some(&(_a, i)) = kg.iter().next_back() {
                tp.insert((rt[i], i));
            }
        }
        at[c] = d;
        let kg = kgs.entry(d).or_insert_with(|| BTreeSet::new());
        if let Some(&(a, i)) = kg.iter().next_back() {
            if rt[c] > a {
                tp.remove(&(rt[i], i));
                tp.insert((rt[c], c));
            }
        } else {
            tp.insert((rt[c], c));
        }
        kg.insert((rt[c], c));
        // dbg!(&kgs, &tp);
        let ans = tp.iter().next().unwrap().0;
        println!("{}", ans);
    }
}
