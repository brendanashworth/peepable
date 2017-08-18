//! A Rust look-alike for Peekable that allows peeping into immutable references.

use std::iter::Iterator;
use std::mem;

/// peepable is a Rust look-alike for
/// [`Peekable`](https://doc.rust-lang.org/std/iter/struct.Peekable.html).
///
/// It behaves slightly different as it eagerly loads the next value in the Iterator.
/// This allows `.peep()` to be called on an immutable reference, saving you from the
/// borrow checker.
///
/// # Examples
///
/// ```
/// use std::iter::Iterator;
/// use peepable::Peepable;
///
/// let mut iter = vec![1, 2, 3].into_iter();
///
/// // Note, this is not "mut peeper"!
/// let peeper = Peepable::new(iter);
///
/// assert_eq!(peeper.peep(), Some(&1));
///
/// // When mutable, we can use it as a normal iterator.
/// let mut peeper = peeper;
///
/// assert_eq!(peeper.next(), Some(1));
///
/// assert_eq!(peeper.peep(), Some(&2));
/// assert_eq!(peeper.next(), Some(2));
///
/// assert_eq!(peeper.next(), Some(3));
///
/// assert_eq!(peeper.peep(), None);
/// assert_eq!(peeper.next(), None);
/// ```
pub struct Peepable<I>
where
    I: Iterator,
{
    /// The underlying iterator for the Peepable.
    iter: I,

    /// The next item in the iterator. Because we're eager, this will
    /// always have a value. Peeking returns a reference to this value,
    /// and next shifts this off and replaces it with a new value.
    next: Option<I::Item>,
}

impl<I: Iterator> Iterator for Peepable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Load the iterator's next value, swap it with the peeked one,
        // and return the peeked value.
        let mut next = self.iter.next();

        mem::swap(&mut next, &mut self.next);

        next
    }
}

impl<I: Iterator> Peepable<I> {
    /// Consumes a given Iterator into a Peepable<Iterator>.
    ///
    /// The given Peepable implements Iterator and can be used as such.
    ///
    /// ```
    /// use peepable::Peepable;
    ///
    /// let iter = vec![1, 5, 10].into_iter();
    ///
    /// let peeper = Peepable::new(iter);
    /// ```
    pub fn new(mut iter: I) -> Peepable<I> {
        let next = iter.next();

        Peepable {
            iter: iter,
            next: next,
        }
    }

    /// Peeps into the iterator, giving a reference to the next item.
    ///
    /// This only takes a reference (doesn't need mutable), and the given
    /// reference to the next item is immutable.
    ///
    /// ```
    /// use peepable::Peepable;
    ///
    /// let iter = Peepable::new((0..5));
    ///
    /// assert_eq!(iter.peep(), Some(&0));
    /// ```
    pub fn peep(&self) -> Option<&I::Item> {
        match self.next {
            Some(ref next) => Some(next),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Peepable;

    #[test]
    fn basic_peeping() {
        let vec = vec![1, 2, 3];

        let peeper: Peepable<_> = Peepable::new(vec.into_iter());

        assert_eq!(peeper.peep(), Some(&1));

        let mut peeper = peeper;

        assert_eq!(peeper.peep(), Some(&1));
        assert_eq!(peeper.peep(), Some(&1));
        assert_eq!(peeper.next(), Some(1));

        assert_eq!(peeper.peep(), Some(&2));
        assert_eq!(peeper.next(), Some(2));

        assert_eq!(peeper.next(), Some(3));

        assert_eq!(peeper.next(), None);
        assert_eq!(peeper.peep(), None);
    }

    #[test]
    fn basic_iterator() {
        let vec = vec![1, 2, 3];

        let mut peepable = Peepable::new(vec.into_iter());

        assert_eq!(peepable.next(), Some(1));
        assert_eq!(peepable.next(), Some(2));
        assert_eq!(peepable.next(), Some(3));
        assert_eq!(peepable.next(), None);
        assert_eq!(peepable.next(), None);
    }

    #[test]
    fn has_iterator_tools() {
        let iter = Peepable::new((0..4));

        let sum: u8 = iter.filter(|x| x > &1).map(|x| x * x).sum();

        assert_eq!(sum, 13);
    }
}
