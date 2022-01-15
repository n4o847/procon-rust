use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, l: u64,
        k: usize,
        a: [u64; n],
    }
    let ok = |m| {
        let mut t = 0;
        let mut c = 0;
        for &a in a.iter().chain(std::iter::once(&l)) {
            if a - t >= m {
                c += 1;
                t = a;
            }
        }
        c >= k + 1
    };
    let mut lo = 1;
    let mut hi = l + 1;
    while hi - lo > 1 {
        let m = (lo + hi) / 2;
        if ok(m) {
            lo = m;
        } else {
            hi = m;
        }
    }
    println!("{}", lo);
}
