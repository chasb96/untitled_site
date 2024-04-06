use axum::{routing::get, Router};
use crate::{auth::AuthRouter, axum::layers::LogLayer, health::health, projects::ProjectsRouter, users::UsersRouter};

trait ApiRouter {
    fn register_api_routes(self) -> Self;

    fn register_api_layers(self) -> Self;
}

impl ApiRouter for Router {
    fn register_api_routes(self) -> Self {
        self.route("/health", get(health))
            
    }
    
    fn register_api_layers(self) -> Self {
        self.layer(LogLayer::new())
    }
}

pub fn create_api_router() -> Router {
    Router::new()
        .register_api_routes()
        .register_auth_routes()
        .register_projects_routes()
        .register_user_routes()
        .register_api_layers()
}