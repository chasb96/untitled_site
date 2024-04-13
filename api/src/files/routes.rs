use axum::{routing::{get, post}, Router};
use super::web::routes::{get_by_id, post_files};

pub trait FilesRouter {
    fn register_files_routes(self) -> Self;
}

impl FilesRouter for Router {
    fn register_files_routes(self) -> Self {
        self.route("/files", post(post_files))
            .route("/files/:id", get(get_by_id))
    }
}