use axum::{extract::DefaultBodyLimit, routing::{get, post}, Router};
use super::web::routes::{get_by_id, post_files};

pub trait FilesRouter {
    fn register_files_routes(self) -> Self;

    fn register_files_layers(self) -> Self;
}

impl FilesRouter for Router {
    fn register_files_routes(self) -> Self {
        self.route("/files", post(post_files))
            .route("/files/:id", get(get_by_id))
    }
    
    fn register_files_layers(self) -> Self {
        self.layer(DefaultBodyLimit::max(16777216))
    }
}