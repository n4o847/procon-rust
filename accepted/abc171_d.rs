use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }
    let mut s = 0;
    let mut z = vec![0; 100001];
    for x in a {
        z[x] += 1;
        s += x;
    }
    for _ in 0..q {
        input! {
            b: usize, c: usize,
        }
        s += c * z[b];
        s -= b * z[b];
        z[c] += z[b];
        z[b] = 0;
        println!("{}", s);
    }
}
