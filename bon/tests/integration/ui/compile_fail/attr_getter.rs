use bon::Builder;

#[derive(Builder)]
struct StartFnCompat {
    #[builder(getter, start_fn)]
    x: u32,
}

#[derive(Builder)]
struct FinishFnCompat {
    #[builder(getter, finish_fn)]
    x: u32,
}

#[derive(Builder)]
struct SkipCompat {
    #[builder(getter, skip)]
    x: u32,
}

#[derive(Builder)]
struct OverwritableCompat {
    #[builder(getter, overwritable)]
    x: u32,
}

#[derive(Builder)]
struct NegativeTest {
    #[builder(getter)]
    x1: u32,

    #[builder(getter, default)]
    x2: u32,

    #[builder(getter)]
    x3: Option<u32>
}

fn main() {
    let builder = NegativeTest::builder();

    builder.get_x1();
    builder.get_x2();
    builder.get_x3();
}
