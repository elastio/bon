#![allow(unsafe_code, dead_code, unreachable_pub, dropping_copy_types)]

// use bon::builder;
use std::hint::black_box;

pub fn regular_bench() -> u32 {
    regular(
        black_box("4"),
        black_box(24),
        black_box(true),
        black_box(Some("5".to_string())),
        black_box(Some(6)),
        // black_box(vec![7, 8, 9]),
        // black_box((10, 11)),
        // black_box([12, 13, 14]),
        // black_box("15".to_string()),
        // black_box("16".to_string().into_boxed_str()),
    )
}

pub fn builder_bench() -> u32 {
    builder()
        .arg1(black_box("4"))
        .arg2(black_box(24))
        .arg3(black_box(true))
        .maybe_arg4(black_box(Some("5".to_string())))
        .maybe_arg5(black_box(Some(6)))
        .call()

    // builder()
    //     .arg1(black_box("4"))
    //     .arg2(black_box(24))
    //     .arg3(black_box(true))
    //     .maybe_arg4(black_box(Some("5".to_string())))
    //     .maybe_arg5(black_box(Some(6)))
    //     // .arg6(black_box(vec![7, 8, 9]))
    //     // .arg7(black_box((10, 11)))
    //     // .arg8(black_box([12, 13, 14]))
    //     // .arg9(black_box("15".to_string()))
    //     // .arg10(black_box("16".to_string().into_boxed_str()))
    //     .call()
}

fn regular(
    arg1: &str,
    arg2: u32,
    arg3: bool,
    arg4: Option<String>,
    arg5: Option<u32>,
    // arg6: Vec<u32>,
    // arg7: (u32, u32),
    // arg8: [u32; 3],
    // arg9: String,
    // arg10: Box<str>,
) -> u32 {
    let x = arg1.parse::<u32>().unwrap() + arg2;
    let x = x + u32::from(arg3);
    let x = x + arg4.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg5.unwrap_or(0);
    // let x = x + arg6.iter().sum::<u32>();
    // let x = x + arg7.0 + arg7.1 + arg8.iter().sum::<u32>();
    // let x = x + arg9.parse::<u32>().unwrap();
    // let x = x + arg10.parse::<u32>().unwrap();
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

trait NoField<T> {}

impl<'a, F: NonArg1, Inner: NoField<Arg1<'a>>> NoField<Arg1<'a>> for Builder<F, Inner> {}
impl<F: NonArg2, Inner: NoField<Arg2>> NoField<Arg2> for Builder<F, Inner> {}
impl<F: NonArg3, Inner: NoField<Arg3>> NoField<Arg3> for Builder<F, Inner> {}
impl<F: NonArg4, Inner: NoField<Arg4>> NoField<Arg4> for Builder<F, Inner> {}
impl<F: NonArg5, Inner: NoField<Arg5>> NoField<Arg5> for Builder<F, Inner> {}

impl NoField<Arg1<'_>> for Builder<Nothing, Nothing> {}
impl NoField<Arg2> for Builder<Nothing, Nothing> {}
impl NoField<Arg3> for Builder<Nothing, Nothing> {}
impl NoField<Arg4> for Builder<Nothing, Nothing> {}
impl NoField<Arg5> for Builder<Nothing, Nothing> {}

impl<'a, F, Inner> Builder<F, Inner>
where
    Self:
        TakeField<Arg1<'a>> + TakeField<Arg2> + TakeField<Arg3> + TakeField<Arg4> + TakeField<Arg5>,
{
    fn call(mut self) -> u32 {
        unsafe {
            regular(
                <Self as TakeField<Arg1<'a>>>::take_field(&mut self).0,
                <Self as TakeField<Arg2>>::take_field(&mut self).0,
                <Self as TakeField<Arg3>>::take_field(&mut self).0,
                <Self as TakeField<Arg4>>::take_field(&mut self).0,
                <Self as TakeField<Arg5>>::take_field(&mut self).0,
            )
        }
    }
}

struct Nothing;
impl NonArg1 for Nothing {}
impl NonArg2 for Nothing {}
impl NonArg3 for Nothing {}
impl NonArg4 for Nothing {}
impl NonArg5 for Nothing {}

struct Arg1<'a>(&'a str);
trait NonArg1 {}
impl NonArg1 for Arg2 {}
impl NonArg1 for Arg3 {}
impl NonArg1 for Arg4 {}
impl NonArg1 for Arg5 {}

struct Arg2(u32);
trait NonArg2 {}
impl NonArg2 for Arg1<'_> {}
impl NonArg2 for Arg3 {}
impl NonArg2 for Arg4 {}
impl NonArg2 for Arg5 {}

struct Arg3(bool);
trait NonArg3 {}
impl NonArg3 for Arg1<'_> {}
impl NonArg3 for Arg2 {}
impl NonArg3 for Arg4 {}
impl NonArg3 for Arg5 {}

struct Arg4(Option<String>);
trait NonArg4 {}
impl NonArg4 for Arg1<'_> {}
impl NonArg4 for Arg2 {}
impl NonArg4 for Arg3 {}
impl NonArg4 for Arg5 {}

struct Arg5(Option<u32>);
trait NonArg5 {}
impl NonArg5 for Arg1<'_> {}
impl NonArg5 for Arg2 {}
impl NonArg5 for Arg3 {}
impl NonArg5 for Arg4 {}
trait TakeField<T> {
    unsafe fn take_field(&mut self) -> T;
}

// impl<T, Inner> TakeField<T> for Builder<T, Inner> {
// #[inline(always)]
//    unsafe fn take_field(self) -> T {
//         self.field.assume_init()
//     }
// }

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
