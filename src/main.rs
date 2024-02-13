use sanctum::{app_state::AppState, router::create_router, utilities::token_wrapper::TokenWrapper};
use sqlx::PgPool;

// async fn hello_world() -> &'static str {
//     "Hello, world!"
// }

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let app_state = AppState {
        pool,
        jwt_secret: TokenWrapper("supersecretkey".to_string()),
    };

    // let router = Router::new().route("/", get(hello_world));

    let router = create_router(app_state);

    Ok(router.into())
}
