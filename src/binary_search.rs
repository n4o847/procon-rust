use cargo_snippet::snippet;

#[snippet("use binary_search")]
#[snippet(prefix = "use binary_search::*;")]
mod binary_search {
    use std::cmp::Ordering::*;
    use std::ops::Range;

    pub trait BinarySearch<T> {
        fn lower_bound(&self, x: &T) -> usize;
        fn upper_bound(&self, x: &T) -> usize;
        fn equal_range(&self, x: &T) -> Range<usize>;
        fn partition_point<F>(&self, f: F) -> usize
        where
            F: FnMut(&T) -> bool;
    }

    impl<T: Ord> BinarySearch<T> for [T] {
        fn lower_bound(&self, x: &T) -> usize {
            self.partition_point(|y| match y.cmp(x) {
                Less => true,
                Equal | Greater => false,
            })
        }

        fn upper_bound(&self, x: &T) -> usize {
            self.partition_point(|y| match y.cmp(x) {
                Less | Equal => true,
                Greater => false,
            })
        }

        fn equal_range(&self, x: &T) -> Range<usize> {
            self.lower_bound(x)..self.upper_bound(x)
        }

        fn partition_point<F>(&self, mut f: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            let mut low = 0;
            let mut high = self.len();
            while low != high {
                let mid = (low + high) / 2;
                if f(&self[mid]) {
                    low = mid + 1;
                } else {
                    high = mid;
                }
            }
            low
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binary_search::*;

    #[test]
    fn test_binary_search() {
        let v = vec![1, 1, 2, 3, 5, 8, 13];
        assert_eq!(v.lower_bound(&8), 5);
        assert_eq!(v.upper_bound(&8), 6);
        assert_eq!(v.partition_point(|x| *x < 8), 5);
    }
}
