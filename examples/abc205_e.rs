use proconio::{fastout, input};

#[fastout]
fn main() {
    type Mint = ModInt<M1000000007>;
    input! {
        n: u64, m: u64, k: u64,
    }
    let mut comb = Combination::<Mint>::new();
    let ans = if n == k {
        comb.comb(Mint::from(n + m), Mint::from(m))
    } else if n <= m + k {
        comb.comb(Mint::from(n + m), Mint::from(m))
            - comb.comb(Mint::from(n + m), Mint::from(n - k - 1))
    } else {
        Mint::from(0)
    };
    println!("{}", ans);
}

use mod_int::*;
mod mod_int {
    use std::marker::PhantomData;
    pub trait Modulus {
        const VALUE: u64;
    }
    #[macro_export]
    macro_rules! impl_modulus {
        ($ t : ident , $ m : expr ) => {
            #[allow(dead_code)]
            pub struct $t;
            impl Modulus for $t {
                const VALUE: u64 = $m;
            }
        };
    }
    impl_modulus!(M1000000007, 1_000_000_007);
    impl_modulus!(M998244353, 998_244_353);
    pub struct ModInt<M: Modulus>(u64, PhantomData<M>);
    impl<M: Modulus> std::marker::Copy for ModInt<M> {}
    impl<M: Modulus> std::clone::Clone for ModInt<M> {
        fn clone(&self) -> Self {
            Self(self.0, PhantomData::<M>)
        }
    }
    macro_rules! impl_primitive {
        ($ t : ty ) => {
            impl<M: Modulus> From<$t> for ModInt<M> {
                fn from(x: $t) -> Self {
                    Self(
                        (x as $t).rem_euclid(M::VALUE as $t) as u64,
                        PhantomData::<M>,
                    )
                }
            }
            impl<M: Modulus> From<ModInt<M>> for $t {
                fn from(x: ModInt<M>) -> $t {
                    x.0 as $t
                }
            }
        };
    }
    impl_primitive!(usize);
    impl_primitive!(u8);
    impl_primitive!(u16);
    impl_primitive!(u32);
    impl_primitive!(u64);
    impl_primitive!(isize);
    impl_primitive!(i8);
    impl_primitive!(i16);
    impl_primitive!(i32);
    impl_primitive!(i64);
    impl<M: Modulus> std::str::FromStr for ModInt<M> {
        type Err = std::num::ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::from(s.parse::<u64>()?))
        }
    }
    impl<M: Modulus> std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<M: Modulus> std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<M: Modulus> PartialEq for ModInt<M> {
        fn eq(&self, rhs: &Self) -> bool {
            self.0 == rhs.0
        }
    }
    impl<M: Modulus, Rhs: Into<Self>> std::ops::Add<Rhs> for ModInt<M> {
        type Output = Self;
        fn add(self, rhs: Rhs) -> Self::Output {
            Self::from(self.0 + rhs.into().0)
        }
    }
    impl<M: Modulus, Rhs: Into<Self>> std::ops::AddAssign<Rhs> for ModInt<M> {
        fn add_assign(&mut self, rhs: Rhs) {
            *self = *self + rhs;
        }
    }
    impl<M: Modulus, Rhs: Into<Self>> std::ops::Sub<Rhs> for ModInt<M> {
        type Output = Self;
        fn sub(self, rhs: Rhs) -> Self::Output {
            Self::from(self.0 + M::VALUE - rhs.into().0)
        }
    }
    impl<M: Modulus, Rhs: Into<Self>> std::ops::SubAssign<Rhs> for ModInt<M> {
        fn sub_assign(&mut self, rhs: Rhs) {
            *self = *self - rhs;
        }
    }
    impl<M: Modulus> std::ops::Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self::from(M::VALUE - self.0)
        }
    }
    impl<M: Modulus, Rhs: Into<Self>> std::ops::Mul<Rhs> for ModInt<M> {
        type Output = Self;
        fn mul(self, rhs: Rhs) -> Self::Output {
            Self::from(self.0 * rhs.into().0)
        }
    }
    impl<M: Modulus, Rhs: Into<Self>> std::ops::MulAssign<Rhs> for ModInt<M> {
        fn mul_assign(&mut self, rhs: Rhs) {
            *self = *self * rhs;
        }
    }
    impl<M: Modulus, Rhs: Into<Self>> std::ops::Div<Rhs> for ModInt<M> {
        type Output = Self;
        fn div(self, rhs: Rhs) -> Self::Output {
            self * num::pow(rhs.into(), M::VALUE as usize - 2)
        }
    }
    impl<M: Modulus, Rhs: Into<Self>> std::ops::DivAssign<Rhs> for ModInt<M> {
        fn div_assign(&mut self, rhs: Rhs) {
            *self = *self / rhs;
        }
    }
    impl<M: Modulus> num::traits::Zero for ModInt<M> {
        fn zero() -> Self {
            Self(0, PhantomData::<M>)
        }
        fn is_zero(&self) -> bool {
            self.0 == 0
        }
    }
    impl<M: Modulus> num::traits::One for ModInt<M> {
        fn one() -> Self {
            Self(1, PhantomData::<M>)
        }
    }
    #[allow(dead_code)]
    pub fn pow<M: Modulus>(x: ModInt<M>, y: ModInt<M>) -> ModInt<M> {
        num::pow(x, y.into())
    }
}

use combination::*;
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
