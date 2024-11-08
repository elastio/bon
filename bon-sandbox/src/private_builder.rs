/// Some docs on the private builder
#[derive(bon::Builder)]
#[builder(builder_type(vis = ""))]
pub struct PrivateBuilder {
    _field: String,
}
