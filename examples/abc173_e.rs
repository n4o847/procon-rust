use proconio::{fastout, input};

use std::cmp::{max, min};

fn pow(mut x: i64, mut y: i64, z: i64) -> i64 {
    let mut r = 1;
    while y > 0 {
        if y & 1 == 1 {
            r = r * x % z;
        }
        x = x * x % z;
        y >>= 1;
    }
    r
}

fn inv(x: i64, y: i64) -> i64 {
    pow((x % y + y) % y, y - 2, y)
}

#[fastout]
fn main() {
    const M: i64 = 1_000_000_007;
    input! {
        n: usize, mut k: usize,
        mut a: [i64; n],
    }
    let mut pos = 0;
    let mut zero = 0;
    let mut neg = 0;
    for &x in a.iter() {
        if x > 0 {
            pos += 1;
        } else if x < 0 {
            neg += 1;
        } else {
            zero += 1;
        }
    }
    if k > pos + neg {
        println!("0");
        return;
    }
    a.sort();

    if pos == 0 {
        if k % 2 == 0 {
            let mut acc = 1;
            for i in 0..k {
                acc = acc * a[i] % M;
            }
            acc = (acc % M + M) % M;
            if zero > 0 && acc < 0 {
                println!("0");
            } else {
                println!("{}", acc);
            }
        } else {
            let mut acc = 1;
            for i in n - k..n {
                acc = acc * a[i] % M;
            }
            acc = (acc % M + M) % M;
            if zero > 0 && acc < 0 {
                println!("0");
            } else {
                println!("{}", acc);
            }
        }
        return;
    }
    {
        let mut l = 0;
        let mut acc = 1;
        while k > pos {
            k -= 2;
            acc = acc * a[l] % M * a[l + 1] % M;
            l += 2;
        }
        for i in n - k..n {
            acc = acc * a[i] % M;
        }
        let mut ans = acc;
        for i in 0..(neg - l) / 2 {
            if n - k + i * 2 + 1 >= n {
                break;
            }
            if a[l + i * 2] * a[l + i * 2 + 1] > a[n - k + i * 2] * a[n - k + i * 2 + 1] {
            } else {
                break;
            }
            acc = acc * a[l + i * 2] % M * a[l + i * 2 + 1] % M;
            acc = acc * inv(a[n - k + i * 2], M) % M * inv(a[n - k + i * 2 + 1], M) % M;
            // ans = max(ans, acc);
        }
        acc = (acc % M + M) % M;
        println!("{}", acc);
    }
}
