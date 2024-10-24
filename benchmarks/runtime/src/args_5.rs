use bon::builder;
use std::hint::black_box;

pub fn regular_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5) = black_box(("4", 24, true, Some("5"), Some(6)));

    regular(arg1, arg2, arg3, arg4, arg5)
}

pub fn builder_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5) = black_box(("4", 24, true, Some("5"), Some(6)));

    builder()
        .arg1(arg1)
        .arg2(arg2)
        .arg3(arg3)
        .maybe_arg4(arg4)
        .maybe_arg5(arg5)
        .call()
}

#[builder(start_fn = builder)]
fn regular(arg1: &str, arg2: u32, arg3: bool, arg4: Option<&str>, arg5: Option<u32>) -> u32 {
    let x = arg1.parse::<u32>().unwrap() + arg2;
    let x = x + u32::from(arg3);
    let x = x + arg4.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg5.unwrap_or(0);
    x
}
