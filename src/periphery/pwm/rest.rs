use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, put};
use serde::Deserialize;

use crate::app::rest::RouteContributor;
use crate::app::transport::ErrorDto;
use crate::periphery::pwm::model::PwmEntity;
use crate::periphery::pwm::transport::{PwmEntityContainerDto, PwmEntityDto, PwmEntityKindDto, PwmEntityUpdateDto};
use crate::periphery::pwm::service::PwmService;

pub struct PwmRouteContributor {
    service: Arc<dyn PwmService>,
}

impl PwmRouteContributor {
    pub fn new(service: Arc<dyn PwmService>) -> Box<PwmRouteContributor> {
        Box::new(PwmRouteContributor {
            service,
        })
    }
}

#[derive(Deserialize)]
struct KindFilter {
    kind: Option<PwmEntityKindDto>,
}

async fn list_entities(
    State(service): State<Arc<dyn PwmService>>,
    Query(filter): Query<KindFilter>,
) -> Json<PwmEntityContainerDto> {
    let entities = match filter.kind {
        None => service.list_entities(),
        Some(kind) => service.list_entities_by_kind(kind.into()),
    }.await
        .iter()
        .map(|e| e.into())
        .collect();

    Json(PwmEntityContainerDto {
        entities,
    })
}

fn respond_with(optional_entity: Option<PwmEntity>) -> Response {
    match optional_entity {
        Some(entity) => (
            StatusCode::OK,
            Json(PwmEntityDto::from(&entity)),
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorDto::not_found("gpio entity not found"))
        ).into_response(),
    }
}

async fn get_entity_by_id(
    State(service): State<Arc<dyn PwmService>>,
    Path(id): Path<String>,
) -> Response {
    let optional_entity = service.get_entity_by_id(id).await;
    respond_with(optional_entity)
}

async fn update_entity_state_by_id(
    State(service): State<Arc<dyn PwmService>>,
    Path(id): Path<String>,
    Json(update): Json<PwmEntityUpdateDto>,
) -> Response {
    let optional_entity = service.update_entity_state_by_id(id, update.state.into()).await;
    respond_with(optional_entity)
}

impl RouteContributor for PwmRouteContributor {
    fn contribute_routes(&self, router: Router) -> Router {
        router
            .route("/pwms", get(list_entities).with_state(self.service.clone()))
            .route("/pwms/:id", get(get_entity_by_id).with_state(self.service.clone()))
            .route("/pwms/:id", put(update_entity_state_by_id).with_state(self.service.clone()))
    }
}
