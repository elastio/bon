mod multi_arg;
mod single_arg;
mod overwritable;

struct IntoStrRef<'a>(&'a str);

impl<'a> From<IntoStrRef<'a>> for &'a str {
    fn from(val: IntoStrRef<'a>) -> Self {
        val.0
    }
}
