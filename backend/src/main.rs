use std::error::Error;
use std::net::SocketAddr;
use hyper::{Body, Method, Response};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    // localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening...");

    let (stream, _) = listener.accept().await.unwrap();

    tokio::task::spawn(async move {
        let http = Http::new();
        let conn = http.serve_connection(stream, service_fn(questions_handler));
        let conn = conn.await.unwrap();
    });
}

async fn questions_handler(req: hyper::Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/questions") => {
            println!("In GET Questions");
            Ok(Response::new("Body Text".into()))
        },
        _ => {
            todo!() // 404
        }
    }
}

pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tag: Option<String>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tag: Option<String>) -> Self {
        Question {
            id,
            title,
            content,
            tag,
        }
    }
}


pub struct QuestionId(pub String);

