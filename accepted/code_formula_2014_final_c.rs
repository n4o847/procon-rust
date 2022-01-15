use proconio::fastout;

use std::collections::BTreeSet;
use std::io::Read;

#[fastout]
fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut b = BTreeSet::new();
    let mut n: Option<String> = None;
    for c in s.chars() {
        match c {
            '@' => {
                if let Some(m) = &n {
                    b.insert(m.clone());
                }
                n = Some(String::new());
            }
            'a'..='z' => {
                if let Some(m) = &mut n {
                    m.push(c);
                }
            }
            ' ' | '\n' => {
                if let Some(m) = &n {
                    b.insert(m.clone());
                }
                n = None;
            }
            _ => {}
        }
    }
    for m in b {
        if m.len() != 0 {
            println!("{}", m);
        }
    }
}
