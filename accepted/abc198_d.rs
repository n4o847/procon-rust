use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        mut a: Chars,
        mut b: Chars,
        mut c: Chars,
    }
    let mut s = BTreeSet::new();
    for &x in a.iter() {
        s.insert(x);
    }
    for &x in b.iter() {
        s.insert(x);
    }
    for &x in c.iter() {
        s.insert(x);
    }
    if s.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }
    let s: Vec<_> = s.iter().collect();

    fn convert(a: &Vec<char>, ps: &Vec<(&&char, &u64)>) -> u64 {
        let mut val = 0;
        for &x in a.iter() {
            let d = ps.iter().find(|&&(ch, _)| **ch == x).unwrap().1;
            val = val * 10 + d;
        }
        val
    }

    let mut data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let heap = permutohedron::Heap::new(&mut data);

    for data in heap {
        let ps: Vec<_> = s.iter().zip(data.iter()).collect();
        let na = convert(&a, &ps);
        let nb = convert(&b, &ps);
        let nc = convert(&c, &ps);
        let ha = *ps.iter().find(|&&(ch, _)| **ch == a[0]).unwrap().1;
        let hb = *ps.iter().find(|&&(ch, _)| **ch == b[0]).unwrap().1;
        let hc = *ps.iter().find(|&&(ch, _)| **ch == c[0]).unwrap().1;
        if ha == 0 || hb == 0 || hc == 0 {
            continue;
        }
        if na + nb == nc {
            println!("{}\n{}\n{}", na, nb, nc);
            return;
        }
    }
    println!("UNSOLVABLE");
    return;
}
