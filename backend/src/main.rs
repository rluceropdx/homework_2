use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use axum::Router;
use axum::routing::{get, MethodRouter};
use hyper::{Body, Method, Response};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_logging();
    
    let addr = get_host_from_env();

    let app = Router::new()
        .route("/questions", get(hello_world));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


}

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
