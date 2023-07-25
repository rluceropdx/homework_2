use axum::extract::{Path, Query, State};
use axum::Json;

use crate::answer::{Answer, CreateAnswer};
use crate::db::Store;
use crate::error::AppError;
use crate::question::{CreateQuestion, GetQuestionById, Question, QuestionId, UpdateQuestion};

#[allow(dead_code)]
pub async fn root() -> String {
    "Hello world!".to_string()
}

// CRUD create - read - update - delete
pub async fn get_questions(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Question>>, AppError> {
    let all_questions = am_database.get_all_questions().await?;

    Ok(Json(all_questions))
}

pub async fn get_question_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>, // localhost:3000/question/5
) -> Result<Json<Question>, AppError> {
    let question = am_database.get_question_by_id(QuestionId(query)).await?;
    Ok(Json(question))
}

pub async fn create_question(
    State(mut am_database): State<Store>,
    Json(question): Json<CreateQuestion>,
) -> Result<Json<()>, AppError> {
    let _ = am_database
        .add_question(question.title, question.content, question.tags)
        .await?;

    Ok(Json(()))
}

pub async fn update_question(
    State(mut am_database): State<Store>,
    Json(question): Json<UpdateQuestion>,
) -> Result<Json<Question>, AppError> {
    let updated_question = am_database.update_question(question).await?;
    Ok(Json(updated_question))
}

pub async fn delete_question(
    State(mut am_database): State<Store>,
    Query(query): Query<GetQuestionById>,
) -> Result<(), AppError> {
    am_database.delete_question(query.question_id).await?;

    Ok(())
}

pub async fn create_answer(
    State(mut am_database): State<Store>,
    Json(answer): Json<CreateAnswer>,
) -> Result<Json<Answer>, AppError> {
    dbg!("GOT CREATE ANSWER:");
    dbg!(&answer);
    let new_answer = am_database
        .add_answer(answer.content, answer.question_id)
        .await?;
    Ok(Json(new_answer))
}
