use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize, q: usize,
        wv: [(u64, u64); n],
        x: [u64; m],
        lr: [(Usize1, Usize1); q],
    }
    for &(l, r) in lr.iter() {
        let mut x: Vec<_> = (0..l).chain((r + 1)..m).map(|i| x[i]).collect();
        x.sort();
        let mut ans = 0;
        let mut used = vec![false; n];
        for &x in x.iter() {
            let mut mj: Option<usize> = None;
            for j in 0..n {
                if used[j] {
                    continue;
                }
                let (w, v) = wv[j];
                if w > x {
                    continue;
                }
                if let Some(k) = mj {
                    if v > wv[k].1 || (v == wv[k].1 && w > wv[k].0) {
                        mj = Some(j);
                    }
                } else {
                    mj = Some(j);
                }
            }
            if let Some(mj) = mj {
                used[mj] = true;
                ans += wv[mj].1;
            }
        }
        println!("{}", ans);
    }
}
