use cargo_snippet::snippet;
use std::cmp::Ordering;

#[snippet("BinarySearch")]
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
    fn partition_point<F>(&self, f: F) -> usize
    where
        F: FnMut(&T) -> bool;
}

#[snippet("BinarySearch")]
#[snippet(prefix = "use std::cmp::Ordering;")]
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        self.partition_point(|y| match y.cmp(x) {
            Ordering::Less => true,
            Ordering::Equal | Ordering::Greater => false,
        })
    }

    fn upper_bound(&self, x: &T) -> usize {
        self.partition_point(|y| match y.cmp(x) {
            Ordering::Less | Ordering::Equal => true,
            Ordering::Greater => false,
        })
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

#[test]
fn test_binary_search() {
    let v = vec![1, 1, 2, 3, 5, 8, 13];
    assert_eq!(v.lower_bound(&8), 5);
    assert_eq!(v.upper_bound(&8), 6);
    assert_eq!(v.partition_point(|x| *x < 8), 5);
}
