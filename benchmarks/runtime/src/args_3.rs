use bon::builder;
use std::hint::black_box;

pub fn regular_bench() -> u32 {
    let (arg1, arg2, arg3) = black_box(("4", 24, Some("5")));

    regular(arg1, arg2, arg3)
}

pub fn builder_bench() -> u32 {
    let (arg1, arg2, arg3) = black_box(("4", 24, Some("5")));

    builder().arg1(arg1).arg2(arg2).maybe_arg3(arg3).call()
}

#[builder(start_fn = builder)]
fn regular(arg1: &str, arg2: u32, arg3: Option<&str>) -> u32 {
    let x = arg1.parse::<u32>().unwrap() + arg2;
    let x = x + arg3.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    x
}
