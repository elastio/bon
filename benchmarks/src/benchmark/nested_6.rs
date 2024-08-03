#![allow(unsafe_code, dead_code, unreachable_pub, dropping_copy_types)]
use std::hint::black_box;

#[inline(never)]
pub fn regular_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5, arg6) =
        black_box(("4", 24, true, Some("5"), Some(6), "6".to_owned()));

    regular(arg1, arg2, arg3, arg4, arg5, arg6)
}

pub fn builder_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5, arg6) =
        black_box(("4", 24, true, Some("5"), Some(6), "6".to_owned()));

    builder()
        .arg1(arg1)
        .arg2(arg2)
        .arg3(arg3)
        .maybe_arg4(arg4)
        .maybe_arg5(arg5)
        .arg6(arg6)
        .call()

    // let mut this = Builder {
    //     inner: {
    //         Builder {
    //             inner: {
    //                 Builder {
    //                     inner: {
    //                         Builder {
    //                             inner: {
    //                                 Builder {
    //                                     inner: builder(),
    //                                     field: Some(Arg1(arg1)),
    //                                 }
    //                             },
    //                             field: Some(Arg2(arg2)),
    //                         }
    //                     },
    //                     field: Some(Arg3(arg3)),
    //                 }
    //             },
    //             field: Some(Arg4(arg4)),
    //         }
    //     },
    //     field: Some(Arg5(arg5)),
    // };

    // let (end_arg1, end_arg2, end_arg3, end_arg4, end_arg5) = (
    //     unsafe { TakeField::<Arg1<'_>>::take_field(&mut this).0 },
    //     unsafe { TakeField::<Arg2>::take_field(&mut this).0 },
    //     unsafe { TakeField::<Arg3>::take_field(&mut this).0 },
    //     unsafe { TakeField::<Arg4>::take_field(&mut this).0 },
    //     unsafe { TakeField::<Arg5>::take_field(&mut this).0 },
    // );

    // regular(end_arg1, end_arg2, end_arg3, end_arg4, end_arg5)

    // regular(
    //     unsafe { TakeField::<Arg1<'_>>::take_field(&mut this).0 },
    //     unsafe { TakeField::<Arg2>::take_field(&mut this).0 },
    //     unsafe { TakeField::<Arg3>::take_field(&mut this).0 },
    //     unsafe { TakeField::<Arg4>::take_field(&mut this).0 },
    //     unsafe { TakeField::<Arg5>::take_field(&mut this).0 },
    // )
}

