use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::periphery::gpio::config::GpioConfig;
use crate::periphery::gpio::model::{GpioEntity, GpioEntityKind, GpioEntityMetadata, GpioEntityState};

#[async_trait]
pub trait GpioService: Send + Sync {
    async fn list_entities(&self) -> Vec<GpioEntity>;
    async fn list_entities_by_kind(&self, kind: GpioEntityKind) -> Vec<GpioEntity>;
}

pub fn create_gpio_service(config: &Vec<GpioConfig>) -> Arc<dyn GpioService> {
    let entities = config.iter().map(|c|
        (c.id.clone(), c.into())
    ).collect();
    Arc::new(DefaultGpioService {
        gpio_entities: entities,
    })
}

struct DefaultGpioService {
    gpio_entities: HashMap<String, GpioEntityMetadata>,
}

#[async_trait]
impl GpioService for DefaultGpioService {
    async fn list_entities(&self) -> Vec<GpioEntity> {
        self.gpio_entities
            .values()
            .map(|metadata|
                GpioEntity {
                    metadata: metadata.clone(),
                    state: GpioEntityState::High,
                }
            ).collect()
    }

    async fn list_entities_by_kind(&self, kind: GpioEntityKind) -> Vec<GpioEntity> {
        self.gpio_entities
            .values()
            .filter(|metadata|
                metadata.kind == kind
            )
            .map(|metadata|
                GpioEntity {
                    metadata: metadata.clone(),
                    state: GpioEntityState::High,
                }
            ).collect()
    }
}
