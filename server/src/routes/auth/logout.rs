use axum::extract::State;
use axum::http::StatusCode;
use time::OffsetDateTime;
use tower_cookies::{Cookie, Cookies};

use database::models::token_revocations::TokenRevocation;

use crate::{auth::extractor::AuthUser, error::ServerError, AppState};

pub async fn logout(
	State(state): State<AppState>,
	cookies: Cookies,
	AuthUser(claims): AuthUser,
) -> Result<StatusCode, ServerError> {
	let expires_at =
		OffsetDateTime::from_unix_timestamp(claims.exp).map_err(|_| ServerError::Internal)?;

	TokenRevocation::revoke(state.pool, claims.jti, expires_at)
		.await
		.map_err(|_| ServerError::Internal)?;

	cookies.remove(Cookie::from("session"));

	Ok(StatusCode::NO_CONTENT)
}
