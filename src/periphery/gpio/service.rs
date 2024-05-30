use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::periphery::gpio::config::GpioEntityConfig;
use crate::periphery::gpio::hardware::GpioHardwareService;
use crate::periphery::gpio::model::{GpioEntity, GpioEntityKind, GpioEntityMetadata, GpioState};

#[async_trait]
pub trait GpioService: Send + Sync {
    async fn list_entities(&self) -> Vec<GpioEntity>;

    async fn list_entities_by_kind(&self, kind: GpioEntityKind) -> Vec<GpioEntity>;

    async fn get_entity_by_id(&self, id: String) -> Option<GpioEntity>;

    async fn update_entity_state_by_id(&self, id: String, state: GpioState) -> Option<GpioEntity>;

    async fn toggle_entity_state_by_id(&self, id: String) -> Option<GpioEntity>;
}

pub fn create_gpio_service(
    hardware_service: Arc<dyn GpioHardwareService>,
    config: &Vec<GpioEntityConfig>
) -> Arc<dyn GpioService> {
    let entities = config.iter().map(|c|
        (c.id.clone(), c.into())
    ).collect();

    Arc::new(DefaultGpioService {
        hardware_service,
        entities,
    })
}

struct DefaultGpioService {
    hardware_service: Arc<dyn GpioHardwareService>,
    entities: HashMap<String, GpioEntityMetadata>,
}

impl DefaultGpioService {
    fn enhance_entity_with_state(&self, metadata: &GpioEntityMetadata) -> GpioEntity {
        let state = self.hardware_service.get_state(metadata.pin);
        GpioEntity {
            metadata: metadata.clone(),
            state,
        }
    }
}

#[async_trait]
impl GpioService for DefaultGpioService {

    async fn list_entities(&self) -> Vec<GpioEntity> {
        self.entities
            .values()
            .map(|metadata| self.enhance_entity_with_state(metadata))
            .collect()
    }

    async fn list_entities_by_kind(&self, kind: GpioEntityKind) -> Vec<GpioEntity> {
        self.entities
            .values()
            .filter(|metadata|
                metadata.kind == kind
            )
            .map(|metadata| self.enhance_entity_with_state(metadata))
            .collect()

    }

    async fn get_entity_by_id(&self, id: String) -> Option<GpioEntity> {
        self.entities.get(&id)
            .map(|metadata| self.enhance_entity_with_state(metadata))
    }

    async fn update_entity_state_by_id(&self, id: String, state: GpioState) -> Option<GpioEntity> {
        self.entities.get(&id).map(|metadata| {
            self.hardware_service.set_state(metadata.pin, state);
            GpioEntity {
                metadata: metadata.clone(),
                state,
            }
        })
    }

    async fn toggle_entity_state_by_id(&self, id: String) -> Option<GpioEntity> {
        self.entities.get(&id).map(|metadata| {
            let state = self.hardware_service.get_state(metadata.pin);
            let inverted_state = state.invert();
            self.hardware_service.set_state(metadata.pin, inverted_state);
            GpioEntity {
                metadata: metadata.clone(),
                state: inverted_state,
            }
        })

    }
}
