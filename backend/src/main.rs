use std::error::Error;
use std::net::SocketAddr;
use hyper::{Body, Method, Response};
use hyper::server::conn::Http;
use hyper::service::service_fn;
use tokio::net::TcpListener;



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

