struct FenwickTree<T> {
  data: Vec<T>,
}

impl<T> FenwickTree<T>
where
  T: Copy + std::ops::AddAssign + std::ops::Sub<Output = T> + num::traits::Zero,
{
  #[allow(dead_code)]
  fn new(n: usize) -> Self {
    Self {
      data: vec![T::zero(); n],
    }
  }

  #[allow(dead_code)]
  fn add(&mut self, mut i: usize, x: T) {
    i += 1;
    while i <= self.data.len() {
      self.data[i - 1] += x;
      i += i & i.wrapping_neg();
    }
  }

  #[allow(dead_code)]
  fn cumsum(&self, mut i: usize) -> T {
    let mut s = T::zero();
    while i > 0 {
      s += self.data[i - 1];
      i -= i & i.wrapping_neg();
    }
    s
  }

  #[allow(dead_code)]
  fn sum(&self, index: impl std::ops::RangeBounds<usize>) -> T {
    use std::ops::Bound::*;
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
