use serde::Deserialize;

use crate::periphery::pin::model::PinEntityMetadata;

#[derive(Debug, Deserialize)]
pub struct PinConfig<K>
    where K: Copy + Clone {
    pub entities: Vec<PinEntityConfig<K>>,
}

#[derive(Debug, Deserialize)]
pub struct PinEntityConfig<K>
    where K: Copy + Clone {
    pub id: String,
    pub name: String,
    pub kind: K,
    pub pin: u8,
}

impl<K1, K2> From<&PinEntityConfig<K1>> for PinEntityMetadata<K2>
    where K1: Copy + Clone + Into<K2>, K2: Copy + Clone {
    fn from(value: &PinEntityConfig<K1>) -> PinEntityMetadata<K2> {
        PinEntityMetadata {
            id: value.id.clone(),
            name: value.name.clone(),
            kind: value.kind.into(),
            pin: value.pin,
        }
    }
}
