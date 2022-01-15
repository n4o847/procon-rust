use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        ns: [u64; t],
    }
    for mut n in ns {
        let mut ds = vec![];
        while n > 0 {
            ds.push(n % 10);
            n /= 10;
        }
        let c = loop {
            let mut f = false;
            let mut c = (ds[ds.len() - 1] + 2) / 3;
            for i in (0..(ds.len() - 1)).rev() {
                if (ds[i] + 2) / 3 < c && c > ds[i] {
                    ds[i + 1] -= 1;
                    ds[i] += 10;
                    f = true;
                    break;
                }
                c = c.max((ds[i] + 2) / 3);
            }
            if !f {
                break c;
            }
        };
        println!("{}", c);
    }
}
