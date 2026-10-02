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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct11 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct11() -> Struct11 {
    Struct11::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct11() -> Struct11 {
    Struct11Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct11() -> Struct11 {
    Struct11 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct12 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct12() -> Struct12 {
    Struct12::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct12() -> Struct12 {
    Struct12Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct12() -> Struct12 {
    Struct12 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct13 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct13() -> Struct13 {
    Struct13::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct13() -> Struct13 {
    Struct13Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct13() -> Struct13 {
    Struct13 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct14 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct14() -> Struct14 {
    Struct14::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct14() -> Struct14 {
    Struct14Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct14() -> Struct14 {
    Struct14 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct15 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct15() -> Struct15 {
    Struct15::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct15() -> Struct15 {
    Struct15Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct15() -> Struct15 {
    Struct15 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct16 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct16() -> Struct16 {
    Struct16::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct16() -> Struct16 {
    Struct16Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct16() -> Struct16 {
    Struct16 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct17 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct17() -> Struct17 {
    Struct17::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct17() -> Struct17 {
    Struct17Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct17() -> Struct17 {
    Struct17 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct18 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct18() -> Struct18 {
    Struct18::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct18() -> Struct18 {
    Struct18Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct18() -> Struct18 {
    Struct18 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct19 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct19() -> Struct19 {
    Struct19::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct19() -> Struct19 {
    Struct19Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct19() -> Struct19 {
    Struct19 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct20 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct20() -> Struct20 {
    Struct20::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct20() -> Struct20 {
    Struct20Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct20() -> Struct20 {
    Struct20 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct21 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct21() -> Struct21 {
    Struct21::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct21() -> Struct21 {
    Struct21Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct21() -> Struct21 {
    Struct21 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct22 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct22() -> Struct22 {
    Struct22::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct22() -> Struct22 {
    Struct22Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct22() -> Struct22 {
    Struct22 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct23 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct23() -> Struct23 {
    Struct23::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct23() -> Struct23 {
    Struct23Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct23() -> Struct23 {
    Struct23 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct24 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct24() -> Struct24 {
    Struct24::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct24() -> Struct24 {
    Struct24Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct24() -> Struct24 {
    Struct24 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct25 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct25() -> Struct25 {
    Struct25::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct25() -> Struct25 {
    Struct25Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct25() -> Struct25 {
    Struct25 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct26 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct26() -> Struct26 {
    Struct26::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct26() -> Struct26 {
    Struct26Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct26() -> Struct26 {
    Struct26 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct27 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct27() -> Struct27 {
    Struct27::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct27() -> Struct27 {
    Struct27Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct27() -> Struct27 {
    Struct27 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct28 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct28() -> Struct28 {
    Struct28::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct28() -> Struct28 {
    Struct28Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct28() -> Struct28 {
    Struct28 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct29 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct29() -> Struct29 {
    Struct29::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct29() -> Struct29 {
    Struct29Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct29() -> Struct29 {
    Struct29 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct30 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct30() -> Struct30 {
    Struct30::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct30() -> Struct30 {
    Struct30Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct30() -> Struct30 {
    Struct30 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct31 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct31() -> Struct31 {
    Struct31::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct31() -> Struct31 {
    Struct31Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct31() -> Struct31 {
    Struct31 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct32 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct32() -> Struct32 {
    Struct32::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct32() -> Struct32 {
    Struct32Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct32() -> Struct32 {
    Struct32 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct33 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct33() -> Struct33 {
    Struct33::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct33() -> Struct33 {
    Struct33Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct33() -> Struct33 {
    Struct33 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct34 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct34() -> Struct34 {
    Struct34::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct34() -> Struct34 {
    Struct34Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct34() -> Struct34 {
    Struct34 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct35 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct35() -> Struct35 {
    Struct35::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct35() -> Struct35 {
    Struct35Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct35() -> Struct35 {
    Struct35 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct36 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct36() -> Struct36 {
    Struct36::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct36() -> Struct36 {
    Struct36Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct36() -> Struct36 {
    Struct36 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct37 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct37() -> Struct37 {
    Struct37::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct37() -> Struct37 {
    Struct37Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct37() -> Struct37 {
    Struct37 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct38 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct38() -> Struct38 {
    Struct38::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct38() -> Struct38 {
    Struct38Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct38() -> Struct38 {
    Struct38 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct39 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct39() -> Struct39 {
    Struct39::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct39() -> Struct39 {
    Struct39Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct39() -> Struct39 {
    Struct39 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct40 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct40() -> Struct40 {
    Struct40::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct40() -> Struct40 {
    Struct40Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct40() -> Struct40 {
    Struct40 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct41 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct41() -> Struct41 {
    Struct41::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct41() -> Struct41 {
    Struct41Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct41() -> Struct41 {
    Struct41 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct42 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct42() -> Struct42 {
    Struct42::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct42() -> Struct42 {
    Struct42Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct42() -> Struct42 {
    Struct42 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct43 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct43() -> Struct43 {
    Struct43::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct43() -> Struct43 {
    Struct43Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct43() -> Struct43 {
    Struct43 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct44 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct44() -> Struct44 {
    Struct44::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct44() -> Struct44 {
    Struct44Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct44() -> Struct44 {
    Struct44 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct45 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct45() -> Struct45 {
    Struct45::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct45() -> Struct45 {
    Struct45Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct45() -> Struct45 {
    Struct45 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct46 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct46() -> Struct46 {
    Struct46::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct46() -> Struct46 {
    Struct46Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct46() -> Struct46 {
    Struct46 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct47 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct47() -> Struct47 {
    Struct47::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct47() -> Struct47 {
    Struct47Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct47() -> Struct47 {
    Struct47 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct48 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct48() -> Struct48 {
    Struct48::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct48() -> Struct48 {
    Struct48Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct48() -> Struct48 {
    Struct48 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct49 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct49() -> Struct49 {
    Struct49::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct49() -> Struct49 {
    Struct49Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct49() -> Struct49 {
    Struct49 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct50 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct50() -> Struct50 {
    Struct50::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct50() -> Struct50 {
    Struct50Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct50() -> Struct50 {
    Struct50 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct51 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct51() -> Struct51 {
    Struct51::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct51() -> Struct51 {
    Struct51Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct51() -> Struct51 {
    Struct51 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct52 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct52() -> Struct52 {
    Struct52::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct52() -> Struct52 {
    Struct52Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct52() -> Struct52 {
    Struct52 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct53 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct53() -> Struct53 {
    Struct53::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct53() -> Struct53 {
    Struct53Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct53() -> Struct53 {
    Struct53 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct54 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct54() -> Struct54 {
    Struct54::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct54() -> Struct54 {
    Struct54Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct54() -> Struct54 {
    Struct54 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct55 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct55() -> Struct55 {
    Struct55::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct55() -> Struct55 {
    Struct55Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct55() -> Struct55 {
    Struct55 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct56 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct56() -> Struct56 {
    Struct56::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct56() -> Struct56 {
    Struct56Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct56() -> Struct56 {
    Struct56 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct57 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct57() -> Struct57 {
    Struct57::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct57() -> Struct57 {
    Struct57Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct57() -> Struct57 {
    Struct57 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct58 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct58() -> Struct58 {
    Struct58::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct58() -> Struct58 {
    Struct58Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct58() -> Struct58 {
    Struct58 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct59 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct59() -> Struct59 {
    Struct59::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct59() -> Struct59 {
    Struct59Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct59() -> Struct59 {
    Struct59 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct60 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct60() -> Struct60 {
    Struct60::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct60() -> Struct60 {
    Struct60Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct60() -> Struct60 {
    Struct60 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct61 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct61() -> Struct61 {
    Struct61::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct61() -> Struct61 {
    Struct61Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct61() -> Struct61 {
    Struct61 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct62 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct62() -> Struct62 {
    Struct62::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct62() -> Struct62 {
    Struct62Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct62() -> Struct62 {
    Struct62 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct63 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct63() -> Struct63 {
    Struct63::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct63() -> Struct63 {
    Struct63Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct63() -> Struct63 {
    Struct63 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct64 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct64() -> Struct64 {
    Struct64::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct64() -> Struct64 {
    Struct64Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct64() -> Struct64 {
    Struct64 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct65 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct65() -> Struct65 {
    Struct65::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct65() -> Struct65 {
    Struct65Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct65() -> Struct65 {
    Struct65 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct66 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct66() -> Struct66 {
    Struct66::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct66() -> Struct66 {
    Struct66Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct66() -> Struct66 {
    Struct66 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct67 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct67() -> Struct67 {
    Struct67::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct67() -> Struct67 {
    Struct67Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct67() -> Struct67 {
    Struct67 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct68 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct68() -> Struct68 {
    Struct68::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct68() -> Struct68 {
    Struct68Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct68() -> Struct68 {
    Struct68 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct69 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct69() -> Struct69 {
    Struct69::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct69() -> Struct69 {
    Struct69Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct69() -> Struct69 {
    Struct69 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct70 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct70() -> Struct70 {
    Struct70::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct70() -> Struct70 {
    Struct70Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct70() -> Struct70 {
    Struct70 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct71 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct71() -> Struct71 {
    Struct71::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct71() -> Struct71 {
    Struct71Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct71() -> Struct71 {
    Struct71 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct72 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct72() -> Struct72 {
    Struct72::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct72() -> Struct72 {
    Struct72Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct72() -> Struct72 {
    Struct72 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct73 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct73() -> Struct73 {
    Struct73::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct73() -> Struct73 {
    Struct73Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct73() -> Struct73 {
    Struct73 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct74 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct74() -> Struct74 {
    Struct74::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct74() -> Struct74 {
    Struct74Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct74() -> Struct74 {
    Struct74 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct75 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct75() -> Struct75 {
    Struct75::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct75() -> Struct75 {
    Struct75Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct75() -> Struct75 {
    Struct75 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct76 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct76() -> Struct76 {
    Struct76::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct76() -> Struct76 {
    Struct76Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct76() -> Struct76 {
    Struct76 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct77 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct77() -> Struct77 {
    Struct77::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct77() -> Struct77 {
    Struct77Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct77() -> Struct77 {
    Struct77 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct78 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct78() -> Struct78 {
    Struct78::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct78() -> Struct78 {
    Struct78Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct78() -> Struct78 {
    Struct78 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct79 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct79() -> Struct79 {
    Struct79::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct79() -> Struct79 {
    Struct79Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct79() -> Struct79 {
    Struct79 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct80 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct80() -> Struct80 {
    Struct80::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct80() -> Struct80 {
    Struct80Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct80() -> Struct80 {
    Struct80 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct81 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct81() -> Struct81 {
    Struct81::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct81() -> Struct81 {
    Struct81Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct81() -> Struct81 {
    Struct81 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct82 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct82() -> Struct82 {
    Struct82::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct82() -> Struct82 {
    Struct82Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct82() -> Struct82 {
    Struct82 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct83 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct83() -> Struct83 {
    Struct83::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct83() -> Struct83 {
    Struct83Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct83() -> Struct83 {
    Struct83 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct84 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct84() -> Struct84 {
    Struct84::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct84() -> Struct84 {
    Struct84Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct84() -> Struct84 {
    Struct84 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct85 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct85() -> Struct85 {
    Struct85::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct85() -> Struct85 {
    Struct85Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct85() -> Struct85 {
    Struct85 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct86 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct86() -> Struct86 {
    Struct86::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct86() -> Struct86 {
    Struct86Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct86() -> Struct86 {
    Struct86 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct87 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct87() -> Struct87 {
    Struct87::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct87() -> Struct87 {
    Struct87Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct87() -> Struct87 {
    Struct87 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct88 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct88() -> Struct88 {
    Struct88::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct88() -> Struct88 {
    Struct88Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct88() -> Struct88 {
    Struct88 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct89 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct89() -> Struct89 {
    Struct89::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct89() -> Struct89 {
    Struct89Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct89() -> Struct89 {
    Struct89 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct90 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct90() -> Struct90 {
    Struct90::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct90() -> Struct90 {
    Struct90Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct90() -> Struct90 {
    Struct90 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct91 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct91() -> Struct91 {
    Struct91::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct91() -> Struct91 {
    Struct91Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct91() -> Struct91 {
    Struct91 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct92 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct92() -> Struct92 {
    Struct92::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct92() -> Struct92 {
    Struct92Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct92() -> Struct92 {
    Struct92 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct93 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct93() -> Struct93 {
    Struct93::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct93() -> Struct93 {
    Struct93Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct93() -> Struct93 {
    Struct93 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct94 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct94() -> Struct94 {
    Struct94::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct94() -> Struct94 {
    Struct94Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct94() -> Struct94 {
    Struct94 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct95 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct95() -> Struct95 {
    Struct95::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct95() -> Struct95 {
    Struct95Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct95() -> Struct95 {
    Struct95 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct96 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct96() -> Struct96 {
    Struct96::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct96() -> Struct96 {
    Struct96Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct96() -> Struct96 {
    Struct96 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct97 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct97() -> Struct97 {
    Struct97::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct97() -> Struct97 {
    Struct97Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct97() -> Struct97 {
    Struct97 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct98 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct98() -> Struct98 {
    Struct98::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct98() -> Struct98 {
    Struct98Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct98() -> Struct98 {
    Struct98 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct99 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct99() -> Struct99 {
    Struct99::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct99() -> Struct99 {
    Struct99Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct99() -> Struct99 {
    Struct99 {
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
    }
}
#[cfg_attr(
    any(feature = "bon", feature = "typed-builder", feature = "derive_builder",),
    derive(crate::Builder)
)]
pub struct Struct100 {
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
}
#[cfg(any(feature = "bon", feature = "typed-builder"))]
pub fn struct100() -> Struct100 {
    Struct100::builder()
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
        .build()
}
#[cfg(all(
    feature = "derive_builder",
    not(any(feature = "bon", feature = "typed-builder")),
))]
pub fn struct100() -> Struct100 {
    Struct100Builder::default()
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
        .build()
        .unwrap()
}
#[cfg(not(any(feature = "bon", feature = "typed-builder", feature = "derive_builder",)))]
pub fn struct100() -> Struct100 {
    Struct100 {
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
    }
}
