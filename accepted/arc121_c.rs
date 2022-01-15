use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! { t: usize }
    for _ in 0..t {
        input! {
            n: usize,
            mut p: [Usize1; n],
        }
        let mut ans = vec![];

        if n == 2 {
            if p[0] > p[1] {
                p.swap(0, 1);
                ans.push(0);
            }
        } else if n == 3 {
            match &p[..] {
                [0, 1, 2] => {}
                [0, 2, 1] => {
                    for &i in [0, 1, 0, 1, 0].iter() {
                        p.swap(i, i + 1);
                        ans.push(i);
                    }
                }
                [1, 0, 2] => {
                    for &i in [0].iter() {
                        p.swap(i, i + 1);
                        ans.push(i);
                    }
                }
                [1, 2, 0] => {
                    for &i in [0, 1, 0, 1].iter() {
                        p.swap(i, i + 1);
                        ans.push(i);
                    }
                }
                [2, 0, 1] => {
                    for &i in [0, 1].iter() {
                        p.swap(i, i + 1);
                        ans.push(i);
                    }
                }
                [2, 1, 0] => {
                    for &i in [0, 1, 0].iter() {
                        p.swap(i, i + 1);
                        ans.push(i);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            for k in (4..n).rev() {
                let mut i = p.iter().position(|&x| x == k).unwrap();
                if i == 1 {
                    p.swap(2, 3);
                    ans.push(2);
                } else if i % 2 == 1 {
                    p.swap(0, 1);
                    ans.push(0);
                }
                while i < k {
                    p.swap(i, i + 1);
                    ans.push(i);
                    i += 1;
                }
                if ans.len() % 2 == 1 {
                    p.swap(1, 2);
                    ans.push(1);
                }
            }
            dbg!(&p[0..4]);
            let a = match &p[0..4] {
                [0, 1, 2, 3] => [].iter(),
                [0, 1, 3, 2] => [2].iter(),
                [0, 2, 1, 3] => [0, 1, 0, 1, 0].iter(),
                [0, 2, 3, 1] => [2, 1].iter(),
                [0, 3, 1, 2] => [2, 1, 2, 1].iter(),
                [0, 3, 2, 1] => [2, 1, 2].iter(),
                [1, 0, 2, 3] => [0].iter(),
                [1, 0, 3, 2] => [0, 1, 2, 1, 2, 1].iter(),
                [1, 2, 0, 3] => [0, 1, 0, 1].iter(),
                [1, 2, 3, 0] => [2, 1, 0].iter(),
                [1, 3, 0, 2] => [0, 1, 0, 1, 2].iter(),
                [1, 3, 2, 0] => [0, 1, 0, 1, 2, 1, 0, 1].iter(),
                [2, 0, 1, 3] => [0, 1].iter(),
                [2, 0, 3, 1] => [0, 1, 2, 1, 2].iter(),
                [2, 1, 0, 3] => [0, 1, 0].iter(),
                [2, 1, 3, 0] => [2, 1, 0, 1].iter(),
                [2, 3, 0, 1] => [0, 1, 0, 1, 2, 1].iter(),
                [2, 3, 1, 0] => [0, 1, 0, 1, 2, 1, 0].iter(),
                [3, 0, 1, 2] => [0, 1, 2].iter(),
                [3, 0, 2, 1] => [0, 1, 2, 1].iter(),
                [3, 1, 0, 2] => [0, 1, 0, 1, 2, 1, 2, 1].iter(),
                [3, 1, 2, 0] => [0, 1, 2, 1, 0].iter(),
                [3, 2, 0, 1] => [0, 1, 0, 1, 2, 1, 2].iter(),
                [3, 2, 1, 0] => [0, 1, 2, 1, 0, 1].iter(),
                _ => unreachable!(),
            };
            for &i in a {
                p.swap(i, i + 1);
                ans.push(i);
            }
            dbg!(&p);
        }

        assert!(p.windows(2).all(|w| w[0] < w[1]));

        println!("{}", ans.len());
        for &i in ans.iter() {
            print!("{} ", i + 1);
        }
        println!();
    }
}
