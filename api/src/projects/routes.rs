use axum::{routing::{delete, get, post, put}, Router};

use super::web::projects::routes::{create_project, get_project_by_id, list_projects};
use super::web::project_files::routes::{disassociate_files, sync_files, associate_files, list_project_files};

pub trait ProjectsRouter {
    fn register_projects_routes(self) -> Self;
}

impl ProjectsRouter for Router {
    fn register_projects_routes(self) -> Self {
        self.route("/projects", get(list_projects))
            .route("/projects", post(create_project))
            .route("/projects/:id", get(get_project_by_id))
            .route("/projects/:id/files", get(list_project_files))
            .route("/projects/:id/files", post(associate_files))
            .route("/projects/:id/files", delete(disassociate_files))
            .route("/projects/:id/files", put(sync_files))
    }
}