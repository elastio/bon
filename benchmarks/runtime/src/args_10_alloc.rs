use bon::builder;
use std::hint::black_box;

pub fn regular_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) = black_box((
        "4".to_owned(),
        24,
        true,
        Some("5".to_owned()),
        Some(6),
        vec![1, 2, 43, 65],
        (10, 11),
        vec![12, 13, 14],
        "15".to_owned(),
        "16".to_owned(),
    ));

    regular(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub fn builder_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) = black_box((
        "4".to_owned(),
        24,
        true,
        Some("5".to_owned()),
        Some(6),
        vec![1, 2, 43, 65],
        (10, 11),
        vec![12, 13, 14],
        "15".to_owned(),
        "16".to_owned(),
    ));

    builder()
        .arg1(arg1)
        .arg2(arg2)
        .arg3(arg3)
        .maybe_arg4(arg4)
        .maybe_arg5(arg5)
        .arg6(arg6)
        .arg7(arg7)
        .arg8(arg8)
        .arg9(arg9)
        .arg10(arg10)
        .call()
}

#[builder(crate = crate::bon, start_fn = builder)]
fn regular(
    arg1: String,
    arg2: u32,
    arg3: bool,
    arg4: Option<String>,
    arg5: Option<u32>,
    arg6: Vec<u32>,
    arg7: (u32, u32),
    arg8: Vec<u32>,
    arg9: String,
    arg10: String,
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
