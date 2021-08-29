use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        mut aa: [[Usize1]; m],
    }
    let mut cs = vec![vec![]; n];
    for i in 0..m {
        cs[aa[i][0]].push(i);
        aa[i].reverse();
    }
    fn f(k: usize, cs: &mut Vec<Vec<usize>>, aa: &mut Vec<Vec<usize>>) {
        if cs[k].len() == 2 {
            for _ in 0..2 {
                let i = cs[k].pop().unwrap();
                aa[i].pop();
                if let Some(&j) = aa[i].last() {
                    cs[j].push(i);
                    if cs[j].len() == 2 {
                        f(j, cs, aa);
                    }
                }
            }
        }
    }
    for k in 0..n {
        f(k, &mut cs, &mut aa);
    }
    if aa.iter().all(|a| a.is_empty()) {
        println!("Yes");
    } else {
        println!("No");
    }
}
