//! For mapping integers based on their parity

use either::Either::{self, Left, Right};
use num::Integer;

/// Methods for mapping an integer by its parity
pub trait ParityMap<MapOddErr, MapEvenErr>: Sized + Integer {
    /// Applies a function to `self` if it's odd.
    /// # Errors
    /// If `self` is odd, but the function called on it fails, an `MapOddError` is returned.
    fn map_odd<F>(self, odd_fn: F) -> Result<Self, MapOddErr>
    where
        F: FnOnce(Self) -> Result<Self, MapOddErr>,
    {
        if self.is_odd() {
            odd_fn(self)
        } else {
            Ok(self)
        }
    }

    /// Applies a function to `self` if it's even.
    /// # Errors
    /// If `self` is even, but the function called on it fails, a `MapEvenError` is returned.
    fn map_even<F>(self, even_fn: F) -> Result<Self, MapEvenErr>
    where
        F: FnOnce(Self) -> Result<Self, MapEvenErr>,
    {
        if self.is_even() {
            even_fn(self)
        } else {
            Ok(self)
        }
    }

    /// Applies `even_fn` to `self` if it's even, else `odd_fn` is applied.
    /// # Errors
    /// If `self` is even, but the function called on it fails, a `MapEvenError` is returned.
    fn parity_map<Odd, Even>(
        self,
        even_fn: Even,
        odd_fn: Odd,
    ) -> Result<Self, Either<MapEvenErr, MapOddErr>>
    where
        Odd: FnOnce(Self) -> Result<Self, MapOddErr>,
        Even: FnOnce(Self) -> Result<Self, MapEvenErr>,
    {
        if self.is_even() {
            even_fn(self).map_err(Left)
        } else {
            odd_fn(self).map_err(Right)
        }
    }
}

impl<T, O, E> ParityMap<O, E> for T where T: Sized + Integer {}
