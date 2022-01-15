use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: usize,
        dc: [(u64, u64); m],
    }
    let mut ans = 0;
    let mut sum = 0;
    for (d, c) in dc {
        ans += c;
        sum += d * c;
    }
    ans -= 1;
    while sum >= 10 {
        ans += sum / 10;
        sum = sum / 10 + sum % 10;
    }
    println!("{}", ans);
}
