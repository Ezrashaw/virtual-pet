use std::ops::{Range, RangeInclusive};

pub trait ClampedArithmetic: Sized {
    fn clamped_add(self, rhs: Self, range: RangeInclusive<Self>) -> Self;
    fn clamped_sub(self, rhs: Self, range: RangeInclusive<Self>) -> Self;
}

macro_rules! impl_clamped_math {
    ($($ty:ty),+) => {
        $(
            impl crate::utils::ClampedArithmetic for $ty {
                fn clamped_add(self, rhs: Self, range: RangeInclusive<Self>) -> Self {
                    let val = self + rhs;
                    if val < *range.start() {
                        *range.start()
                    } else if val > *range.end() {
                        *range.end()
                    } else {
                        val
                    }
                }

                fn clamped_sub(self, rhs: Self, range: RangeInclusive<Self>) -> Self {
                    let val = self + rhs;
                    if val < *range.start() {
                        *range.start()
                    } else if val > *range.end() {
                        *range.end()
                    } else {
                        val
                    }
                }
            }
        )*
    };
}

impl_clamped_math!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
