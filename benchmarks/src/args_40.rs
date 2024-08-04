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
        arg21,
        arg22,
        arg23,
        arg24,
        arg25,
        arg26,
        arg27,
        arg28,
        arg29,
        arg30,
        arg31,
        arg32,
        arg33,
        arg34,
        arg35,
        arg36,
        arg37,
        arg38,
        arg39,
        arg40,
    ) = black_box((
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
        "5",
        25,
        true,
        Some("6".to_string()),
        Some(7),
        vec![2, 3, 44, 66],
        (11, 12),
        [13, 14, 15],
        "16".to_string(),
        "17".to_string().into_boxed_str(),
        "6",
        26,
        true,
        Some("7".to_string()),
        Some(8),
        vec![3, 4, 45, 67],
        (12, 13),
        [14, 15, 16],
        "17".to_string(),
        "18".to_string().into_boxed_str(),
        "7",
        27,
        true,
        Some("8".to_string()),
        Some(9),
        vec![4, 5, 46, 68],
        (13, 14),
        [15, 16, 17],
        "18".to_string(),
        "19".to_string().into_boxed_str(),
    ));

    regular(
        arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14,
        arg15, arg16, arg17, arg18, arg19, arg20, arg21, arg22, arg23, arg24, arg25, arg26, arg27,
        arg28, arg29, arg30, arg31, arg32, arg33, arg34, arg35, arg36, arg37, arg38, arg39, arg40,
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
        arg21,
        arg22,
        arg23,
        arg24,
        arg25,
        arg26,
        arg27,
        arg28,
        arg29,
        arg30,
        arg31,
        arg32,
        arg33,
        arg34,
        arg35,
        arg36,
        arg37,
        arg38,
        arg39,
        arg40,
    ) = black_box((
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
        "5",
        25,
        true,
        Some("6".to_string()),
        Some(7),
        vec![2, 3, 44, 66],
        (11, 12),
        [13, 14, 15],
        "16".to_string(),
        "17".to_string().into_boxed_str(),
        "6",
        26,
        true,
        Some("7".to_string()),
        Some(8),
        vec![3, 4, 45, 67],
        (12, 13),
        [14, 15, 16],
        "17".to_string(),
        "18".to_string().into_boxed_str(),
        "7",
        27,
        true,
        Some("8".to_string()),
        Some(9),
        vec![4, 5, 46, 68],
        (13, 14),
        [15, 16, 17],
        "18".to_string(),
        "19".to_string().into_boxed_str(),
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
        .arg21(arg21)
        .arg22(arg22)
        .arg23(arg23)
        .maybe_arg24(arg24)
        .maybe_arg25(arg25)
        .arg26(arg26)
        .arg27(arg27)
        .arg28(arg28)
        .arg29(arg29)
        .arg30(arg30)
        .arg31(arg31)
        .arg32(arg32)
        .arg33(arg33)
        .maybe_arg34(arg34)
        .maybe_arg35(arg35)
        .arg36(arg36)
        .arg37(arg37)
        .arg38(arg38)
        .arg39(arg39)
        .arg40(arg40)
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

    arg11: &str,
    arg12: u32,
    arg13: bool,
    arg14: Option<String>,
    arg15: Option<u32>,
    arg16: Vec<u32>,
    arg17: (u32, u32),
    arg18: [u32; 3],
    arg19: String,
    arg20: Box<str>,

    arg21: &str,
    arg22: u32,
    arg23: bool,
    arg24: Option<String>,
    arg25: Option<u32>,
    arg26: Vec<u32>,
    arg27: (u32, u32),
    arg28: [u32; 3],
    arg29: String,
    arg30: Box<str>,

    arg31: &str,
    arg32: u32,
    arg33: bool,
    arg34: Option<String>,
    arg35: Option<u32>,
    arg36: Vec<u32>,
    arg37: (u32, u32),
    arg38: [u32; 3],
    arg39: String,
    arg40: Box<str>,
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

    let x = x + arg21.parse::<u32>().unwrap() + arg22;
    let x = x + u32::from(arg23);
    let x = x + arg24.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg25.unwrap_or(0);
    let x = x + arg26.iter().sum::<u32>();
    let x = x + arg27.0 + arg27.1 + arg28.iter().sum::<u32>();
    let x = x + arg29.parse::<u32>().unwrap();
    let x = x + arg30.parse::<u32>().unwrap();

    let x = x + arg31.parse::<u32>().unwrap() + arg32;
    let x = x + u32::from(arg33);
    let x = x + arg34.map(|x| x.parse::<u32>().unwrap()).unwrap_or(0);
    let x = x + arg35.unwrap_or(0);
    let x = x + arg36.iter().sum::<u32>();
    let x = x + arg37.0 + arg37.1 + arg38.iter().sum::<u32>();
    let x = x + arg39.parse::<u32>().unwrap();
    let x = x + arg40.parse::<u32>().unwrap();

    x
}
