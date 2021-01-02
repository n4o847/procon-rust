use proconio::{fastout, input};

#[fastout]
fn main() {
    let abs = |x: i64| if x >= 0 { x } else { -x };
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let x = abs(xy[i].0 - xy[j].0);
            let y = abs(xy[i].1 - xy[j].1);
            if x >= y {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
