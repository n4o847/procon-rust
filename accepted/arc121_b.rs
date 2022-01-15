use proconio::{fastout, input};

fn cost(x: &Vec<i64>, y: &Vec<i64>) -> i64 {
    if x.is_empty() || y.is_empty() {
        return std::i64::MAX / 2;
    }
    let mut ans = (x[0] - y[0]).abs();
    for &a in x.iter() {
        let v = match y.binary_search(&a) {
            Ok(_) => 0,
            Err(i) => {
                let l = if i == 0 { y[i] } else { y[i - 1] };
                let r = if i < y.len() { y[i] } else { y[i - 1] };
                (a - l).abs().min((r - a).abs())
            }
        };
        ans = ans.min(v);
    }
    ans
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ac: [(i64, char); 2 * n],
    }
    let mut r = vec![];
    let mut g = vec![];
    let mut b = vec![];
    for &(a, c) in ac.iter() {
        match c {
            'R' => r.push(a),
            'G' => g.push(a),
            'B' => b.push(a),
            _ => {}
        }
    }
    r.sort();
    g.sort();
    b.sort();
    let (x, y, z) = if r.len() % 2 == 0 {
        if g.len() % 2 == 0 {
            println!("0");
            return;
        } else {
            (g, b, r)
        }
    } else {
        if g.len() % 2 == 0 {
            (r, b, g)
        } else {
            (r, g, b)
        }
    };
    let ans = cost(&x, &y).min(cost(&x, &z) + cost(&y, &z));
    println!("{}", ans);
}
