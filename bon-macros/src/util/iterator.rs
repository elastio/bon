pub(crate) trait IteratorExt: Iterator + Sized {
    fn try_collect<U, E>(self) -> Result<U, E>
    where
        Result<U, E>: FromIterator<Self::Item>,
    {
        self.collect()
    }
}

impl<I: Iterator> IteratorExt for I {}
