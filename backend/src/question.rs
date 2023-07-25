use crate::answer::AnswerResult;
use crate::comment::CommentResult;
use chrono::{DateTime, Utc};
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
    fmt = "id: {}, title: {}, content: {}, tags: {:?}",
    id,
    title,
    content,
    tags
)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    #[allow(dead_code)]
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    sqlx::Type,
    Display,
    derive_more::Deref,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct QuestionId(pub i32);

impl From<i32> for QuestionId {
    fn from(value: i32) -> Self {
        QuestionId(value)
    }
}

impl From<QuestionId> for i32 {
    fn from(value: QuestionId) -> Self {
        value.0
    }
}

pub trait IntoQuestionId {
    fn into_question_id(self) -> QuestionId;
}

impl IntoQuestionId for i32 {
    fn into_question_id(self) -> QuestionId {
        QuestionId::from(self)
    }
}

impl IntoQuestionId for QuestionId {
    fn into_question_id(self) -> QuestionId {
        self
    }
}

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GetQuestionById {
    pub question_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestion {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestionResult {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub created_on: DateTime<Utc>,
    pub comments: Vec<CommentResult>,
    pub answers: Vec<AnswerResult>,
}
