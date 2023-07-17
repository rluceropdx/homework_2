use axum::extract::{Path, Query, State};
use axum::Json;

use crate::db::Store;
use crate::error::AppError;
use crate::question::{CreateQuestion, GetQuestionById, Question, QuestionId};

#[allow(dead_code)]
pub async fn root() -> String {
    "Hello world!".to_string()
}

// CRUD create - read - update - delete
pub async fn get_questions(
    State(am_database): State<Store>
) -> Result<Json<Vec<Question>>, AppError> {
    let all_questions = am_database.get_all_questions();

    Ok(Json(all_questions))
}

pub async fn get_question_by_id(
    State(am_database): State<Store>,
    Path(query): Path<u32>,    // localhost:3000/question/5
) -> Result<Json<Question>, AppError> {
    let question = am_database.get_question_by_id(QuestionId(query))?;
    Ok(Json(question))
}

pub async fn create_question(
    State(mut am_database): State<Store>,
    Json(question): Json<CreateQuestion>,
) -> Result<Json<()>, AppError> {
    am_database.add_question(question.title, question.content, question.tags).await?;

    Ok(Json(())) // ORM - object relational mapper
}

pub async fn update_question(
    State(mut am_database): State<Store>,
    Json(question): Json<Question>,
) -> Result<Json<Question>, AppError> {
   let updated_question = am_database.update_question(question)?;
    Ok(Json(updated_question))
}

pub async fn delete_question(
    State(mut am_database): State<Store>,
    Query(query): Query<GetQuestionById> // /question?question_id=1
) -> Result<(), AppError> {
    am_database.delete_question(QuestionId(query.question_id))?;

    Ok(())
}
