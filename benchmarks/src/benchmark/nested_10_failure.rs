use std::hint::black_box;

pub fn regular_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) = black_box((
        "4",
        24,
        true,
        Some("5".to_string()),
        Some(6),
        vec![1, 2, 43, 65],
        (10, 11),
        [12, 13, 14],
        "15".to_string(),
        "16".to_string().into_boxed_str(),
    ));

    regular(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub fn builder_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) = black_box((
        "4",
        24,
        true,
        Some("5".to_string()),
        Some(6),
        vec![1, 2, 43, 65],
        (10, 11),
        [12, 13, 14],
        "15".to_string(),
        "16".to_string().into_boxed_str(),
    ));

    {
        let mut this = builder()
            .arg1(arg1)
            .arg2(arg2)
            .arg3(arg3)
            .maybe_arg4(arg4)
            .maybe_arg5(arg5)
            .arg6(arg6)
            .arg7(arg7)
            .arg8(arg8)
            .arg9(arg9)
            .arg10(arg10);
        let arg_1 = unsafe { TakeField::<Arg1<'_>>::take_field(&mut this).0 };
        let arg_2 = unsafe { TakeField::<Arg2>::take_field(&mut this).0 };
        let arg_3 = unsafe { TakeField::<Arg3>::take_field(&mut this).0 };
        let arg_4 = unsafe { TakeField::<Arg4>::take_field(&mut this).0 };
        let arg_5 = unsafe { TakeField::<Arg5>::take_field(&mut this).0 };
        let arg_6 = unsafe { TakeField::<Arg6>::take_field(&mut this).0 };
        let arg_7 = unsafe { TakeField::<Arg7>::take_field(&mut this).0 };
        let arg_8 = unsafe { TakeField::<Arg8>::take_field(&mut this).0 };
        let arg_9 = unsafe { TakeField::<Arg9>::take_field(&mut this).0 };
        let arg_10 = unsafe { TakeField::<Arg10>::take_field(&mut this).0 };

        regular(
            arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, arg_9, arg_10,
        )
    }
}

fn regular(
    arg1: &str,
    arg2: u32,
    arg3: bool,
    arg4: Option<String>,
    arg5: Option<u32>,
    arg6: Vec<u32>,
    arg7: (u32, u32),
    arg8: [u32; 3],
    arg9: String,
    arg10: Box<str>,
) -> u32 {
    let x = arg1.parse::<u32>().unwrap() + arg2;
    let x = x + u32::from(arg3);
    let x = x + arg4.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg5.unwrap_or(0);
    let x = x + arg6.iter().sum::<u32>();
    let x = x + arg7.0 + arg7.1 + arg8.iter().sum::<u32>();
    let x = x + arg9.parse::<u32>().unwrap();
    let x = x + arg10.parse::<u32>().unwrap();
    x
}

#[inline(always)]
fn builder() -> Builder<Nothing, Nothing> {
    Builder {
        inner: Nothing,
        field: Some(Nothing),
    }
}

struct Builder<F, Inner> {
    inner: Inner,
    field: Option<F>,
}

