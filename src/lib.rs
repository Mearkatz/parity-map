//! For mapping integers based on their parity

use num::{BigInt, BigUint, Integer};
use std::convert::identity;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Represents a number's parity (whether it is even or odd).
pub enum Parity {
    #[allow(missing_docs)]
    Even,
    #[allow(missing_docs)]
    Odd,
}

impl Parity {
    /// Returns `true` if the parity is [`Even`].
    ///
    /// [`Even`]: Parity::Even
    #[must_use]
    pub const fn is_even(self) -> bool {
        matches!(self, Self::Even)
    }

    /// Returns `true` if the parity is [`Odd`].
    ///
    /// [`Odd`]: Parity::Odd
    #[must_use]
    pub const fn is_odd(self) -> bool {
        matches!(self, Self::Odd)
    }
}

/// Gives access to methods for mapping based on a number's parity.
pub trait ParityMap: Sized + Clone {
    /// Returns the parity of `&self`
    fn parity(&self) -> Parity;

    /// Returns `odd(self)` if `self` is odd, else `even(self)`.
    #[must_use]
    fn parity_map(self, odd: impl Fn(Self) -> Self, even: impl Fn(Self) -> Self) -> Self {
        match self.parity() {
            Parity::Even => even(self),
            Parity::Odd => odd(self),
        }
    }

    /// Calls a function on the result of `self.parity_map(..)`, returning the result.
    #[must_use]
    fn parity_map_and_then<F>(
        self,
        odd: impl Fn(Self) -> Self,
        even: impl Fn(Self) -> Self,
        after: impl Fn(Self) -> Self,
    ) -> Self {
        after(self.parity_map(odd, even))
    }

    /// Returns `f(self)` if `self` is even, else `self`.
    #[must_use]
    fn map_even(self, f: impl Fn(Self) -> Self) -> Self {
        self.parity_map(identity, f)
    }

    /// Returns `f(self)` if `self` is odd, else `self`.
    #[must_use]
    fn map_odd(self, f: impl Fn(Self) -> Self) -> Self {
        self.parity_map(f, identity)
    }
}

macro_rules! impl_parity_map {
    ($($t: ty),*) => {
        $(
        impl ParityMap for $t {
            fn parity(&self) -> Parity {
                if self.is_even() {
                    Parity::Even
                } else {
                    Parity::Odd
                }
            }
        })*
    };
}

impl_parity_map!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, BigInt, BigUint);
