use std::iter::Sum;

/// A wrapper for summing on results with `all` sematics
/// this will produce the sum if all the elements are Ok,
/// or return the first error
///
/// Note: the implementation of [`Sum`] for T must
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
///
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