impl<'a, F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg1<'a>>,
{
    #[inline(always)]
    fn arg1(self, value: &'a str) -> Builder<Arg1<'a>, Self> {
        Builder {
            inner: self,
            field: Some(Arg1(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg2>,
{
    #[inline(always)]
    fn arg2(self, value: u32) -> Builder<Arg2, Self> {
        Builder {
            inner: self,
            field: Some(Arg2(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg3>,
{
    #[inline(always)]
    fn arg3(self, value: bool) -> Builder<Arg3, Self> {
        Builder {
            inner: self,
            field: Some(Arg3(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg4>,
{
    #[inline(always)]
    fn maybe_arg4(self, value: Option<String>) -> Builder<Arg4, Self> {
        Builder {
            inner: self,
            field: Some(Arg4(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg5>,
{
    #[inline(always)]
    fn maybe_arg5(self, value: Option<u32>) -> Builder<Arg5, Self> {
        Builder {
            inner: self,
            field: Some(Arg5(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg6>,
{
    #[inline(always)]
    fn arg6(self, value: Vec<u32>) -> Builder<Arg6, Self> {
        Builder {
            inner: self,
            field: Some(Arg6(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg7>,
{
    #[inline(always)]
    fn arg7(self, value: (u32, u32)) -> Builder<Arg7, Self> {
        Builder {
            inner: self,
            field: Some(Arg7(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg8>,
{
    #[inline(always)]
    fn arg8(self, value: [u32; 3]) -> Builder<Arg8, Self> {
        Builder {
            inner: self,
            field: Some(Arg8(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg9>,
{
    #[inline(always)]
    fn arg9(self, value: String) -> Builder<Arg9, Self> {
        Builder {
            inner: self,
            field: Some(Arg9(value)),
        }
    }
}

impl<F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg10>,
{
    #[inline(always)]
    fn arg10(self, value: Box<str>) -> Builder<Arg10, Self> {
        Builder {
            inner: self,
            field: Some(Arg10(value)),
        }
    }
}

trait NoField<T> {}

impl<'a, F: NonArg1, Inner: NoField<Arg1<'a>>> NoField<Arg1<'a>> for Builder<F, Inner> {}
impl<F: NonArg2, Inner: NoField<Arg2>> NoField<Arg2> for Builder<F, Inner> {}
impl<F: NonArg3, Inner: NoField<Arg3>> NoField<Arg3> for Builder<F, Inner> {}
impl<F: NonArg4, Inner: NoField<Arg4>> NoField<Arg4> for Builder<F, Inner> {}
impl<F: NonArg5, Inner: NoField<Arg5>> NoField<Arg5> for Builder<F, Inner> {}
impl<F: NonArg6, Inner: NoField<Arg6>> NoField<Arg6> for Builder<F, Inner> {}
impl<F: NonArg7, Inner: NoField<Arg7>> NoField<Arg7> for Builder<F, Inner> {}
impl<F: NonArg8, Inner: NoField<Arg8>> NoField<Arg8> for Builder<F, Inner> {}
impl<F: NonArg9, Inner: NoField<Arg9>> NoField<Arg9> for Builder<F, Inner> {}
impl<F: NonArg10, Inner: NoField<Arg10>> NoField<Arg10> for Builder<F, Inner> {}

impl NoField<Arg1<'_>> for Builder<Nothing, Nothing> {}
impl NoField<Arg2> for Builder<Nothing, Nothing> {}
impl NoField<Arg3> for Builder<Nothing, Nothing> {}
impl NoField<Arg4> for Builder<Nothing, Nothing> {}
impl NoField<Arg5> for Builder<Nothing, Nothing> {}
impl NoField<Arg6> for Builder<Nothing, Nothing> {}
impl NoField<Arg7> for Builder<Nothing, Nothing> {}
impl NoField<Arg8> for Builder<Nothing, Nothing> {}
impl NoField<Arg9> for Builder<Nothing, Nothing> {}
impl NoField<Arg10> for Builder<Nothing, Nothing> {}

impl<'a, F, Inner> Builder<F, Inner>
where
    Self: TakeField<Arg1<'a>>
        + TakeField<Arg2>
        + TakeField<Arg3>
        + TakeField<Arg4>
        + TakeField<Arg5>
        + TakeField<Arg6>
        + TakeField<Arg7>
        + TakeField<Arg8>
        + TakeField<Arg9>
        + TakeField<Arg10>,
{
    #[inline(always)]
    fn call(mut self) -> u32 {
        let arg_1 = unsafe { TakeField::<Arg1<'a>>::take_field(&mut self).0 };
        let arg_2 = unsafe { TakeField::<Arg2>::take_field(&mut self).0 };
        let arg_3 = unsafe { TakeField::<Arg3>::take_field(&mut self).0 };
        let arg_4 = unsafe { TakeField::<Arg4>::take_field(&mut self).0 };
        let arg_5 = unsafe { TakeField::<Arg5>::take_field(&mut self).0 };
        let arg_6 = unsafe { TakeField::<Arg6>::take_field(&mut self).0 };
        let arg_7 = unsafe { TakeField::<Arg7>::take_field(&mut self).0 };
        let arg_8 = unsafe { TakeField::<Arg8>::take_field(&mut self).0 };
        let arg_9 = unsafe { TakeField::<Arg9>::take_field(&mut self).0 };
        let arg_10 = unsafe { TakeField::<Arg10>::take_field(&mut self).0 };

        regular(
            arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, arg_9, arg_10,
        )
    }
}

struct Nothing;
impl NonArg1 for Nothing {}
impl NonArg2 for Nothing {}
impl NonArg3 for Nothing {}
impl NonArg4 for Nothing {}
impl NonArg5 for Nothing {}
impl NonArg6 for Nothing {}
impl NonArg7 for Nothing {}
impl NonArg8 for Nothing {}
impl NonArg9 for Nothing {}
impl NonArg10 for Nothing {}

struct Arg1<'a>(&'a str);
trait NonArg1 {}
impl NonArg1 for Arg2 {}
impl NonArg1 for Arg3 {}
impl NonArg1 for Arg4 {}
impl NonArg1 for Arg5 {}
impl NonArg1 for Arg6 {}
impl NonArg1 for Arg7 {}
impl NonArg1 for Arg8 {}
impl NonArg1 for Arg9 {}
impl NonArg1 for Arg10 {}

struct Arg2(u32);
trait NonArg2 {}
impl NonArg2 for Arg1<'_> {}
impl NonArg2 for Arg3 {}
impl NonArg2 for Arg4 {}
impl NonArg2 for Arg5 {}
impl NonArg2 for Arg6 {}
impl NonArg2 for Arg7 {}
impl NonArg2 for Arg8 {}
impl NonArg2 for Arg9 {}
impl NonArg2 for Arg10 {}

struct Arg3(bool);
trait NonArg3 {}
impl NonArg3 for Arg1<'_> {}
impl NonArg3 for Arg2 {}
impl NonArg3 for Arg4 {}
impl NonArg3 for Arg5 {}
impl NonArg3 for Arg6 {}
impl NonArg3 for Arg7 {}
impl NonArg3 for Arg8 {}
impl NonArg3 for Arg9 {}
impl NonArg3 for Arg10 {}

struct Arg4(Option<String>);
trait NonArg4 {}
impl NonArg4 for Arg1<'_> {}
impl NonArg4 for Arg2 {}
impl NonArg4 for Arg3 {}
impl NonArg4 for Arg5 {}
impl NonArg4 for Arg6 {}
impl NonArg4 for Arg7 {}
impl NonArg4 for Arg8 {}
impl NonArg4 for Arg9 {}
impl NonArg4 for Arg10 {}

struct Arg5(Option<u32>);
trait NonArg5 {}
impl NonArg5 for Arg1<'_> {}
impl NonArg5 for Arg2 {}
impl NonArg5 for Arg3 {}
impl NonArg5 for Arg4 {}
impl NonArg5 for Arg6 {}
impl NonArg5 for Arg7 {}
impl NonArg5 for Arg8 {}
impl NonArg5 for Arg9 {}
impl NonArg5 for Arg10 {}

struct Arg6(Vec<u32>);
trait NonArg6 {}
impl NonArg6 for Arg1<'_> {}
impl NonArg6 for Arg2 {}
impl NonArg6 for Arg3 {}
impl NonArg6 for Arg4 {}
impl NonArg6 for Arg5 {}
impl NonArg6 for Arg7 {}
impl NonArg6 for Arg8 {}
impl NonArg6 for Arg9 {}
impl NonArg6 for Arg10 {}

struct Arg7((u32, u32));
trait NonArg7 {}
impl NonArg7 for Arg1<'_> {}
impl NonArg7 for Arg2 {}
impl NonArg7 for Arg3 {}
impl NonArg7 for Arg4 {}
impl NonArg7 for Arg5 {}
impl NonArg7 for Arg6 {}
impl NonArg7 for Arg8 {}
impl NonArg7 for Arg9 {}
impl NonArg7 for Arg10 {}

struct Arg8([u32; 3]);
trait NonArg8 {}
impl NonArg8 for Arg1<'_> {}
impl NonArg8 for Arg2 {}
impl NonArg8 for Arg3 {}
impl NonArg8 for Arg4 {}
impl NonArg8 for Arg5 {}
impl NonArg8 for Arg6 {}
impl NonArg8 for Arg7 {}
impl NonArg8 for Arg9 {}
impl NonArg8 for Arg10 {}

struct Arg9(String);
trait NonArg9 {}
impl NonArg9 for Arg1<'_> {}
impl NonArg9 for Arg2 {}
impl NonArg9 for Arg3 {}
impl NonArg9 for Arg4 {}
impl NonArg9 for Arg5 {}
impl NonArg9 for Arg6 {}
impl NonArg9 for Arg7 {}
impl NonArg9 for Arg8 {}
impl NonArg9 for Arg10 {}

struct Arg10(Box<str>);
trait NonArg10 {}
impl NonArg10 for Arg1<'_> {}
impl NonArg10 for Arg2 {}
impl NonArg10 for Arg3 {}
impl NonArg10 for Arg4 {}
impl NonArg10 for Arg5 {}
impl NonArg10 for Arg6 {}
impl NonArg10 for Arg7 {}
impl NonArg10 for Arg8 {}
impl NonArg10 for Arg9 {}

trait TakeField<T> {
    unsafe fn take_field(&mut self) -> T;
}

impl<'a, Inner> TakeField<Arg1<'a>> for Builder<Arg1<'a>, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg1<'a> {
        self.field.take().unwrap_unchecked()
    }
}

impl<'a, T: NonArg1, Inner: TakeField<Arg1<'a>>> TakeField<Arg1<'a>> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg1<'a> {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg2> for Builder<Arg2, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg2 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg2, Inner: TakeField<Arg2>> TakeField<Arg2> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg2 {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg3> for Builder<Arg3, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg3 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg3, Inner: TakeField<Arg3>> TakeField<Arg3> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg3 {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg4> for Builder<Arg4, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg4 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg4, Inner: TakeField<Arg4>> TakeField<Arg4> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg4 {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg5> for Builder<Arg5, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg5 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg5, Inner: TakeField<Arg5>> TakeField<Arg5> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg5 {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg6> for Builder<Arg6, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg6 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg6, Inner: TakeField<Arg6>> TakeField<Arg6> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg6 {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg7> for Builder<Arg7, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg7 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg7, Inner: TakeField<Arg7>> TakeField<Arg7> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg7 {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg8> for Builder<Arg8, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg8 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg8, Inner: TakeField<Arg8>> TakeField<Arg8> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg8 {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg9> for Builder<Arg9, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg9 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg9, Inner: TakeField<Arg9>> TakeField<Arg9> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg9 {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg10> for Builder<Arg10, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg10 {
        self.field.take().unwrap_unchecked()
    }
}

impl<T: NonArg10, Inner: TakeField<Arg10>> TakeField<Arg10> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg10 {
        self.inner.take_field()
    }
}
