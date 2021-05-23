use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        s: Chars,
    }
    let mut t = SegTree::new(n);
    for i in 0..n {
        t.update(i, (s[i], i));
    }
    let mut l = 0;
    let mut z = String::new();
    for r in (n - k + 1)..=n {
        let (c, i) = t.query(l, r);
        l = i + 1;
        z.push(c);
    }
    println!("{}", z);
}

#[derive(Debug)]
struct SegTree {
    n: usize,
    data: Vec<(char, usize)>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        Self {
            n: n_,
            data: (0..(2 * n_ - 1)).map(|_| ('~', 0)).collect(),
        }
    }

    fn update(&mut self, mut k: usize, a: (char, usize)) {
        k += self.n - 1;
        self.data[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.data[k] = self.data[k * 2 + 1].min(self.data[k * 2 + 2]);
        }
    }

    fn query(&self, a: usize, b: usize) -> (char, usize) {
        self.query_(a, b, 0, 0, self.n)
    }

    fn query_(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> (char, usize) {
        if r <= a || b <= l {
            ('~', 0)
        } else if a <= l && r <= b {
            self.data[k]
        } else {
            let vl = self.query_(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.query_(a, b, k * 2 + 2, (l + r) / 2, r);
            vl.min(vr)
        }
    }
}
