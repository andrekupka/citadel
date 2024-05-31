use async_trait::async_trait;

use crate::periphery::pin::model::{PinEntity, PinEntityKind, PinState};

#[async_trait]
pub trait PinService<K, S>: Send + Sync
    where K: PinEntityKind,
          S: PinState {
    async fn list_entities(&self) -> Vec<PinEntity<K, S>>;

    async fn list_entities_by_kind(&self, kind: K) -> Vec<PinEntity<K, S>>;

    async fn get_entity_by_id(&self, id: String) -> Option<PinEntity<K, S>>;

    async fn update_entity_state_by_id(&self, id: String, state: S) -> Option<PinEntity<K, S>>;
}