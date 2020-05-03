use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, mut a: u64, mut b: u64, mut c: u64,
        mut q: [String; n],
    }
    q.push(String::from(""));
    let mut ans = vec![];
    for i in 0..n {
        match &q[i][..] {
            "AB" => {
                if a > 1 || (a, b) == (1, 0) {
                    a -= 1;
                    b += 1;
                    ans.push("B");
                } else if b > 1 || (a, b) == (0, 1) {
                    a += 1;
                    b -= 1;
                    ans.push("A");
                } else if (a, b) == (0, 0) {
                    println!("No");
                    return;
                } else {
                    if q[i + 1] == "AC" {
                        a += 1;
                        b -= 1;
                        ans.push("A");
                    } else {
                        a -= 1;
                        b += 1;
                        ans.push("B");
                    }
                }
            }
            "AC" => {
                if a > 1 || (a, c) == (1, 0) {
                    a -= 1;
                    c += 1;
                    ans.push("C");
                } else if c > 1 || (a, c) == (0, 1) {
                    a += 1;
                    c -= 1;
                    ans.push("A");
                } else if (a, c) == (0, 0) {
                    println!("No");
                    return;
                } else {
                    if q[i + 1] == "AB" {
                        a += 1;
                        c -= 1;
                        ans.push("A");
                    } else {
                        a -= 1;
                        c += 1;
                        ans.push("C");
                    }
                }
            }
            "BC" => {
                if b > 1 || (b, c) == (1, 0) {
                    b -= 1;
                    c += 1;
                    ans.push("C");
                } else if c > 1 || (b, c) == (0, 1) {
                    b += 1;
                    c -= 1;
                    ans.push("B");
                } else if (b, c) == (0, 0) {
                    println!("No");
                    return;
                } else {
                    if q[i + 1] == "AB" {
                        b += 1;
                        c -= 1;
                        ans.push("B");
                    } else {
                        b -= 1;
                        c += 1;
                        ans.push("C");
                    }
                }
            }
            _ => (),
        }
    }
    println!("Yes");
    for s in ans.iter() {
        println!("{}", s);
    }
}
