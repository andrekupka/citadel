use serde::{Deserialize, Serialize};
use crate::periphery::pin::model::PinEntity;

#[derive(Serialize)]
pub struct PinEntityDto<K, S> {
    pub id: String,
    pub name: String,
    pub kind: K,
    pub state: S,
}

#[derive(Serialize)]
pub struct PinEntityContainerDto<K, S> {
    pub entities: Vec<PinEntityDto<K, S>>,
}

impl<K1, K2, S1, S2> From<&PinEntity<K1, S1>> for PinEntityDto<K2, S2>
    where K1: Copy + Clone + Into<K2>,
          S1: Copy + Clone + Into<S2> {
    fn from(value: &PinEntity<K1, S1>) -> Self {
        Self {
            id: value.metadata.id.clone(),
            name: value.metadata.name.clone(),
            kind: value.metadata.kind.into(),
            state: value.state.into(),
        }
    }
}

#[derive(Deserialize)]
pub struct PinEntityUpdateDto<S> {
    pub state: S,
}
