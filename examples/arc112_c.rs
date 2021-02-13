use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
    }
    let mut parent = vec![0; n];
    let mut children = vec![vec![]; n];
    for i in 1..n {
        parent[i] = p[i - 1];
        children[p[i - 1]].push(i);
    }
    let mut st = vec![(0, false); n];
    fn g(i: usize, ch: &Vec<Vec<usize>>, st: &mut Vec<(i64, bool)>) {
        let mut gain = -1;
        let mut flip = true;
        let mut odds = vec![];
        let mut neg = 0;
        for &j in ch[i].iter() {
            g(j, ch, st);
            flip ^= st[j].1;
            if !st[j].1 {
                if st[j].0 >= 0 {
                    gain += st[j].0;
                } else {
                    neg += st[j].0;
                }
            } else {
                odds.push(st[j].0);
            }
        }
        odds.sort();
        odds.reverse();
        for i in 0..odds.len() {
            if i % 2 == 0 {
                gain += odds[i];
            } else {
                gain -= odds[i];
            }
        }
        if odds.len() % 2 == 0 {
            gain += neg;
        } else {
            gain -= neg;
        }
        st[i] = (gain, flip);
    }
    g(0, &children, &mut st);
    let loss = st[0].0;
    let ans = (n as i64 - loss) / 2;
    println!("{}", ans);
}
