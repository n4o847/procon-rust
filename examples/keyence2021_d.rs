use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", (1 << n) - 1);

    let mut h = vec![vec![vec![true, false]]];

    for i in 1..n {
        let mut a = vec![];
        for u in h[i - 1].iter() {
            let mut p = vec![];
            for &x in u.iter() {
                p.push(x);
                p.push(x);
            }
            a.push(p);
        }
        let mut p = vec![];
        for _ in 0..(1 << i) {
            p.push(true);
            p.push(false);
        }
        a.push(p);
        for u in h[i - 1].iter() {
            let mut p = vec![];
            for &x in u.iter() {
                p.push(x);
                p.push(!x);
            }
            a.push(p);
        }
        h.push(a);
    }

    for c in h[n - 1].iter() {
        for &x in c.iter() {
            print!("{}", if x { 'A' } else { 'B' });
        }
        println!("");
    }
}
