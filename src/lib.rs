use std::{fmt, str};

mod error;
mod parser;

use error::Error;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money(Inner);
pub type Inner = i64;

impl Money {
    pub const MIN_INNER: Inner = -9223372036854775808;
    pub const MAX_INNER: Inner = 9223372036854775807;

    pub const fn min() -> Money {
        Money(Money::MIN_INNER)
    }

    pub const fn max() -> Money {
        Money(Money::MAX_INNER)
    }

    pub const fn none() -> Money {
        Money(0)
    }

    fn inner(&self) -> Inner {
        self.0
    }

    fn dollars(&self) -> String {
        format!("{}", self.0 / 100)
    }

    fn cents(&self) -> String {
        let n = self.0.abs() % 100;
        let mut zero_pad = "";
        if n < 10 {
            zero_pad = "0"
        }
        format!("{}{}", zero_pad, n)
    }
}

impl fmt::Debug for Money {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{}", self.dollars(), self.cents())
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{}", self.dollars(), self.cents())
    }
}

impl str::FromStr for Money {
    type Err = Error;

    fn from_str(money_str: &str) -> Result<Self, Self::Err> {
        Money::parse_str(money_str)
    }
}

impl Default for Money {
    #[inline]
    fn default() -> Self {
        Money::none()
    }
}

macro_rules! derive_trait_from_inner {
    (impl $imp:ident, $method:ident) => {
        impl $imp for Money {
            type Output = Self;

            fn $method(self, rhs: Money) -> Self::Output {
                Money(self.inner().$method(rhs.inner()))
            }
        }
    };
}

macro_rules! derive_trait_for_type {
    (impl $imp:ident with $t:ty, $method:ident) => {
        impl $imp<$t> for Money
        where
            $t: Into<i64>,
        {
            type Output = Self;

            fn $method(self, rhs: $t) -> Self::Output {
                Money(self.inner().$method(rhs as i64))
            }
        }
    };
}

macro_rules! add_mul_impl {
     ($($t:ty)+) => ($(
         derive_trait_for_type! { impl Mul with $t, mul }
     )+)
}

derive_trait_from_inner!(impl Add, add);
derive_trait_from_inner!(impl Sub, sub);
add_mul_impl! { i64 i32 i16 i8 u32 u16 u8 }

#[cfg(test)]
mod tests {
    use crate::Money;

    macro_rules! gen_mul_tests {
        ($t:ty, $success:ident, $of_max:ident, $of_min:ident) => {
            #[test]
            fn $success() {
                assert_eq!(Money(7) * 3 as $t, Money(21))
            }

            #[test]
            #[should_panic]
            #[allow(unused_must_use)]
            fn $of_max() {
                Money::max() * 100 as $t;
            }

            #[test]
            #[should_panic]
            #[allow(unused_must_use)]
            fn $of_min() {
                Money::min() * 100 as $t;
            }
        };
    }

    gen_mul_tests! {
        i64,
        test_mul_success_i64,
        test_mul_fail_overflow_max_i64,
        test_mul_fail_overflow_min_i64
    }

    gen_mul_tests! {
        i32,
        test_mul_success_i32,
        test_mul_fail_overflow_max_i32,
        test_mul_fail_overflow_min_i32
    }

    gen_mul_tests! {
        i16,
        test_mul_success_i16,
        test_mul_fail_overflow_max_i16,
        test_mul_fail_overflow_min_i16
    }

    gen_mul_tests! {
        i8,
        test_mul_success_i8,
        test_mul_fail_overflow_max_i8,
        test_mul_fail_overflow_min_i8
    }

    gen_mul_tests! {
        u32,
        test_mul_success_u32,
        test_mul_fail_overflow_max_u32,
        test_mul_fail_overflow_min_u32
    }

    gen_mul_tests! {
        u16,
        test_mul_success_u16,
        test_mul_fail_overflow_max_u16,
        test_mul_fail_overflow_min_u16
    }

    gen_mul_tests! {
        u8,
        test_mul_success_u8,
        test_mul_fail_overflow_max_u8,
        test_mul_fail_overflow_min_u8
    }

    // macro_rules! test_mul_for_type {
    //     ($t:ty $success:expr $overflow_max:expr $overflow_min:) => {
    //         #[test]
    //         fn test_multiplication_i32_success() {
    //             assert_eq!(Money(7) * 3 as $t, Money(21))
    //         }
    //
    //         #[test]
    //         #[should_panic]
    //         #[allow(unused_must_use)]
    //         fn test_multiplication_($t)_failure_overflow_max() {
    //             Money::max() * 100 as $t;
    //         }
    //
    //         #[test]
    //         #[should_panic]
    //         #[allow(unused_must_use)]
    //         fn test_multiplication_$t_failure_overflow_min() {
    //             Money::min() * 100 as $t;
    //         }
    //     }
    // }

    #[test]
    fn test_money_default() {
        let default_money = Money::default();
        let nil_money = Money::none();

        assert_eq!(default_money, nil_money);
    }

    #[test]
    fn test_addition_success() {
        assert_eq!(Money(1) + Money(1), Money(2))
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_addition_failure_overflow_max() {
        Money::max() + Money(1);
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_addition_failure_overflow_min() {
        Money::min() + Money(-1);
    }

    #[test]
    fn test_subtraction_success() {
        assert_eq!(Money(2) - Money(1), Money(1))
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_subtraction_failure_overflow_max() {
        Money::max() - Money(-1);
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_subtraction_failure_overflow_min() {
        Money::min() - Money(1);
    }

    // #[test]
    // fn test_multiplication_i32_success() {
    //     assert_eq!(Money(7) * 3 as i32, Money(21))
    // }
    //
    // #[test]
    // #[should_panic]
    // #[allow(unused_must_use)]
    // fn test_multiplication_i32_failure_overflow_max() {
    //     Money::max() * 100 as i32;
    // }
    //
    // #[test]
    // #[should_panic]
    // #[allow(unused_must_use)]
    // fn test_multiplication_i32_failure_overflow_min() {
    //     Money::min() * 100 as i32;
    // }

    #[test]
    fn test_division() {
        println!("{}", 87808 / 11);
    }

    // SELECT m + '123' FROM money_data;
    // SELECT m + '123.45' FROM money_data;
    // SELECT m - '123.45' FROM money_data;
    // SELECT m / '2'::money FROM money_data;
    // SELECT m * 2 FROM money_data;
    // SELECT 2 * m FROM money_data;
    // SELECT m / 2 FROM money_data;

    // -- All true
    // SELECT m = '$123.00' FROM money_data;
    // SELECT m != '$124.00' FROM money_data;
    // SELECT m <= '$123.00' FROM money_data;
    // SELECT m >= '$123.00' FROM money_data;
    // SELECT m < '$124.00' FROM money_data;
    // SELECT m > '$122.00' FROM money_data;
    //
    // -- All false
    // SELECT m = '$123.01' FROM money_data;
    // SELECT m != '$123.00' FROM money_data;
    // SELECT m <= '$122.99' FROM money_data;
    // SELECT m >= '$123.01' FROM money_data;
    // SELECT m > '$124.00' FROM money_data;
    // SELECT m < '$122.00' FROM money_data;
}
