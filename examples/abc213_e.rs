use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    const INF: u64 = std::u64::MAX;
    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![INF; w]; h];
    dist[0][0] = 0;
    heap.push(Reverse((0, (0, 0))));
    while let Some(Reverse((c, (vi, vj)))) = heap.pop() {
        if c > dist[vi][vj] {
            continue;
        }
        for &(di, dj) in &[(!0, 0), (1, 0), (0, !0), (0, 1)] {
            let wi = vi.wrapping_add(di);
            let wj = vj.wrapping_add(dj);
            if wi < h && wj < w {
                if s[wi][wj] == '.' {
                    let next = dist[vi][vj];
                    if next < dist[wi][wj] {
                        dist[wi][wj] = next;
                        heap.push(Reverse((next, (wi, wj))));
                    }
                }
            }
        }
        for &(di, dj) in &[
            (!1, !0),
            (!1, 0),
            (!1, 1),
            //
            (!0, !1),
            (!0, !0),
            (!0, 0),
            (!0, 1),
            (!0, 2),
            //
            (0, !1),
            (0, !0),
            (0, 1),
            (0, 2),
            //
            (1, !1),
            (1, !0),
            (1, 0),
            (1, 1),
            (1, 2),
            //
            (2, !0),
            (2, 0),
            (2, 1),
        ] {
            let wi = vi.wrapping_add(di);
            let wj = vj.wrapping_add(dj);
            if wi < h && wj < w {
                if s[wi][wj] == '#' {
                    let next = dist[vi][vj] + 1;
                    if next < dist[wi][wj] {
                        dist[wi][wj] = next;
                        heap.push(Reverse((next, (wi, wj))));
                    }
                }
            }
        }
    }
    let ans = dist[h - 1][w - 1];
    println!("{}", ans);
}
