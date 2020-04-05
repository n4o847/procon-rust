use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64, a: i64, b: i64,
    }
    let mut ans = 0;
    for i in 1..=n {
        let s = i % 10 + i / 10 % 10 + i / 100 % 10 + i / 1000 % 10 + i / 10000 % 10;
        if a <= s && s <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}
