use axum::extract::State;
use axum::Json;
use crate::db::AppDatabase;
use crate::error::AppError;
use crate::question::{Question, QuestionId};

pub async fn root() -> String {
    "Hello world!".to_string()
}



pub async fn get_questions(
    State(am_database): State<AppDatabase>
)-> Result<Json<Vec<Question>>, AppError> {

    let mut questions = am_database.questions.lock().unwrap();

    let db_count = questions.len() as usize;
    let question = Question::new(
        QuestionId(db_count),
        "Default question".to_string(),
        "Default Content".to_string(),
        Some("Default tag".to_string())
    );

    (*questions).push(question.clone());

    let all_questions: Vec<Question> = (*questions).clone();

    Ok(Json(all_questions))
}
