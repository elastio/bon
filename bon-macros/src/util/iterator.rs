use crate::util::prelude::*;
use std::fmt::Write;
use std::iter::Peekable;

pub(crate) trait IteratorExt: Iterator + Sized {
    /// Based on itertools:
    /// <https://github.com/rust-itertools/itertools/blob/a4a82e4b97eb76687c2a57cdfd2e5343ff507827/src/lib.rs#L2301-L2330>
    fn join(mut self, sep: &str) -> String
    where
        Self::Item: std::fmt::Display,
    {
        let first = match self.next() {
            Some(first) => first,
            _ => return String::new(),
        };

        // estimate lower bound of capacity needed
        let (lower, _) = self.size_hint();
        let mut result = String::with_capacity(sep.len() * lower);

        write!(&mut result, "{first}").unwrap();

        for elt in self {
            result.push_str(sep);
            write!(&mut result, "{elt}").unwrap();
        }

        result
    }
}

impl<I: Iterator> IteratorExt for I {}

pub(crate) trait IntoIteratorExt: IntoIterator + Sized {
    fn try_equals_with<O>(
        self,
        other: O,
        compare: impl Fn(Self::Item, O::Item) -> Result<bool>,
    ) -> Result<bool>
    where
        O: IntoIterator,
        O::IntoIter: ExactSizeIterator,
        Self::IntoIter: ExactSizeIterator,
    {
        let me = self.into_iter();
        let other = other.into_iter();

        if me.len() != other.len() {
            return Ok(false);
        }

        for (a, b) in me.zip(other) {
            if !compare(a, b)? {
                return Ok(false);
            }
        }

        Ok(true)
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

pub(crate) trait PeekableExt: Iterator {
    fn peeking_take_while(
        &mut self,
        predicate: impl FnMut(&Self::Item) -> bool,
    ) -> impl Iterator<Item = Self::Item>;
}

impl<I: Iterator> PeekableExt for Peekable<I> {
    fn peeking_take_while(
        &mut self,
        mut predicate: impl FnMut(&Self::Item) -> bool,
    ) -> impl Iterator<Item = Self::Item> {
        std::iter::from_fn(move || {
            if !predicate(self.peek()?) {
                return None;
            }

            self.next()
        })
    }
}
