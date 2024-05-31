use serde::Deserialize;

use crate::periphery::pin::model::{PinEntityKind, PinEntityMetadata};

pub trait PinEntityKindConfig: Copy + Clone {}

#[derive(Debug, Deserialize)]
pub struct PinEntityMetadataConfig<K>
    where K: PinEntityKindConfig {
    pub id: String,
    pub name: String,
    pub kind: K,
    pub pin: u8,
}


#[derive(Debug, Deserialize)]
pub struct PinEntityConfig<K>
    where K: PinEntityKindConfig {
    pub id: String,
    pub name: String,
    pub kind: K,
    pub pin: u8,
}

impl<K1, K2> From<&PinEntityMetadataConfig<K1>> for PinEntityMetadata<K2>
    where K1: PinEntityKindConfig + Into<K2>, K2: PinEntityKind {
    fn from(value: &PinEntityMetadataConfig<K1>) -> PinEntityMetadata<K2> {
        PinEntityMetadata {
            id: value.id.clone(),
            name: value.name.clone(),
            kind: value.kind.into(),
            pin: value.pin,
        }
    }
}
