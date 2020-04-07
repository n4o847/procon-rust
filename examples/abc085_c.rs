use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64, y: u64
    }
    let (mut a, mut b, mut c) = (0, 0, y / 1000);
    while c >= 5 && a + b + c - 4 >= n {
        c -= 5;
        b += 1;
    }
    while b >= 2 && a + b + c - 1 >= n {
        b -= 2;
        a += 1;
    }
    if a + b + c == n {
        println!("{} {} {}", a, b, c);
    } else {
        println!("-1 -1 -1");
    }
}
