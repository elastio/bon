mod multi_arg;
mod overwritable;
mod single_arg;
mod some;
mod from_iter;

struct IntoStrRef<'a>(&'a str);

impl<'a> From<IntoStrRef<'a>> for &'a str {
    fn from(val: IntoStrRef<'a>) -> Self {
        val.0
    }
}
