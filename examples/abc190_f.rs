use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut bit = vec![0; n + 1];
    fn sum(bit: &Vec<i64>, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += bit[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    fn add(bit: &mut Vec<i64>, mut i: usize, x: i64) {
        while i < bit.len() {
            bit[i] += x;
            i += i & i.wrapping_neg();
        }
    }
    let mut ans = 0;
    for i in 0..n {
        ans += i as i64 - sum(&bit, a[i] as usize + 1);
        add(&mut bit, a[i] as usize + 1, 1);
    }
    for i in 0..n {
        println!("{}", ans);
        ans += n as i64 - a[i] as i64 * 2 - 1;
    }
}
