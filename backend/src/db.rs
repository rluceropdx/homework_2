use std::sync::{Arc, Mutex, RwLock};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower::util::ServiceExt;
use tracing::info;

use crate::answer::Answer;
use crate::error::{AppError, QuestionError};
use crate::question::{Question, QuestionId};

#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
    pub questions: Arc<Mutex<Vec<Question>>>,
    pub answers: Arc<RwLock<Vec<Answer>>>,
}

pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

impl Store {
    pub fn with_pool(pool: PgPool) -> Self {
        Self {
            conn_pool: pool,
            questions: Default::default(),
            answers: Default::default(),
        }
    }

    pub async fn test_database(&self) -> Result<(), sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.conn_pool)
            .await?;

        info!("{}", &row.0);

        assert_eq!(row.0, 150);
        Ok(())
    }

    pub fn add_answer(
        &mut self,
        content: String,
        question_id: QuestionId,
    ) -> Result<Answer, AppError> {
        let mut answer = self.answers.write().unwrap();
        let len = answer.len() as u32;

        let new_answer = Answer {
            id: len.into(),
            content,
            question_id,
        };
        answer.push(new_answer.clone());
        Ok(new_answer)
    }

    pub async fn add_question(
        &mut self,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"INSERT INTO "questions"(title, content, tags)
               VALUES ($1, $2, $3)
            "#,
            title,
            content,
            tags.as_deref()
        )
        .execute(&self.conn_pool)
        .await?;

        Ok(())
    }

    pub fn get_all_questions(&self) -> Vec<Question> {
        self.questions.lock().unwrap().clone()
    }

    pub fn get_question_by_id(&self, id: QuestionId) -> Result<Question, AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");
        let question = questions.iter().find(|q| q.id == id).cloned();

        question.ok_or(AppError::Question(QuestionError::InvalidId))
    }

    pub fn update_question(&mut self, new_question: Question) -> Result<Question, AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");

        let index = new_question.id.0;

        if index as usize >= questions.len() {
            return Err(AppError::Question(QuestionError::InvalidId));
        }

        questions[index as usize] = new_question.clone();

        Ok(new_question)
    }

    pub fn delete_question(&mut self, question_id: QuestionId) -> Result<(), AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");

        questions.retain(|q| q.id != question_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
