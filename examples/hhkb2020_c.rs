use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        pp: [usize; n],
    }
    let mut a = vec![false; 200_002];
    let mut idx = 0;
    for &p in pp.iter() {
        a[p] = true;
        while a[idx] {
            idx += 1;
        }
        println!("{}", idx);
    }
}
