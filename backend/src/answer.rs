use crate::comment::CommentResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::question::QuestionId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnswerResult {
    pub id: i32,
    pub content: String,
    pub created_on: DateTime<Utc>,
    pub comments: Vec<CommentResult>,
}

impl From<i32> for AnswerId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswer {
    pub content: String,
    pub question_id: i32,
}
