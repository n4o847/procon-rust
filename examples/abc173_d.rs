use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut b = vec![];
    for x in a {
        b.push(x);
        b.push(x);
    }
    b.sort();
    let mut ans = 0;
    for i in n..n * 2 - 1 {
        ans += b[i];
    }
    println!("{}", ans);
}
