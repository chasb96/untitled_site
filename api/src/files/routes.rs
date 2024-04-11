use axum::{routing::post, Router};
use super::web::routes::post_files;

pub trait FilesRouter {
    fn register_files_routes(self) -> Self;
}

impl FilesRouter for Router {
    fn register_files_routes(self) -> Self {
        self.route("/files", post(post_files))
    }
}