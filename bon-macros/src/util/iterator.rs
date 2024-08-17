pub(crate) trait IteratorExt: Iterator + Sized {
    fn try_collect<U, E>(self) -> Result<U, E>
    where
        Result<U, E>: FromIterator<Self::Item>,
    {
        self.collect()
    }
}

impl<I: Iterator> IteratorExt for I {}

pub(crate) trait IntoIteratorExt: IntoIterator + Sized {
    fn equals_with<O>(self, other: O, compare: impl Fn(Self::Item, O::Item) -> bool) -> bool
    where
        O: IntoIterator,
        O::IntoIter: ExactSizeIterator,
        Self::IntoIter: ExactSizeIterator,
    {
        let me = self.into_iter();
        let other = other.into_iter();

        if me.len() != other.len() {
            return false;
        }

        me.zip(other).all(|(a, b)| compare(a, b))
    }
}

impl<I: IntoIterator> IntoIteratorExt for I {}
