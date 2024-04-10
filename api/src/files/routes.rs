use axum::{routing::post, Router};
use super::web::routes::create_file;

pub trait FilesRouter {
    fn register_files_routes(self) -> Self;
}

impl FilesRouter for Router {
    fn register_files_routes(self) -> Self {
        self.route("/files", post(create_file))
    }
}