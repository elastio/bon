#[derive(bon::Builder)]
#[builder(state_mod(vis = "pub"))]
#[allow(dead_code)]
pub struct PubStateMod {
    required_arg: u32,
    optional_arg: Option<u32>,

    #[builder(default)]
    default_arg: u32,

    #[builder(overwritable)]
    overwritable_required_arg: u32,

    #[builder(overwritable)]
    overwritable_optional_arg: Option<u32>,

    #[builder(overwritable, default = 2 * 2 + 3)]
    overwritable_default_arg: u32,

    #[builder(transparent)]
    transparent_arg: Option<u64>,

    #[builder(with = |x: &str| -> Result<_, std::num::ParseIntError> { x.parse() })]
    with_arg: u32,
}
