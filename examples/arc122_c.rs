use num_integer::Roots;
use proconio::{fastout, input};

fn gen(mut x: u64, mut y: u64) -> Vec<u8> {
    let mut ops = Vec::new();
    loop {
        if x == 0 && y == 0 {
            break;
        } else if y == 0 {
            x -= 1;
            ops.push(1);
        } else if x == 0 {
            y -= 1;
            ops.push(2);
        } else if x > y {
            x -= y;
            ops.push(3);
        } else if x < y {
            y -= x;
            ops.push(4);
        } else {
            for _ in 0..y.sqrt() {
                y -= 1;
                ops.push(2);
            }
        }
    }
    ops.reverse();
    ops
}

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    const PHI: f64 = 0.61803398874989484820;
    let m = (n as f64 * PHI).round() as u64;
    let ops = (m.saturating_sub(1000)..=m.saturating_add(1000))
        .map(|k| gen(n, k))
        .min_by_key(|ops| ops.len())
        .unwrap();
    println!("{}", ops.len());
    for &op in ops.iter() {
        println!("{}", op);
    }
}
