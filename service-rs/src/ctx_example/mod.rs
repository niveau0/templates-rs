mod service;

use crate::cfg::Settings;
use crate::Shared;

pub async fn run_service(settings: Shared<Settings>) -> Result<(), String> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = axum::Router::new()
        .route("/resources", axum::routing::get(service::resources))
        .route("/resources", axum::routing::post(service::create_resource));

    let settings = settings.read().await;
    let listener = tokio::net::TcpListener::bind((settings.http_ip, settings.http_port))
        .await?;
    tracing::debug!("listening on {}", listener.local_addr()?);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
    axum::serve(listener, app).await
}
