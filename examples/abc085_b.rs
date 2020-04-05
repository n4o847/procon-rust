use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        mut d: [i64; n],
    }
    let mut ans = 0;
    let mut a = 0;
    d.sort();
    for x in d.iter() {
        if a < *x {
            ans += 1;
        }
        a = *x;
    }
    println!("{}", ans);
}
