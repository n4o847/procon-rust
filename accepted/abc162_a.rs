use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String
    }
    println!("{}", if n.find('7').is_some() { "Yes" } else { "No" });
}
