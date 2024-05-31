use std::sync::Arc;

use async_trait::async_trait;

use crate::periphery::gpio::config::GpioEntityConfig;
use crate::periphery::gpio::hardware::GpioHardwareService;
use crate::periphery::gpio::model::{GpioEntity, GpioEntityKind, GpioState};
use crate::periphery::pin::model::PinEntity;
use crate::periphery::pin::service::{DefaultPinService, PinService};

#[async_trait]
pub trait GpioService : PinService<GpioEntityKind, GpioState> {

    async fn toggle_entity_state_by_id(&self, id: String) -> Option<GpioEntity>;
}

pub fn create_gpio_service(
    hardware_service: Arc<dyn GpioHardwareService>,
    config: &Vec<GpioEntityConfig>
) -> Arc<dyn GpioService> {
    let entities = config.iter().map(|c|
        (c.metadata.id.clone(), (&c.metadata).into())
    ).collect();

    Arc::new(DefaultGpioService {
        pin_service: Arc::new(
            DefaultPinService::new(entities, hardware_service),
        )
    })
}

struct DefaultGpioService {
    pin_service: Arc<DefaultPinService<GpioEntityKind, GpioState, dyn GpioHardwareService>>,
}

#[async_trait]
impl PinService<GpioEntityKind, GpioState> for DefaultGpioService {
    async fn list_entities(&self) -> Vec<PinEntity<GpioEntityKind, GpioState>> {
        self.pin_service.list_entities().await
    }

    async fn list_entities_by_kind(&self, kind: GpioEntityKind) -> Vec<PinEntity<GpioEntityKind, GpioState>> {
        self.pin_service.list_entities_by_kind(kind).await
    }

    async fn get_entity_by_id(&self, id: String) -> Option<PinEntity<GpioEntityKind, GpioState>> {
        self.pin_service.get_entity_by_id(id).await
    }

    async fn update_entity_state_by_id(&self, id: String, state: GpioState) -> Option<PinEntity<GpioEntityKind, GpioState>> {
        self.pin_service.update_entity_state_by_id(id, state).await
    }
}

#[async_trait]
impl GpioService for DefaultGpioService {

    async fn toggle_entity_state_by_id(&self, id: String) -> Option<GpioEntity> {
        self.pin_service.entities.get(&id).map(|metadata| {
            let state = self.pin_service.hardware_service.get_state(metadata.pin);
            let inverted_state = state.invert();
            self.pin_service.hardware_service.set_state(metadata.pin, inverted_state);
            GpioEntity {
                metadata: metadata.clone(),
                state: inverted_state,
            }
        })
    }
}
