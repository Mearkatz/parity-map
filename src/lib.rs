//! For mapping integers based on their parity

use num::Integer;
use std::convert::identity;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Represents a number's parity (whether it is even or odd).
#[allow(missing_docs)]
pub enum Parity {
    Even,
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
pub trait ParityMap: Sized {
    /// Returns the parity of `&self`
    fn parity(&self) -> Parity;

    /// Returns `odd(self)` if `self` is odd, else `even(self)`.
    #[must_use]
    fn parity_map(self, odd: impl FnOnce(Self) -> Self, even: impl FnOnce(Self) -> Self) -> Self {
        match self.parity() {
            Parity::Even => even(self),
            Parity::Odd => odd(self),
        }
    }

    /// Calls a function on the result of `self.parity_map(..)`, returning the result.
    #[must_use]
    fn parity_map_and_then<F>(
        self,
        odd: impl FnOnce(Self) -> Self,
        even: impl FnOnce(Self) -> Self,
        after: impl FnOnce(Self) -> Self,
    ) -> Self {
        after(self.parity_map(odd, even))
    }

    /// Returns `f(self)` if `self` is even, else `self`.
    #[must_use]
    fn map_even(self, f: impl FnOnce(Self) -> Self) -> Self {
        self.parity_map(identity, f)
    }

    /// Returns `f(self)` if `self` is odd, else `self`.
    #[must_use]
    fn map_odd(self, f: impl FnOnce(Self) -> Self) -> Self {
        self.parity_map(f, identity)
    }
}

impl<T: Integer> ParityMap for T {
    fn parity(&self) -> Parity {
        if self.is_even() {
            Parity::Even
        } else {
            Parity::Odd
        }
    }
}
