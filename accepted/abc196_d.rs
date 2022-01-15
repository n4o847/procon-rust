use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize, a: usize, b: usize,
    }
    let mut ans = 0;
    for s in 0..(1 << ((h - 1) * w)) {
        for i in 0..h {
            for j in 0..w {
                let u = if i == 0 {
                    0
                } else {
                    (s >> ((i - 1) * w + j)) & 1
                };
                let d = if i == h - 1 {
                    0
                } else {
                    (s >> (i * w + j)) & 1
                };
                if u + d == 2 {
                    continue;
                }
            }
        }
        'pat: for t in 0..(1 << (h * (w - 1))) {
            let mut tt = 0;
            for i in 0..h {
                for j in 0..w {
                    let u = if i == 0 {
                        0
                    } else {
                        (s >> ((i - 1) * w + j)) & 1
                    };
                    let d = if i == h - 1 {
                        0
                    } else {
                        (s >> (i * w + j)) & 1
                    };
                    let l = if j == 0 {
                        0
                    } else {
                        (t >> ((j - 1) * h + i)) & 1
                    };
                    let r = if j == w - 1 {
                        0
                    } else {
                        (t >> (j * h + i)) & 1
                    };
                    if u + d + l + r > 1 {
                        continue 'pat;
                    }
                    if u + d + l + r == 0 {
                        tt += 1;
                    }
                }
            }
            if tt == b {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
