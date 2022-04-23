use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        s: [Chars; n],
    }
    let mut ts = vec![];
    for s in s.iter() {
        let mut t = 0;
        for &c in s.iter() {
            t |= 1 << (c as usize - 'a' as usize);
        }
        ts.push(t);
    }
    let mut ans = 0;
    for b in 0..(1 << n) {
        let mut c = vec![0; 26];
        for i in 0..n {
            if (b >> i) & 1 == 1 {
                for ch in 0..26 {
                    if (ts[i] >> ch) & 1 == 1 {
                        c[ch] += 1;
                    }
                }
            }
        }
        let mut z = 0;
        for &cnt in c.iter() {
            if cnt == k {
                z += 1;
            }
        }
        ans = ans.max(z);
    }
    println!("{}", ans);
}
