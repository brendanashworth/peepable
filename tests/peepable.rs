// Just test to make sure we can access it with extern crate, basically.
extern crate peepable;

#[cfg(test)]
mod tests {
    use std::iter::Iterator;
    use peepable::Peepable;

    #[test]
    fn peep_correctly() {
        let iter = 1..5;

        let peeper = Peepable::new(iter);

        // Peeping multiple times does not advance.
        // Can peep on an immutable variable.
        assert_eq!(peeper.peep(), Some(&1));
        assert_eq!(peeper.peep(), Some(&1));

        let mut peeper = peeper;

        // Sweet, now it should work as any other iterator.
        assert_eq!(peeper.peep(), Some(&1));
        assert_eq!(peeper.next(), Some(1));

        assert_eq!(peeper.next(), Some(2));

        assert_eq!(peeper.peep(), Some(&3));
        assert_eq!(peeper.next(), Some(3));

        assert_eq!(peeper.peep(), Some(&4));
        assert_eq!(peeper.next(), Some(4));

        assert_eq!(peeper.peep(), None);
        assert_eq!(peeper.peep(), None);
        assert_eq!(peeper.next(), None);
    }
}
