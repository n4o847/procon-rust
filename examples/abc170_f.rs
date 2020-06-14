use proconio::{fastout, input};

use proconio::marker::{Chars, Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, k: usize,
        x1: Usize1, y1: Usize1, x2: Usize1, y2: Usize1,
        c: [Chars; h],
    }
    let mut d = vec![vec![None; w]; h];
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, x1, y1)));
    while let Some(Reverse((z, x, y))) = q.pop() {
        for &(dx, dy) in [(!0, 0), (1, 0), (0, !0), (0, 1)].iter() {
            for t in 1..=k {
                let nx = t.wrapping_mul(dx).wrapping_add(x);
                let ny = t.wrapping_mul(dy).wrapping_add(y);
                if nx >= h || ny >= w {
                    break;
                }
                if c[nx][ny] == '@' {
                    break;
                }
                if let Some(dxy) = d[nx][ny] {
                    if z + 1 > dxy {
                        break;
                    }
                    if z + 1 == dxy {
                        continue;
                    }
                }
                d[nx][ny] = Some(z + 1);
                q.push(Reverse((z + 1, nx, ny)));
            }
        }
    }
    println!("{}", d[x2][y2].unwrap_or(-1));
}
