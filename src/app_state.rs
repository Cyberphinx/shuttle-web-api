use axum::extract::FromRef;
use sqlx::PgPool;

use crate::utilities::token_wrapper::TokenWrapper;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: TokenWrapper,
}
