use std::sync::Arc;

use async_trait::async_trait;

use crate::periphery::pin::model::PinEntity;
use crate::periphery::pin::service::{DefaultPinService, PinService};
use crate::periphery::pwm::config::PwmEntityConfig;
use crate::periphery::pwm::hardware::PwmHardwareService;
use crate::periphery::pwm::model::{PwmEntityKind, PwmState};

#[async_trait]
pub trait PwmService : PinService<PwmEntityKind, PwmState> {
}

pub fn create_pwm_service(
    hardware_service: Arc<dyn PwmHardwareService>,
    config: &Vec<PwmEntityConfig>
) -> Arc<dyn PwmService> {
    let entities = config.iter().map(|c|
        (c.metadata.id.clone(), (&c.metadata).into())
    ).collect();

    Arc::new(DefaultPwmService {
        pin_service: Arc::new(
            DefaultPinService::new(entities, hardware_service),
        )
    })
}

struct DefaultPwmService {
    pin_service: Arc<DefaultPinService<PwmEntityKind, PwmState, dyn PwmHardwareService>>,
}

impl PwmService for DefaultPwmService {
}

#[async_trait]
impl PinService<PwmEntityKind, PwmState> for DefaultPwmService {
    async fn list_entities(&self) -> Vec<PinEntity<PwmEntityKind, PwmState>> {
        self.pin_service.list_entities().await
    }

    async fn list_entities_by_kind(&self, kind: PwmEntityKind) -> Vec<PinEntity<PwmEntityKind, PwmState>> {
        self.pin_service.list_entities_by_kind(kind).await
    }

    async fn get_entity_by_id(&self, id: String) -> Option<PinEntity<PwmEntityKind, PwmState>> {
        self.pin_service.get_entity_by_id(id).await
    }

    async fn update_entity_state_by_id(&self, id: String, state: PwmState) -> Option<PinEntity<PwmEntityKind, PwmState>> {
        self.pin_service.update_entity_state_by_id(id, state).await
    }
}
