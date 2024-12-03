use std::iter::{Product, Sum};

/// A wrapper for summing on results with `all` sematics
/// this will produce the sum if all the elements are Ok,
/// or return the first error
///
/// # Example
/// ```
/// # use advent_of_code_2024::All;
/// let vals = [1, 2, 3];
/// // `fallible` is now an iterator of results, so can't be summed
/// // in a straightforward way
/// let fallible = vals.into_iter().map(|x| -> Result<u32, ()> {
///     Ok(x)
/// });
/// // All helps with that
/// let All(res_total) = fallible.map(All).sum();
/// assert_eq!(res_total, Ok(6));
///
/// let fallible = [Ok(1), Err("first"), Ok(3), Err("second")];
/// let All(res_total) = fallible.into_iter().map(All).sum();
/// assert_eq!(res_total, Err("first"));
///
/// ```
/// # Note
/// the implementation of [`Sum`] for T must
/// respect associotivity in order for this to
/// work correctly, i.e the following test should pass for
/// your type
/// ```
/// # type T = usize;
/// # let [a, b, c] = [1, 2, 3];
/// let left: T = [[a, b].into_iter().sum::<T>(), c].into_iter().sum();
/// let right: T = [a, b, c].into_iter().sum();
/// assert_eq!(left, right);
/// ```
pub struct All<T, E>(pub Result<T, E>);

impl<T: Sum, E> Sum for All<T, E> {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut acc = match iter.next() {
            None => return All(Ok([].into_iter().sum())),
            Some(All(Ok(val))) => val,
            Some(All(err)) => return All(err),
        };
        for All(item) in iter {
            match item {
                Ok(item) => acc = [acc, item].into_iter().sum(),
                err => return All(err),
            }
        }
        All(Ok(acc))
    }
}

impl<T: Product, E> Product for All<T, E> {
    fn product<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut acc = match iter.next() {
            None => return All(Ok([].into_iter().product())),
            Some(All(Ok(val))) => val,
            Some(All(err)) => return All(err),
        };
        for All(item) in iter {
            match item {
                Ok(item) => acc = [acc, item].into_iter().product(),
                err => return All(err),
            }
        }
        All(Ok(acc))
    }
}
