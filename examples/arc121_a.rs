use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }
    let dist = |p: (i64, i64, usize), q: (i64, i64, usize)| {
        (
            (p.0 - q.0).abs().max((p.1 - q.1).abs()),
            p.2.min(q.2),
            q.2.max(q.2),
        )
    };
    let mut ds = vec![];
    let mut xyi = vec![];
    for i in 0..n {
        xyi.push((xy[i].0, xy[i].1, i));
    }
    xyi.sort();
    ds.push(dist(xyi[0], xyi[xyi.len() - 1]));
    ds.push(dist(xyi[1], xyi[xyi.len() - 1]));
    ds.push(dist(xyi[0], xyi[xyi.len() - 2]));
    xyi.sort_by_key(|&p| p.1);
    ds.push(dist(xyi[0], xyi[xyi.len() - 1]));
    ds.push(dist(xyi[1], xyi[xyi.len() - 1]));
    ds.push(dist(xyi[0], xyi[xy.len() - 2]));
    ds.sort();
    ds.dedup();
    println!("{}", ds[ds.len() - 2].0);
}