fn regular(
    arg1: &str,
    arg2: u32,
    arg3: bool,
    arg4: Option<&str>,
    arg5: Option<u32>,
    arg6: String,
) -> u32 {
    let x = arg1.parse::<u32>().unwrap() + arg2;
    let x = x + u32::from(arg3);
    let x = x + arg4.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg5.unwrap_or(0);
    let x = x + arg6.parse::<u32>().unwrap();
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

impl<'a, F, Inner> Builder<F, Inner>
where
    Self: NoField<Arg4<'a>>,
{
    #[inline(always)]
    fn maybe_arg4(self, value: Option<&'a str>) -> Builder<Arg4<'a>, Self> {
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
    fn arg6(self, value: String) -> Builder<Arg6, Self> {
        Builder {
            inner: self,
            field: Some(Arg6(value)),
        }
    }
}

trait NoField<T> {}

impl<'a, F: NonArg1, Inner: NoField<Arg1<'a>>> NoField<Arg1<'a>> for Builder<F, Inner> {}
impl<F: NonArg2, Inner: NoField<Arg2>> NoField<Arg2> for Builder<F, Inner> {}
impl<F: NonArg3, Inner: NoField<Arg3>> NoField<Arg3> for Builder<F, Inner> {}
impl<'a, F: NonArg4, Inner: NoField<Arg4<'a>>> NoField<Arg4<'a>> for Builder<F, Inner> {}
impl<F: NonArg5, Inner: NoField<Arg5>> NoField<Arg5> for Builder<F, Inner> {}
impl<F: NonArg6, Inner: NoField<Arg6>> NoField<Arg6> for Builder<F, Inner> {}

impl NoField<Arg1<'_>> for Builder<Nothing, Nothing> {}
impl NoField<Arg2> for Builder<Nothing, Nothing> {}
impl NoField<Arg3> for Builder<Nothing, Nothing> {}
impl NoField<Arg4<'_>> for Builder<Nothing, Nothing> {}
impl NoField<Arg5> for Builder<Nothing, Nothing> {}
impl NoField<Arg6> for Builder<Nothing, Nothing> {}

impl<'a, F, Inner> Builder<F, Inner>
where
    Self: TakeField<Arg1<'a>>
        + TakeField<Arg2>
        + TakeField<Arg3>
        + TakeField<Arg4<'a>>
        + TakeField<Arg5>
        + TakeField<Arg6>,
{
    #[inline(always)]
    fn call(mut self) -> u32 {
        let me = &mut self;
        let arg1 = unsafe { TakeField::<Arg1<'a>>::take_field(me).0 };
        let arg2 = unsafe { TakeField::<Arg2>::take_field(me).0 };
        let arg3 = unsafe { TakeField::<Arg3>::take_field(me).0 };
        let arg4 = unsafe { TakeField::<Arg4<'a>>::take_field(me).0 };
        let arg5 = unsafe { TakeField::<Arg5>::take_field(me).0 };
        let arg6 = unsafe { TakeField::<Arg6>::take_field(me).0 };

        regular(arg1, arg2, arg3, arg4, arg5, arg6)
    }
}

struct Nothing;
impl NonArg1 for Nothing {}
impl NonArg2 for Nothing {}
impl NonArg3 for Nothing {}
impl NonArg4 for Nothing {}
impl NonArg5 for Nothing {}
impl NonArg6 for Nothing {}

struct Arg1<'a>(&'a str);
trait NonArg1 {}
impl NonArg1 for Arg2 {}
impl NonArg1 for Arg3 {}
impl NonArg1 for Arg4<'_> {}
impl NonArg1 for Arg5 {}
impl NonArg1 for Arg6 {}

struct Arg2(u32);
trait NonArg2 {}
impl NonArg2 for Arg1<'_> {}
impl NonArg2 for Arg3 {}
impl NonArg2 for Arg4<'_> {}
impl NonArg2 for Arg5 {}
impl NonArg2 for Arg6 {}

struct Arg3(bool);
trait NonArg3 {}
impl NonArg3 for Arg1<'_> {}
impl NonArg3 for Arg2 {}
impl NonArg3 for Arg4<'_> {}
impl NonArg3 for Arg5 {}
impl NonArg3 for Arg6 {}

struct Arg4<'a>(Option<&'a str>);
trait NonArg4 {}
impl NonArg4 for Arg1<'_> {}
impl NonArg4 for Arg2 {}
impl NonArg4 for Arg3 {}
impl NonArg4 for Arg5 {}
impl NonArg4 for Arg6 {}

struct Arg5(Option<u32>);
trait NonArg5 {}
impl NonArg5 for Arg1<'_> {}
impl NonArg5 for Arg2 {}
impl NonArg5 for Arg3 {}
impl NonArg5 for Arg4<'_> {}
impl NonArg5 for Arg6 {}

struct Arg6(String);
trait NonArg6 {}
impl NonArg6 for Arg1<'_> {}
impl NonArg6 for Arg2 {}
impl NonArg6 for Arg3 {}
impl NonArg6 for Arg4<'_> {}
impl NonArg6 for Arg5 {}

trait TakeField<T> {
    unsafe fn take_field(&mut self) -> T;
}

impl<'a, Inner> TakeField<Arg1<'a>> for Builder<Arg1<'a>, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg1<'a> {
        self.field.take().unwrap()
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
        self.field.take().unwrap()
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
        self.field.take().unwrap()
    }
}

impl<T: NonArg3, Inner: TakeField<Arg3>> TakeField<Arg3> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg3 {
        self.inner.take_field()
    }
}

impl<'a, Inner> TakeField<Arg4<'a>> for Builder<Arg4<'a>, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg4<'a> {
        self.field.take().unwrap()
    }
}

impl<'a, T: NonArg4, Inner: TakeField<Arg4<'a>>> TakeField<Arg4<'a>> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg4<'a> {
        self.inner.take_field()
    }
}

impl<Inner> TakeField<Arg5> for Builder<Arg5, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg5 {
        self.field.take().unwrap()
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
        self.field.take().unwrap()
    }
}

impl<T: NonArg6, Inner: TakeField<Arg6>> TakeField<Arg6> for Builder<T, Inner> {
    #[inline(always)]
    unsafe fn take_field(&mut self) -> Arg6 {
        self.inner.take_field()
    }
}
