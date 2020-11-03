use proconio::{fastout, input};

fn divisors(n: u64) -> Vec<u64> {
    let mut a = vec![];
    let mut b = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            a.push(i);
            if i * i != n {
                b.push(n / i);
            }
        }
        i += 1;
    }
    b.reverse();
    a.append(&mut b);
    return a;
}

fn f(b: u64, n: u64) -> u64 {
    if n < b {
        n
    } else {
        f(b, n / b) + n % b
    }
}

#[fastout]
fn main() {
    input! {
        n: u64,
        s: u64,
    }
    if s > n {
        println!("-1");
        return;
    }
    if s == n {
        println!("{}", n + 1);
        return;
    }
    for b_ in divisors(n - s) {
        let b = b_ + 1;
        if f(b, n) == s {
            println!("{}", b);
            return;
        }
    }
    println!("-1");
}
