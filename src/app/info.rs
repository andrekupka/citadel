use axum::Router;
use axum::routing::get;
use crate::app::rest::RouteContributor;

pub struct InfoRouteContributor {
}

impl InfoRouteContributor {

    pub(crate) fn new() -> Box<InfoRouteContributor> {
        Box::new(InfoRouteContributor {})
    }
}

async fn get_info() -> &'static str {
    "This is citadel"
}

impl RouteContributor for InfoRouteContributor {
    fn contribute_routes(&self, router: Router) -> Router {
        router.route("/info", get(get_info))
    }
}