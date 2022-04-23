use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let m = *a.iter().max().unwrap();
    let mut h = vec![0; 1 + m as usize];
    for &a in a.iter() {
        h[a as usize] += 1;
    }
    let mut cnt: u64 = 0;
    for aj in 1..=m {
        for ak in 1..=(m / aj) {
            let ai = aj * ak;
            cnt += h[ai as usize] * h[aj as usize] * h[ak as usize];
        }
    }
    println!("{}", cnt);
}
