/// Unexported builder
#[derive(bon::Builder)]
pub struct UnexportedBuilder {
    _field1: u32,
    _field2: u32,
}

/// Unexported state mod
#[derive(bon::Builder)]
pub struct UnexportedStateMod {
    _field1: u32,
    _field2: u32,
}
