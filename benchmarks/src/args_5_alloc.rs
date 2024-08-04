use bon::builder;
use std::hint::black_box;

pub fn regular_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5) = black_box((
        "4".to_owned(),
        Some("5".to_owned()),
        vec![1, 2, 43, 65],
        vec![12, 13, 14],
        "15".to_owned(),
    ));

    regular(arg1, arg2, arg3, arg4, arg5)
}

pub fn builder_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5) = black_box((
        "4".to_owned(),
        Some("5".to_owned()),
        vec![1, 2, 43, 65],
        vec![12, 13, 14],
        "15".to_owned(),
    ));

    builder()
        .arg1(arg1)
        .maybe_arg2(arg2)
        .arg3(arg3)
        .arg4(arg4)
        .arg5(arg5)
        .call()
}

#[builder(expose_positional_fn = regular)]
fn builder(
    arg1: String,
    arg2: Option<String>,
    arg3: Vec<u32>,
    arg4: Vec<u32>,
    arg5: String,
) -> u32 {
    let x = arg1.parse::<u32>().unwrap();
    let x = x + arg2.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg3.iter().sum::<u32>();
    let x = x + arg4.iter().sum::<u32>();
    let x = x + arg5.parse::<u32>().unwrap();
    x
}
