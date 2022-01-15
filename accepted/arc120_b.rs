use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }
    const M: u64 = 998244353;
    let mut ans = 1;
    for k in 0..=(h + w - 2) {
        let mut r = 0;
        let mut b = 0;
        for i in 0..=k {
            let j = k - i;
            if !(i < h && j < w) {
                continue;
            }
            match s[i][j] {
                'R' => r += 1,
                'B' => b += 1,
                _ => (),
            }
        }
        if r == 0 && b == 0 {
            ans = ans * 2 % M;
        } else if b != 0 && r != 0 {
            ans = 0;
            break;
        }
    }
    println!("{}", ans);
}
