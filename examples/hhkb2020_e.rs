use proconio::{fastout, input};

use proconio::marker::Chars;

fn pow(mut x: u64, mut y: u64, z: u64) -> u64 {
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

#[fastout]
fn main() {
    const M: u64 = 1_000_000_007;
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    let mut a = vec![vec![0_u64; w]; h];
    let mut l = 0;
    for i in 0..h {
        let mut c = 0_u64;
        for j in 0..w {
            match s[i][j] {
                '.' => {
                    c += 1;
                    l += 1;
                }
                '#' => {
                    for k in (j - c as usize)..j {
                        a[i][k] += c;
                    }
                    c = 0;
                }
                _ => unreachable!(),
            }
        }
        for k in (w - c as usize)..w {
            a[i][k] += c;
        }
    }
    for j in 0..w {
        let mut c = 0_u64;
        for i in 0..h {
            match s[i][j] {
                '.' => {
                    c += 1;
                }
                '#' => {
                    for k in (i - c as usize)..i {
                        a[k][j] += c - 1;
                    }
                    c = 0;
                }
                _ => unreachable!(),
            }
        }
        for k in (h - c as usize)..h {
            a[k][j] += c - 1;
        }
    }
    let mut s = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != 0 {
                s += pow(2, l - a[i][j], M);
                s %= M;
            }
        }
    }
    let ans = (pow(2, l, M) * l % M + M - s) % M;
    println!("{}", ans);
}
