use serde::Serialize;

#[derive(Serialize)]
pub struct CreateProjectResponse {
    pub id: String,
}

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct ListProjectsResponse {
    pub projects: Vec<ProjectResponse>,
}
