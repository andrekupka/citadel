use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use async_trait::async_trait;

use crate::periphery::pin::hardware::PinHardwareService;
use crate::periphery::pin::model::{PinEntity, PinEntityKind, PinEntityMetadata, PinState};

#[async_trait]
pub trait PinService<K, S>: Send + Sync
    where K: PinEntityKind,
          S: PinState {
    async fn list_entities(&self) -> Vec<PinEntity<K, S>>;

    async fn list_entities_by_kind(&self, kind: K) -> Vec<PinEntity<K, S>>;

    async fn get_entity_by_id(&self, id: String) -> Option<PinEntity<K, S>>;

    async fn update_entity_state_by_id(&self, id: String, state: S) -> Option<PinEntity<K, S>>;
}

pub struct DefaultPinService<K, S, H>
    where K: PinEntityKind,
          S: PinState,
          H: PinHardwareService<S> + ?Sized {
    pub entities: HashMap<String, PinEntityMetadata<K>>,
    pub hardware_service: Arc<H>,
    phantom: PhantomData<S>,
}

impl<K, S, H> DefaultPinService<K, S, H>
    where K: PinEntityKind,
          S: PinState,
          H: PinHardwareService<S> + ?Sized {
    pub fn new(
        entities: HashMap<String, PinEntityMetadata<K>>,
        hardware_service: Arc<H>,
    ) -> Self {
        Self {
            entities,
            hardware_service,
            phantom: PhantomData::default(),
        }
    }

    fn enhance_entity_with_state(&self, metadata: &PinEntityMetadata<K>) -> PinEntity<K, S> {
        let state = self.hardware_service.get_state(metadata.pin);
        PinEntity {
            metadata: metadata.clone(),
            state,
        }
    }
}

#[async_trait]
impl<K, S, H> PinService<K, S> for DefaultPinService<K, S, H>
    where K: PinEntityKind,
          S: PinState,
          H: PinHardwareService<S> + ?Sized {
    async fn list_entities(&self) -> Vec<PinEntity<K, S>> {
        self.entities
            .values()
            .map(|metadata| self.enhance_entity_with_state(metadata))
            .collect()
    }

    async fn list_entities_by_kind(&self, kind: K) -> Vec<PinEntity<K, S>> {
        self.entities
            .values()
            .filter(|metadata|
                metadata.kind == kind
            )
            .map(|metadata| self.enhance_entity_with_state(metadata))
            .collect()
    }

    async fn get_entity_by_id(&self, id: String) -> Option<PinEntity<K, S>> {
        self.entities.get(&id)
            .map(|metadata| self.enhance_entity_with_state(metadata))
    }


    async fn update_entity_state_by_id(&self, id: String, state: S) -> Option<PinEntity<K, S>> {
        self.entities.get(&id).map(|metadata| {
            self.hardware_service.set_state(metadata.pin, state);
            PinEntity {
                metadata: metadata.clone(),
                state,
            }
        })
    }
}