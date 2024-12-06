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

    #[builder(getter)]
    x2: Option<u32>,

    #[builder(getter, default)]
    x3: u32,
}

#[derive(Builder)]
struct NonCopy {
    #[builder(getter(copy))]
    x1: String,

    #[builder(getter(copy))]
    x2: Option<String>,

    #[builder(getter(copy), default)]
    x3: String,
}

fn main() {
    let builder = NegativeTest::builder();

    builder.get_x1();
    builder.get_x2();
    builder.get_x3();
}
