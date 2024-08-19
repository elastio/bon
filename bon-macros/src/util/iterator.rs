use std::fmt::Write;

pub(crate) trait IteratorExt: Iterator + Sized {
    fn try_collect<U, E>(self) -> Result<U, E>
    where
        Result<U, E>: FromIterator<Self::Item>,
    {
        self.collect()
    }

    /// Based on itertools:
    /// <https://github.com/rust-itertools/itertools/blob/a4a82e4b97eb76687c2a57cdfd2e5343ff507827/src/lib.rs#L2301-L2330>
    fn join(mut self, sep: &str) -> String
    where
        Self::Item: std::fmt::Display,
    {
        let Some(first) = self.next() else {
            return String::new();
        };

        // estimate lower bound of capacity needed
        let (lower, _) = self.size_hint();
        let mut result = String::with_capacity(sep.len() * lower);

        write!(&mut result, "{}", first).unwrap();

        for elt in self {
            result.push_str(sep);
            write!(&mut result, "{}", elt).unwrap();
        }

        result
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

    fn concat(self) -> Self::Item
    where
        Self::Item: Extend<<Self::Item as IntoIterator>::Item> + IntoIterator + Default,
    {
        self.into_iter()
            .reduce(|mut a, b| {
                a.extend(b);
                a
            })
            .unwrap_or_default()
    }
}

impl<I: IntoIterator> IntoIteratorExt for I {}
