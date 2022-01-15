use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        a: [[u64; w]; h],
    }
    let mut y = vec![0; h];
    let mut x = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            y[i] += a[i][j];
            x[j] += a[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            if j > 0 {
                print!(" ");
            }
            print!("{}", y[i] + x[j] - a[i][j]);
        }
        println!();
    }
}
