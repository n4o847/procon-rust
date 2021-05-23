use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        b: [i64; q],
    }
    a.sort();
    for &b in b.iter() {
        let ans = match a.binary_search(&b) {
            Ok(_) => 0,
            Err(i) => {
                let l = if i == 0 { a[i] } else { a[i - 1] };
                let r = if i < a.len() { a[i] } else { a[i - 1] };
                (b - l).abs().min((r - b).abs())
            }
        };
        println!("{}", ans);
    }
}
