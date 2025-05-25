use std::sync::Arc;
use axum::{ http::Method, routing::post, Router };
use ledger::{
    handlers::user::{ create_user },
    setting::Settings,
    usecases::user::{ UserUseCase, UserUseCaseImpl },
    repositories::user::{ UserRepo },
};
use sqlx::mysql::MySqlPoolOptions;
use tracing::{ info, Level };
use tower_http::{ cors::{ Any, CorsLayer }, trace::TraceLayer };
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    let settings = Settings::new()?;
    info!("Settings loaded successfully");

    let database_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&settings.get_database_url().as_str()).await?;
    info!("Database connection pool created");

    let user_repo = UserRepo::new(database_pool);
    let user_usecase = UserUseCaseImpl::new(user_repo);
    let app = create_router(&user_usecase);

    let bind_addr = format!("0.0.0.0:{}", settings.server_port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Server is running on http://{}", bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(user_usecase: &Arc<UserUseCaseImpl>) -> Router {
    Router::new()
        // Routes
        .route(
            "/api/v1/user/create",
            post({
                let usecase = Arc::clone(&user_usecase);
                move |body| create_user(body, usecase)
            })
        )
        // Middleware
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
}
