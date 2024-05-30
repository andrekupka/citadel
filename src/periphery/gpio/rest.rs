use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Query, State};
use axum::routing::get;
use serde::Deserialize;

use crate::app::rest::RouteContributor;
use crate::periphery::gpio::service::GpioService;
use crate::periphery::gpio::transport::{GpioEntityContainerDto, GpioEntityKindDto};

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

#[derive(Deserialize)]
struct KindFilter {
    kind: Option<GpioEntityKindDto>,
}

async fn list_entities(
    State(service): State<Arc<dyn GpioService>>,
    Query(filter): Query<KindFilter>,
) -> Json<GpioEntityContainerDto> {
    let entities = match filter.kind {
        None => service.list_entities(),
        Some(kind) => service.list_entities_by_kind(kind.into()),
    }.await
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
