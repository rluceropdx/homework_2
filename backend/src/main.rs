use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use dotenvy::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::error::AppError;

mod question;
mod db;
mod routes;
mod error;
mod layers;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    init_logging();
    
    let addr = get_host_from_env();

    let (cors_layer, trace_layer) = layers::get_layers();

    //let app = Router::new()
    //    .route("/questions", get(hello_world));
    let app = routes::get_router().await?.layer(cors_layer).layer(trace_layer);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[allow(dead_code)]
async fn hello_world() -> String {
    "Hello World!".to_string()
}

fn get_host_from_env() -> SocketAddr {
    let host = std::env::var("API_HOST").unwrap();
    let api_host = IpAddr::from_str(&host).unwrap();
    let api_port: u16 = std::env::var("API_PORT")
        .expect("Could not find API_PORT in .env file")
        .parse()
        .expect("Can't create a u16 from the given API_PORT string");

    SocketAddr::from((api_host, api_port))
}

fn init_logging() {
    // https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "backend=trace,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
