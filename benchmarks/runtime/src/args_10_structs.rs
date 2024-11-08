use bon::builder;
use std::hint::black_box;

struct Point2D {
    x: u32,
    y: u32,
}

struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

pub fn regular_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) = black_box((
        Point3D { x: 4, y: 5, z: 6 },
        24,
        true,
        Some(Point2D { x: 55, y: 63 }),
        Some(6),
        &[
            Point3D { x: 1, y: 2, z: 43 },
            Point3D {
                x: 65,
                y: 43,
                z: 52,
            },
        ],
        (10, 11),
        &[
            Point2D { x: 12, y: 13 },
            Point2D { x: 4, y: 0 },
            Point2D { x: 1, y: 1 },
        ],
        Point2D { x: 15, y: 16 },
        Point3D {
            x: 17,
            y: 18,
            z: 19,
        },
    ));

    regular(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)
}

pub fn builder_bench() -> u32 {
    let (arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) = black_box((
        Point3D { x: 4, y: 5, z: 6 },
        24,
        true,
        Some(Point2D { x: 55, y: 63 }),
        Some(6),
        &[
            Point3D { x: 1, y: 2, z: 43 },
            Point3D {
                x: 65,
                y: 43,
                z: 52,
            },
        ],
        (10, 11),
        &[
            Point2D { x: 12, y: 13 },
            Point2D { x: 4, y: 0 },
            Point2D { x: 1, y: 1 },
        ],
        Point2D { x: 15, y: 16 },
        Point3D {
            x: 17,
            y: 18,
            z: 19,
        },
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
    arg1: Point3D,
    arg2: u32,
    arg3: bool,
    arg4: Option<Point2D>,
    arg5: Option<u32>,
    arg6: &[Point3D],
    arg7: (u32, u32),
    arg8: &[Point2D],
    arg9: Point2D,
    arg10: Point3D,
) -> u32 {
    let x = arg1.x + arg1.y + arg1.z;
    let x = x + arg2;
    let x = x + u32::from(arg3);
    let x = x + arg4.map(|point| point.x * point.y).unwrap_or(0);
    let x = x + arg5.unwrap_or(0);
    let x = x + arg6
        .iter()
        .map(|point| point.x + point.y * point.z)
        .sum::<u32>();
    let x = x + arg7.0 + arg7.1 + arg8.iter().map(|point| point.x + point.y).sum::<u32>();
    let x = x + arg9.x * arg9.y;
    let x = x + arg10.x * arg9.y * arg10.z;
    x
}
