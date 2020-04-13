use proconio::{fastout, input};

fn pow<T>(mut x: T, mut y: T, p: T) -> T
where
    T: num::PrimInt,
{
    let mut ret = num::one();
    while y > num::zero() {
        if y & num::one() == num::one() {
            ret = ret * x % p;
        }
        x = x * x % p;
        y = y >> 1;
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: u64, k: u64
    }
    const M: u64 = 1000000007;
    let mut v = vec![0; (k + 1) as usize];
    let mut ans = 0;
    for x in (1..=k).rev() {
        let mut s = pow(k / x, n, M);
        for y in 2..=(k / x) {
            s = (s + (M - v[(x * y) as usize])) % M;
        }
        v[x as usize] = s;
        ans += s * x;
        ans %= M;
    }
    println!("{}", ans);
}
