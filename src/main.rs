use std::sync::Arc;
use axum::{ http::Method, routing::{ get, post, delete }, Router };
use ledger::{
    handlers::{
        transaction::create_transaction,
        user::{ create_user, delete_user, get_user_by_id, update_user },
    },
    repositories::{ user::UserRepo, transaction::TransactionRepo },
    setting::Settings,
    usecases::{
        user::{ UserUseCase, UserUseCaseImpl },
        transaction::{ TransactionUseCase, TransactionUseCaseImpl },
    },
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

    let user_repo = UserRepo::new(database_pool.clone());
    let user_usecase = UserUseCaseImpl::new(user_repo);
    let transaction_repo = TransactionRepo::new(database_pool.clone());
    let transaction_usecase = TransactionUseCaseImpl::new(transaction_repo);
    let app = create_router(user_usecase, transaction_usecase);

    let bind_addr = format!("0.0.0.0:{}", settings.server_port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Server is running on http://{}", bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(
    user_usecase: Arc<UserUseCaseImpl>,
    transaction_usecase: Arc<TransactionUseCaseImpl>
) -> Router {
    let user_usecase = Arc::clone(&user_usecase);
    let transaction_usecase = Arc::clone(&transaction_usecase);

    // User routes
    let user_routes = Router::new()
        .route("/create", post(create_user))
        .route("/", get(get_user_by_id))
        .route("/update", post(update_user))
        .route("/delete", delete(delete_user))
        .with_state(user_usecase);

    // Transaction routes
    let transaction_routes = Router::new()
        .route("/create", post(create_transaction))
        .with_state(transaction_usecase);

    // Final app router
    Router::new()
        .nest("/api/v1/user", user_routes)
        .nest("/api/v1/transaction", transaction_routes)
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
}
