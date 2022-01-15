use proconio::{fastout, input};

fn solve() -> bool {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut used = vec![false; n * 2 + 1];
    let mut p = vec![];
    let mut q = vec![];
    let mut r = vec![];
    for (a, b) in ab {
        if a != -1 && b != -1 {
            p.push((a as usize, b as usize));
        } else if a != -1 {
            q.push(a as usize);
        } else if b != -1 {
            r.push(b as usize);
        }
    }
    p.sort();
    q.sort();
    r.sort();
    for &(a, b) in p.iter() {
        if used[a] {
            return false;
        }
        used[a] = true;
        if used[b] {
            return false;
        }
        used[b] = true;
        if a > b {
            return false;
        }
    }
    for &a in q.iter() {
        if used[a] {
            return false;
        }
        used[a] = true;
    }
    for &b in r.iter() {
        if used[b] {
            return false;
        }
        used[b] = true;
    }
    for &a0 in q.iter() {
        let mut u = None;
        for &(a1, b1) in p.iter() {
            if a1 < a0 && a0 < b1 || a0 + 1 == a1 {
                let b0 = a0 + (b1 - a1);
                if b0 > n * 2 {
                    return false;
                }
                if used[b0] {
                    return false;
                }
                used[b0] = true;
                u = Some(b0);
                break;
            }
        }
        if let Some(b0) = u {
            p.push((a0, b0));
        }
    }
    for &b0 in r.iter() {
        let mut u = None;
        for &(a1, b1) in p.iter() {
            if a1 < b0 && b0 < b1 || b1 + 1 == b0 {
                if b0 < b1 - a1 {
                    return false;
                }
                let a0 = b0 - (b1 - a1);
                if used[a0] {
                    return false;
                }
                used[a0] = true;
                u = Some(a0);
                break;
            }
        }
        if let Some(a0) = u {
            p.push((a0, b0));
        }
    }
    p.sort();
    for &(a0, b0) in p.iter() {
        for &(a1, b1) in p.iter() {
            if a0 < a1 && b1 < b0 {
                return false;
            }
            if a0 < a1 && a1 < b0 && b0 < b1 {
                if b0 - a0 != b1 - a1 {
                    return false;
                }
            }
        }
    }
    return true;
}

#[fastout]
fn main() {
    if solve() {
        println!("Yes");
    } else {
        println!("No");
    }
}
