mod routes;
mod health;
mod axum;
mod auth;
mod util;
mod jwt;
mod projects;
mod users;
mod files;

pub use routes::create_api_router;