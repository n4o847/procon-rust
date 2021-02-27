use num::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            x: i128, y: i128, p: i128, q: i128,
        }
        let s = (x + y) * 2;
        let t = p + q;
        let mut ans = std::i128::MAX;
        for i in x..(x + y) {
            for j in p..(p + q) {
                if let Some((r, _m)) = chinese_rem(i, s, j, t) {
                    assert_ne!(r, 0);
                    ans = ans.min(r);
                }
            }
        }
        if ans >= std::i128::MAX {
            println!("infinity");
        } else {
            println!("{}", ans);
        }
    }
}

fn chinese_rem(b1: i128, m1: i128, b2: i128, m2: i128) -> Option<(i128, i128)> {
    let (d, p, q) = ext_gcd(m1, m2);
    assert_eq!(d, p * m1 + q * m2);
    if (b2 - b1) % d != 0 {
        return None;
    }
    let m = m1.lcm(&m2);
    let t = (b2 - b1) / d * p.rem_euclid(m2 / d);
    let r = (b1 + m1 * t).rem_euclid(m);
    Some((r, m))
}

fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, y, x) = ext_gcd(b.rem_euclid(a), a);
        (g, x - (b / a) * y, y)
    }
}
