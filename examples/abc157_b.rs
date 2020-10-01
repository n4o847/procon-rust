use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [[u64; 3]; 3],
        n: usize,
        b: [u64; n],
    }
    let mut c = vec![vec![false; 3]; 3];
    for b in b {
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == b {
                    c[i][j] = true;
                }
            }
        }
    }
    let ans = (|| {
        for i in 0..3 {
            if c[i][0] && c[i][1] && c[i][2] {
                return true;
            }
            if c[0][i] && c[1][i] && c[2][i] {
                return true;
            }
        }
        if c[0][0] && c[1][1] && c[2][2] {
            return true;
        }
        if c[0][2] && c[1][1] && c[2][0] {
            return true;
        }
        false
    })();
    println!("{}", if ans { "Yes" } else { "No" });
}
