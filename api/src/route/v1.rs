use axum::Router;
use registry::AppRegistry;

use crate::route::{
    book::build_book_routes, health::build_health_check_routers, user::build_user_router,
};

pub fn routes() -> Router<AppRegistry> {
    let routers = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routes())
        .merge(build_user_router());

    Router::new().nest("/api/v1", routers)
}
