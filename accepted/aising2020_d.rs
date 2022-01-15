use proconio::{fastout, input};

use proconio::marker::Chars;
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        xx: Chars,
    }
    let mut m = vec![None; n + 10];
    fn f(x: usize, m: &mut [Option<usize>]) -> usize {
        if x == 0 {
            return 0;
        }
        if let Some(r) = m[x] {
            return r;
        } else {
            let r = 1 + f(x % x.count_ones() as usize, m);
            m[x] = Some(r);
            return r;
        }
    }
    let ones = xx.iter().filter(|&&x| x == '1').count() as u64;
    let mut a = 0;
    let mut c = 0;
    for i in 0..n {
        a *= 2;
        c *= 2;
        if xx[i] == '1' {
            a += 1;
            c += 1;
        }
        a %= max(ones, 2) - 1;
        c %= ones + 1;
    }
    let mut p = 1;
    let mut r = 1;
    let mut res = vec![0; n];
    for i in (0..n).rev() {
        p %= max(ones, 2) - 1;
        r %= ones + 1;
        if xx[i] == '0' {
            let x = (c + r) % (ones + 1);
            res[i] = 1 + f(x as usize, &mut m);
        }
        if xx[i] == '1' {
            if ones <= 1 {
                res[i] = 0;
            } else {
                let x = (ones - 1 + a - p) % (ones - 1);
                res[i] = 1 + f(x as usize, &mut m);
            }
        }
        p *= 2;
        r *= 2;
    }
    for i in 0..n {
        println!("{}", res[i]);
    }
}
