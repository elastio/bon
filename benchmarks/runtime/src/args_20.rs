use bon::builder;
use std::hint::black_box;

pub fn regular_bench() -> u32 {
    let (
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7,
        arg8,
        arg9,
        arg10,
        arg11,
        arg12,
        arg13,
        arg14,
        arg15,
        arg16,
        arg17,
        arg18,
        arg19,
        arg20,
    ) = black_box((
        "4",
        24,
        true,
        Some("5"),
        Some(6),
        &[1, 2, 43, 65],
        (10, 11),
        [12, 13, 14],
        "15",
        "16",
        "5",
        25,
        true,
        Some("6"),
        Some(7),
        &[2, 3, 44, 66],
        (11, 12),
        [13, 14, 15],
        "16",
        "17",
    ));

    regular(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
        arg15, arg16, arg17, arg18, arg19, arg20,
    )
}

pub fn builder_bench() -> u32 {
    let (
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7,
        arg8,
        arg9,
        arg10,
        arg11,
        arg12,
        arg13,
        arg14,
        arg15,
        arg16,
        arg17,
        arg18,
        arg19,
        arg20,
    ) = black_box((
        "4",
        24,
        true,
        Some("5"),
        Some(6),
        &[1, 2, 43, 65],
        (10, 11),
        [12, 13, 14],
        "15",
        "16",
        "5",
        25,
        true,
        Some("6"),
        Some(7),
        &[2, 3, 44, 66],
        (11, 12),
        [13, 14, 15],
        "16",
        "17",
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
        .arg11(arg11)
        .arg12(arg12)
        .arg13(arg13)
        .maybe_arg14(arg14)
        .maybe_arg15(arg15)
        .arg16(arg16)
        .arg17(arg17)
        .arg18(arg18)
        .arg19(arg19)
        .arg20(arg20)
        .call()
}

#[builder(crate = crate::bon, start_fn = builder)]
fn regular(
    arg1: &str,
    arg2: u32,
    arg3: bool,
    arg4: Option<&str>,
    arg5: Option<u32>,
    arg6: &[u32],
    arg7: (u32, u32),
    arg8: [u32; 3],
    arg9: &str,
    arg10: &str,

    arg11: &str,
    arg12: u32,
    arg13: bool,
    arg14: Option<&str>,
    arg15: Option<u32>,
    arg16: &[u32],
    arg17: (u32, u32),
    arg18: [u32; 3],
    arg19: &str,
    arg20: &str,
) -> u32 {
    let x = arg1.parse::<u32>().unwrap() + arg2;
    let x = x + u32::from(arg3);
    let x = x + arg4.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg5.unwrap_or(0);
    let x = x + arg6.iter().sum::<u32>();
    let x = x + arg7.0 + arg7.1 + arg8.iter().sum::<u32>();
    let x = x + arg9.parse::<u32>().unwrap();
    let x = x + arg10.parse::<u32>().unwrap();

    let x = x + arg11.parse::<u32>().unwrap() + arg12;
    let x = x + u32::from(arg13);
    let x = x + arg14.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg15.unwrap_or(0);
    let x = x + arg16.iter().sum::<u32>();
    let x = x + arg17.0 + arg17.1 + arg18.iter().sum::<u32>();
    let x = x + arg19.parse::<u32>().unwrap();
    let x = x + arg20.parse::<u32>().unwrap();

    x
}
