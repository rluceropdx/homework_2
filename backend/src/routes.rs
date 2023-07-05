
use axum::response::Response;
use axum::Router;
use axum::routing::get;
use http::StatusCode;
use hyper::Body;
use crate::db::AppDatabase;
use crate::handlers;

pub fn get_router() -> Router {
    let db = AppDatabase::default();
    Router::new()
        .route("/questions", get(handlers::get_questions))
        .route("/*_", get(handle_404))
        .with_state(db)
}

async fn handle_404() ->  Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}
