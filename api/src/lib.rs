mod routes;
mod health;
mod axum;
mod auth;
mod util;
mod jwt;
mod projects;
mod users;

pub use routes::create_api_router;