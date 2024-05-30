use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

use crate::periphery::gpio::config::GpioConfig;
use crate::periphery::gpio::model::{GpioEntity, GpioEntityKind, GpioEntityMetadata, GpioEntityState};

#[async_trait]
pub trait GpioService : Send + Sync {
    async fn list_entities_by_kind(&self, kind: Option<GpioEntityKind>) -> Vec<GpioEntity>;
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
    async fn list_entities_by_kind(&self, _optional_kind: Option<GpioEntityKind>) -> Vec<GpioEntity> {
        /*let filter_function: fn((&String, &GpioEntityMetadata)) -> bool = match optional_kind {
            Some(kind) =>
                |(_id, metadata)| { metadata.kind == kind },
            None =>
                |(_id, metadata)| { true },
        };*/

        self.gpio_entities.iter()
            //.filter(filter_function)
            .map(|(_id, metadata)|
                GpioEntity {
                    metadata: metadata.clone(),
                    state: GpioEntityState::High,
                }
            ).collect()
    }
}
