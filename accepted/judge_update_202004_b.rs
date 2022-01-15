use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xc: [(u32, char); n]
    }
    let mut r = xc
        .iter()
        .filter(|(_, c)| *c == 'R')
        .map(|(x, _)| x)
        .collect::<Vec<_>>();
    let mut b = xc
        .iter()
        .filter(|(_, c)| *c == 'B')
        .map(|(x, _)| x)
        .collect::<Vec<_>>();
    r.sort();
    b.sort();
    for x in r.iter() {
        println!("{}", x);
    }
    for x in b.iter() {
        println!("{}", x);
    }
}
