// https://github.com/atcoder/ac-library/blob/master/atcoder/math.hpp
// https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/math.rs

#[allow(dead_code)]
fn pow_mod(mut x: u64, mut n: u64, m: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = r * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    r
}

#[allow(dead_code)]
fn inv_mod(x: u64, m: u64) -> u64 {
    pow_mod(x, m - 2, m)
}

#[allow(dead_code)]
fn crt(rm: &[(i64, i64)]) -> Option<(i64, i64)> {
    let (mut r0, mut m0) = (0, 1);
    for &(r1, mut m1) in rm.iter() {
        assert!(1 <= m1);
        let mut r1 = r1.rem_euclid(m1);
        if m0 < m1 {
            std::mem::swap(&mut r0, &mut r1);
            std::mem::swap(&mut m0, &mut m1);
        }
        if m0 % m1 == 0 {
            if r0 % m1 != r1 {
                return None;
            }
            continue;
        }
        let (g, im) = inv_gcd(m0, m1);
        let u1 = m1 / g;
        if (r1 - r0) % g != 0 {
            return None;
        }
        let x = (r1 - r0) / g % u1 * im % u1;
        r0 += x * m0;
        m0 *= u1;
        if r0 < 0 {
            r0 += m0;
        }
    }
    Some((r0, m0))
}

#[allow(dead_code)]
fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let a = a.rem_euclid(b);
    if a == 0 {
        return (b, 0);
    }
    let (mut s, mut t) = (b, a);
    let (mut m0, mut m1) = (0, 1);
    while t > 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u;
        std::mem::swap(&mut s, &mut t);
        std::mem::swap(&mut m0, &mut m1);
    }
    if m0 < 0 {
        m0 += b / s;
    }
    (s, m0)
}
