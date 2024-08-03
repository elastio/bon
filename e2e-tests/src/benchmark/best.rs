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
    Builder {
        field: Some(Arg5(black_box(Some(6)))),
        inner: Builder {
            field: Some(Arg4(black_box(Some("5".to_string())))),
            inner: Builder {
                field: Some(Arg3(black_box(true))),
                inner: Builder {
                    field: Some(Arg2(black_box(24))),
                    inner: Builder {
                        field: Some(Arg1(black_box("4"))),
                        inner: (),
                    },
                },
            },
        },
    }
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

#[inline(always)]
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

// fn builder() -> Builder<(), ()> {}

struct Builder<F, Inner> {
    inner: Inner,
    field: Option<F>,
}

impl Builder<Arg5, Builder<Arg4, Builder<Arg3, Builder<Arg2, Builder<Arg1<'_>, ()>>>>> {
    #[inline(always)]
    fn call(self) -> u32 {
        regular(
            unsafe { self.inner.inner.inner.inner.field.unwrap_unchecked() }.0,
            unsafe { self.inner.inner.inner.field.unwrap_unchecked() }.0,
            unsafe { self.inner.inner.field.unwrap_unchecked() }.0,
            unsafe { self.inner.field.unwrap_unchecked() }.0,
            unsafe { self.field.unwrap_unchecked() }.0,
        )
    }
}

#[repr(transparent)]
struct Arg1<'a>(&'a str);
#[repr(transparent)]
struct Arg2(u32);
#[repr(transparent)]
struct Arg3(bool);
#[repr(transparent)]
struct Arg4(Option<String>);
#[repr(transparent)]
struct Arg5(Option<u32>);
