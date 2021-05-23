use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut s = vec![0];
    for &a in a.iter() {
        s.push(s.last().unwrap() + a);
    }
    let mut m = a[0];
    let mut t = 0;
    for i in 0..n {
        m = m.max(a[i]);
        t += s[i + 1];
        let f = t + m * (i as u64 + 1);
        println!("{}", f);
    }
}
