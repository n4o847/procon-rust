use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        at: [(i64, usize); n],
        q: usize,
        xs: [i64; q],
    }
    let mut c = 0;
    let mut l: Option<i64> = None;
    let mut r: Option<i64> = None;
    for &(a, t) in at.iter() {
        if t == 1 {
            c += a;
            l = l.map(|x| x + a);
            r = r.map(|x| x + a);
        } else if t == 2 {
            l = Some(if let Some(x) = l { x.max(a) } else { a });
            r = r.map(|y| y.max(l.unwrap()));
        } else if t == 3 {
            r = Some(if let Some(x) = r { x.min(a) } else { a });
            l = l.map(|y| y.min(r.unwrap()));
        }
    }
    for &x in xs.iter() {
        let mut a = x + c;
        if let Some(y) = l {
            a = a.max(y);
        }
        if let Some(y) = r {
            a = a.min(y);
        }
        println!("{}", a);
    }
}
