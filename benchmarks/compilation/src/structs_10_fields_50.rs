#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct1 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct1() -> Struct1 {
    Struct1::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct1() -> Struct1 {
    Struct1Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct1() -> Struct1 {
    Struct1 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct2 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct2() -> Struct2 {
    Struct2::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct2() -> Struct2 {
    Struct2Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct2() -> Struct2 {
    Struct2 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct3 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct3() -> Struct3 {
    Struct3::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct3() -> Struct3 {
    Struct3Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct3() -> Struct3 {
    Struct3 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct4 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct4() -> Struct4 {
    Struct4::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct4() -> Struct4 {
    Struct4Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct4() -> Struct4 {
    Struct4 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct5 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct5() -> Struct5 {
    Struct5::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct5() -> Struct5 {
    Struct5Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct5() -> Struct5 {
    Struct5 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct6 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct6() -> Struct6 {
    Struct6::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct6() -> Struct6 {
    Struct6Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct6() -> Struct6 {
    Struct6 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct7 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct7() -> Struct7 {
    Struct7::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct7() -> Struct7 {
    Struct7Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct7() -> Struct7 {
    Struct7 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct8 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct8() -> Struct8 {
    Struct8::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct8() -> Struct8 {
    Struct8Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct8() -> Struct8 {
    Struct8 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct9 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct9() -> Struct9 {
    Struct9::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct9() -> Struct9 {
    Struct9Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct9() -> Struct9 {
    Struct9 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct10 {
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
    x9: i32,
    x10: i32,
    x11: i32,
    x12: i32,
    x13: i32,
    x14: i32,
    x15: i32,
    x16: i32,
    x17: i32,
    x18: i32,
    x19: i32,
    x20: i32,
    x21: i32,
    x22: i32,
    x23: i32,
    x24: i32,
    x25: i32,
    x26: i32,
    x27: i32,
    x28: i32,
    x29: i32,
    x30: i32,
    x31: i32,
    x32: i32,
    x33: i32,
    x34: i32,
    x35: i32,
    x36: i32,
    x37: i32,
    x38: i32,
    x39: i32,
    x40: i32,
    x41: i32,
    x42: i32,
    x43: i32,
    x44: i32,
    x45: i32,
    x46: i32,
    x47: i32,
    x48: i32,
    x49: i32,
    x50: i32,
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct10() -> Struct10 {
    Struct10::builder()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct10() -> Struct10 {
    Struct10Builder::default()
        .x1(1)
        .x2(2)
        .x3(3)
        .x4(4)
        .x5(5)
        .x6(6)
        .x7(7)
        .x8(8)
        .x9(9)
        .x10(10)
        .x11(11)
        .x12(12)
        .x13(13)
        .x14(14)
        .x15(15)
        .x16(16)
        .x17(17)
        .x18(18)
        .x19(19)
        .x20(20)
        .x21(21)
        .x22(22)
        .x23(23)
        .x24(24)
        .x25(25)
        .x26(26)
        .x27(27)
        .x28(28)
        .x29(29)
        .x30(30)
        .x31(31)
        .x32(32)
        .x33(33)
        .x34(34)
        .x35(35)
        .x36(36)
        .x37(37)
        .x38(38)
        .x39(39)
        .x40(40)
        .x41(41)
        .x42(42)
        .x43(43)
        .x44(44)
        .x45(45)
        .x46(46)
        .x47(47)
        .x48(48)
        .x49(49)
        .x50(50)
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct10() -> Struct10 {
    Struct10 {
        x1: 1,
        x2: 2,
        x3: 3,
        x4: 4,
        x5: 5,
        x6: 6,
        x7: 7,
        x8: 8,
        x9: 9,
        x10: 10,
        x11: 11,
        x12: 12,
        x13: 13,
        x14: 14,
        x15: 15,
        x16: 16,
        x17: 17,
        x18: 18,
        x19: 19,
        x20: 20,
        x21: 21,
        x22: 22,
        x23: 23,
        x24: 24,
        x25: 25,
        x26: 26,
        x27: 27,
        x28: 28,
        x29: 29,
        x30: 30,
        x31: 31,
        x32: 32,
        x33: 33,
        x34: 34,
        x35: 35,
        x36: 36,
        x37: 37,
        x38: 38,
        x39: 39,
        x40: 40,
        x41: 41,
        x42: 42,
        x43: 43,
        x44: 44,
        x45: 45,
        x46: 46,
        x47: 47,
        x48: 48,
        x49: 49,
        x50: 50,
    }
}
