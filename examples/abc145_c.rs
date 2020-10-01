use proconio::{fastout, input};

use permutohedron::{factorial, Heap};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(f64, f64); n],
    }
    let mut sum = 0.0;
    let mut heap = Heap::new(&mut xy);
    while let Some(elt) = heap.next_permutation() {
        let mut total = 0.0;
        for i in 0..n - 1 {
            let (x1, y1) = elt[i];
            let (x2, y2) = elt[i + 1];
            total += ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
        }
        sum += total;
    }
    let ans = sum / factorial(n) as f64;
    println!("{}", ans);
}
