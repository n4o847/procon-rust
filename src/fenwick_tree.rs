use cargo_snippet::snippet;

#[snippet("use fenwick_tree")]
#[snippet(prefix = "use fenwick_tree::*;")]
mod fenwick_tree {
    use num::traits::Zero;
    use std::ops::{AddAssign, Bound::*, RangeBounds, Sub};

    pub struct FenwickTree<T> {
        data: Vec<T>,
    }

    impl<T> FenwickTree<T>
    where
        T: Copy + AddAssign + Sub<Output = T> + Zero,
    {
        #[allow(dead_code)]
        pub fn new(n: usize) -> Self {
            Self {
                data: vec![T::zero(); n],
            }
        }

        #[allow(dead_code)]
        pub fn add(&mut self, mut i: usize, x: T) {
            i += 1;
            while i <= self.data.len() {
                self.data[i - 1] += x;
                i += i & i.wrapping_neg();
            }
        }

        #[allow(dead_code)]
        pub fn cumsum(&self, mut i: usize) -> T {
            let mut s = T::zero();
            while i > 0 {
                s += self.data[i - 1];
                i -= i & i.wrapping_neg();
            }
            s
        }

        #[allow(dead_code)]
        pub fn sum(&self, index: impl RangeBounds<usize>) -> T {
            let l = match index.start_bound() {
                Included(&i) => i,
                Excluded(&i) => i + 1,
                Unbounded => 0,
            };
            let r = match index.end_bound() {
                Included(&i) => i + 1,
                Excluded(&i) => i,
                Unbounded => self.data.len() - 1,
            };
            self.cumsum(r) - self.cumsum(l)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fenwick_tree::*;

    #[test]
    fn practice2_a() {
        let mut ft = FenwickTree::new(5);
        for (i, a) in vec![1, 2, 3, 4, 5].into_iter().enumerate() {
            ft.add(i, a);
        }
        assert_eq!(ft.sum(0..5), 15);
        assert_eq!(ft.sum(2..4), 7);
        ft.add(3, 10);
        assert_eq!(ft.sum(0..5), 25);
        assert_eq!(ft.sum(0..3), 6);
    }
}
