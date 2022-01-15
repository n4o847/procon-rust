use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: Chars,
    }
    fn comb(mut m: usize, mut n: usize) -> usize {
        let comb3 = &[[1, 0, 0], [1, 1, 0], [1, 2, 1]];
        let mut ans = 1;
        while m > 0 || n > 0 {
            ans *= comb3[m % 3][n % 3];
            ans %= 3;
            m /= 3;
            n /= 3;
        }
        ans
    }
    let c: Vec<_> = c
        .into_iter()
        .map(|c| match c {
            'B' => 0,
            'W' => 1,
            'R' => 2,
            _ => unreachable!(),
        })
        .collect();
    let mut ans = 0;
    for i in 0..n {
        // eprintln!("{}: {} * {}", i, comb(n - 1, i), c[i]);
        ans += comb(n - 1, i) * c[i];
        ans %= 3;
    }
    let ans = if n % 2 == 0 { (3 - ans) % 3 } else { ans };
    println!(
        "{}",
        match ans {
            0 => 'B',
            1 => 'W',
            2 => 'R',
            _ => unreachable!(),
        }
    );
}
