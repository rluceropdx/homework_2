use crate::answer::*;
use crate::question::*;
use chrono::NaiveDateTime;
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize)]
#[display(
    fmt = "id: {}, content: {}, applied_to_question_id: {:?}, applied_to_answer_id: {:?}",
    id,
    content,
    applied_to_question_id,
    applied_to_answer_id
)]
pub struct Comment {
    pub id: CommentId,
    pub content: String,
    pub applied_to_question_id: Option<QuestionId>,
    pub applied_to_answer_id: Option<AnswerId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentDbResult {
    pub id: i32,
    pub content: String,
    pub applied_to_question_id: Option<i32>,
    pub applied_to_answer_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentResult {
    pub id: i32,
    pub content: String,
    pub created_on: NaiveDateTime,
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommentId(pub u32);

impl From<u32> for CommentId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateComment {
    pub content: String,
    pub applied_to_question_id: QuestionId,
    pub applied_to_answer_id: AnswerId,
}
