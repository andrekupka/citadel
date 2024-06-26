use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use serde::Deserialize;

use crate::app::rest::RouteContributor;
use crate::app::transport::ErrorDto;
use crate::periphery::gpio::model::GpioEntity;
use crate::periphery::gpio::service::GpioService;
use crate::periphery::gpio::transport::{GpioEntityContainerDto, GpioEntityDto, GpioEntityKindDto, GpioEntityUpdateDto};

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

fn respond_with(optional_entity: Option<GpioEntity>) -> Response {
    match optional_entity {
        Some(entity) => (
            StatusCode::OK,
            Json(GpioEntityDto::from(&entity)),
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorDto::not_found("gpio entity not found"))
        ).into_response(),
    }
}

async fn get_entity_by_id(
    State(service): State<Arc<dyn GpioService>>,
    Path(id): Path<String>,
) -> Response {
    let optional_entity = service.get_entity_by_id(id).await;
    respond_with(optional_entity)
}

async fn update_entity_state_by_id(
    State(service): State<Arc<dyn GpioService>>,
    Path(id): Path<String>,
    Json(update): Json<GpioEntityUpdateDto>,
) -> Response {
    let optional_entity = service.update_entity_state_by_id(id, update.state.into()).await;
    respond_with(optional_entity)
}

async fn toggle_entity_state_by_id(
    State(service): State<Arc<dyn GpioService>>,
    Path(id): Path<String>,
) -> Response {
    let optional_entity = service.toggle_entity_state_by_id(id).await;
    respond_with(optional_entity)
}

impl RouteContributor for GpioRouteContributor {
    fn contribute_routes(&self, router: Router) -> Router {
        router
            .route("/gpios", get(list_entities).with_state(self.service.clone()))
            .route("/gpios/:id", get(get_entity_by_id).with_state(self.service.clone()))
            .route("/gpios/:id", put(update_entity_state_by_id).with_state(self.service.clone()))
            .route("/gpios/:id/toggle", post(toggle_entity_state_by_id).with_state(self.service.clone()))
    }
}
