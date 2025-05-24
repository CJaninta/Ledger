use axum::{ http::Method, routing::get, Router };
use ledger::{ setting::Settings };
use tracing::{ info, Level };
use tower_http::{ cors::{ Any, CorsLayer }, trace::TraceLayer };
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    let settings = Settings::new()?;
    info!("Settings loaded successfully");

    let app = create_router();

    let bind_addr = format!("0.0.0.0:{}", settings.server_port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Server is running on http://{}", bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router() -> Router {
    Router::new()
        // Routes
        .route("/", get(root))
        // Middleware
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
}

async fn root() -> &'static str {
    "Hello, world!"
}
