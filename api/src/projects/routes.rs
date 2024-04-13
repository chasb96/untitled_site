use axum::{routing::{get, post}, Router};

use super::web::routes::{create_project, get_project_by_id, list_project_files, list_projects};

pub trait ProjectsRouter {
    fn register_projects_routes(self) -> Self;
}

impl ProjectsRouter for Router {
    fn register_projects_routes(self) -> Self {
        self.route("/projects", get(list_projects))
            .route("/projects", post(create_project))
            .route("/projects/:id", get(get_project_by_id))
            .route("/projects/:id/files", get(list_project_files))
    }
}