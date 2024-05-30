use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;

use crate::app::rest::RouteContributor;
use crate::periphery::gpio::service::GpioService;
use crate::periphery::gpio::transport::GpioEntityContainerDto;

pub struct GpioRouteContributor {
    service: Arc<dyn GpioService>,
}

impl GpioRouteContributor {
    pub fn new(service: Arc<dyn GpioService>) -> Box<GpioRouteContributor> {
        Box::new(GpioRouteContributor {
            service,
        })
    }
}

async fn list_entities(
    State(service): State<Arc<dyn GpioService>>,
) -> Json<GpioEntityContainerDto> {
    let entities = service.list_entities_by_kind(None).await
        .iter()
        .map(|e| e.into())
        .collect();
    Json(GpioEntityContainerDto {
        entities,
    })
}

impl RouteContributor for GpioRouteContributor {
    fn contribute_routes(&self, router: Router) -> Router {
        router
            .route("/gpios", get(list_entities).with_state(self.service.clone()))
    }
}
