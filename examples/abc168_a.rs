use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let ans = match n % 10 {
        2 | 4 | 5 | 7 | 9 => "hon",
        0 | 1 | 6 | 8 => "pon",
        3 => "bon",
        _ => unreachable!(),
    };
    println!("{}", ans);
}
