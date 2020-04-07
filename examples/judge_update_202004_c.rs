use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64, c: u64
    }
    fn f(x: u64, y: u64, z: u64, a: u64, b: u64, c: u64) -> u64 {
        if (x, y, z) == (a, b, c) {
            return 1;
        }
        let mut s = 0;
        if x < a {
            s += f(x + 1, y, z, a, b, c);
        }
        if y < b && y < x {
            s += f(x, y + 1, z, a, b, c);
        }
        if z < c && z < y {
            s += f(x, y, z + 1, a, b, c);
        }
        s
    }
    println!("{}", f(0, 0, 0, a, b, c));
}
