use axum::Router;
use axum::routing::get;
use crate::app::rest::AppRouteContributor;

pub struct InfoAppRouterContributor {
}

impl InfoAppRouterContributor {

    pub(crate) fn new() -> Box<InfoAppRouterContributor> {
        Box::new(InfoAppRouterContributor {})
    }
}

async fn get_info() -> &'static str {
    "This is citadel"
}

impl AppRouteContributor for InfoAppRouterContributor {
    fn contribute_routes(&self, router: Router) -> Router {
        router.route("/info", get(get_info))
    }
}