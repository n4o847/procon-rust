use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let mut n = s.len();
    loop {
        if s[..n].ends_with("dream") {
            n -= 5;
        } else if s[..n].ends_with("dreamer") {
            n -= 7;
        } else if s[..n].ends_with("erase") {
            n -= 5;
        } else if s[..n].ends_with("eraser") {
            n -= 6;
        } else {
            break;
        }
    }
    if n == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
