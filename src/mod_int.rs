use cargo_snippet::snippet;

#[snippet("mod_int")]
#[snippet(prefix = "use mod_int::*;")]
mod mod_int {
    use std::marker::PhantomData;

    #[macro_export]
    macro_rules! impl_modulus {
        ($t:ident, $m:expr) => {
            #[allow(dead_code)]
            pub struct $t;
            impl Modulus for $t {
                const VALUE: u64 = $m;
            }
        };
    }

    pub trait Modulus {
        const VALUE: u64;
    }

    impl_modulus!(M1000000007, 1_000_000_007);

    impl_modulus!(M998244353, 998_244_353);

    pub struct ModInt<M: Modulus>(pub u64, PhantomData<M>);

    impl<M: Modulus> std::marker::Copy for ModInt<M> {}

    impl<M: Modulus> std::clone::Clone for ModInt<M> {
        fn clone(&self) -> Self {
            Self(self.0, PhantomData::<M>)
        }
    }

    impl<M: Modulus, T: num::traits::AsPrimitive<i64>> From<T> for ModInt<M> {
        fn from(x: T) -> Self {
            Self(x.as_().rem_euclid(M::VALUE as i64) as u64, PhantomData::<M>)
        }
    }

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
}
