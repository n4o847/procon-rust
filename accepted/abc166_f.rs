use proconio::{fastout, input};

use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize, mut a: [u64; 3],
        mut q: [Chars; n],
    }
    q.push(vec!['X', 'X']);
    let mut ans = vec![];
    let id = |x| match x {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => 3,
    };
    for i in 0..n {
        let x = id(q[i][0]);
        let y = id(q[i][1]);
        if a[x] > 1 || (a[x], a[y]) == (1, 0) {
            a[x] -= 1;
            a[y] += 1;
            ans.push(q[i][1]);
        } else if a[y] > 1 || (a[x], a[y]) == (0, 1) {
            a[x] += 1;
            a[y] -= 1;
            ans.push(q[i][0]);
        } else if (a[x], a[y]) == (0, 0) {
            println!("No");
            return;
        } else {
            if q[i][0] == q[i + 1][0] || q[i][0] == q[i + 1][1] {
                a[x] += 1;
                a[y] -= 1;
                ans.push(q[i][0]);
            } else {
                a[x] -= 1;
                a[y] += 1;
                ans.push(q[i][1]);
            }
        }
    }
    println!("Yes");
    for &s in ans.iter() {
        println!("{}", s);
    }
}
