use axum::routing::{get, post};
use axum::Router;
use tower_cookies::CookieManagerLayer;

mod auth;

use crate::error::AppResult;
use crate::AppState;

async fn health() -> &'static str {
	"ok"
}

pub async fn routes(state: AppState) -> AppResult<Router> {
	Ok(Router::new()
		.route("/health", get(health))
		.route("/auth/login", post(auth::login))
		.route("/auth/logout", post(auth::logout))
		.layer(CookieManagerLayer::new())
		.with_state(state))
}
