use bon::builder;
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

#[builder(expose_positional_fn = regular)]
fn builder(
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
