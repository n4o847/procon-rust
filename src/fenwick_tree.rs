use cargo_snippet::snippet;

#[snippet("FenwickTree")]
struct FenwickTree<T>(Vec<T>);

#[snippet("FenwickTree")]
impl<T> FenwickTree<T>
where
  T: Copy + std::ops::AddAssign + num::traits::Zero,
{
  fn new(n: usize) -> Self {
    Self(vec![T::zero(); n + 1])
  }

  fn sum(&self, mut i: usize) -> T {
    let mut s = T::zero();
    while i > 0 {
      s += self.0[i];
      i -= i & i.wrapping_neg();
    }
    s
  }

  fn add(&mut self, mut i: usize, x: T) {
    while i < self.0.len() {
      self.0[i] += x;
      i += i & i.wrapping_neg();
    }
  }
}
