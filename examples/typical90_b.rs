use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    fn f(s: &mut String, i: usize, n: usize, t: i32) {
        if t < 0 {
            return;
        }
        if i == n {
            if t == 0 {
                println!("{}", s);
            }
        } else {
            s.push('(');
            f(s, i + 1, n, t + 1);
            s.pop();
            s.push(')');
            f(s, i + 1, n, t - 1);
            s.pop();
        }
    }
    f(&mut String::new(), 0, n, 0);
}
