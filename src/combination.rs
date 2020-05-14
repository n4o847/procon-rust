use cargo_snippet::snippet;

#[snippet("combination")]
#[snippet(prefix = "use combination::*;")]
mod combination {
    use num::traits::One;
    use std::ops::{Add, Div, Sub};
    pub struct Combination<T>(Vec<T>);
    impl<T> Combination<T>
    where
        T: One
            + Add<Output = T>
            + Sub<Output = T>
            + Div<Output = T>
            + Copy
            + From<usize>
            + Into<usize>,
    {
        #[allow(dead_code)]
        pub fn new() -> Self {
            Self(vec![T::one()])
        }

        #[allow(dead_code)]
        pub fn fact(&mut self, x: T) -> T {
            let a = self.0.len();
            let b: usize = x.into();
            for i in a..=b {
                self.0.push(self.0[i - 1] * T::from(i));
            }
            self.0[b]
        }

        #[allow(dead_code)]
        pub fn comb(&mut self, n: T, r: T) -> T {
            self.fact(n) / self.fact(r) / self.fact(n - r)
        }

        #[allow(dead_code)]
        pub fn multicomb(&mut self, n: T, r: T) -> T {
            self.comb(n + r - T::from(1), r)
        }
    }
}
