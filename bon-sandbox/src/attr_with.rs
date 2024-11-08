use bon::Builder;
use std::collections::BTreeMap;

#[derive(Builder)]
pub struct AttrWith {
    #[builder(with = |iter: impl IntoIterator<Item = u32>| Vec::from_iter(iter))]
    _vec: Vec<u32>,

    #[builder(with = |iter: impl IntoIterator<Item = (impl Into<String>, u32)>| {
        iter
            .into_iter()
            .map(|(k, v)| (k.into(), v))
            .collect()
    })]
    _map: BTreeMap<String, u32>,
}
