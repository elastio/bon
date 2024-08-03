#![allow(unsafe_code, dead_code, unreachable_pub, dropping_copy_types)]

// use bon::builder;
use std::hint::black_box;
use std::marker::PhantomData;

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
fn builder<'a>() -> Builder<'a> {
    Builder {
        arg1: None,
        arg2: None,
        arg3: None,
        arg4: None,
        arg5: None,
    }
}

struct Builder<'a> {
    arg1: Option<&'a str>,
    arg2: Option<u32>,
    arg3: Option<bool>,
    arg4: Option<Option<String>>,
    arg5: Option<Option<u32>>,
}

impl<'a> Builder<'a> {
    #[inline(always)]
    fn arg1(mut self, arg1: &'a str) -> Self {
        self.arg1 = Some(arg1);
        self
    }

    #[inline(always)]
    fn arg2(mut self, arg2: u32) -> Self {
        self.arg2 = Some(arg2);
        self
    }

    #[inline(always)]
    fn arg3(mut self, arg3: bool) -> Self {
        self.arg3 = Some(arg3);
        self
    }

    #[inline(always)]
    fn maybe_arg4(mut self, arg4: Option<String>) -> Self {
        self.arg4 = Some(arg4);
        self
    }

    #[inline(always)]
    fn maybe_arg5(mut self, arg5: Option<u32>) -> Self {
        self.arg5 = Some(arg5);
        self
    }

    #[inline(always)]
    fn call(self) -> u32 {
        regular(
            unsafe { self.arg1.unwrap_unchecked() },
            unsafe { self.arg2.unwrap_unchecked() },
            unsafe { self.arg3.unwrap_unchecked() },
            unsafe { self.arg4.unwrap_unchecked() },
            unsafe { self.arg5.unwrap_unchecked() },
        )
    }
}
