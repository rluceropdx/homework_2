use axum::Json;
use std::sync::{Arc, Mutex, RwLock};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

use crate::answer::{Answer, AnswerId, AnswerResult};
use crate::comment::{CommentDbResult, CommentResult};
use crate::error::AppError;
use crate::question::{IntoQuestionId, Question, QuestionId, QuestionResult, UpdateQuestion};

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

    pub async fn add_answer(
        &mut self,
        content: String,
        question_id: i32,
    ) -> Result<Answer, AppError> {
        let res = sqlx::query!(
            r#"
    INSERT INTO answers (content, question_id)
    VALUES ($1, $2)
    RETURNING *
    "#,
            content,
            question_id,
        )
        .fetch_one(&self.conn_pool)
        .await?;

        let answer = Answer {
            id: AnswerId(res.id),
            content: res.content,
            question_id: QuestionId(res.question_id.unwrap()),
        };

        Ok(answer)
    }

    pub async fn get_all_questions(&mut self) -> Result<Vec<Question>, AppError> {
        let rows = sqlx::query!(
            r#"
SELECT * FROM questions
"#
        )
        .fetch_all(&self.conn_pool)
        .await?;

        let questions: Vec<_> = rows
            .into_iter()
            .map(|row| {
                Question {
                    id: row.id.into(), // Assuming you have a From<u32> for QuestionId
                    title: row.title,
                    content: row.content,
                    tags: row.tags,
                }
            })
            .collect();

        Ok(questions)
    }

    pub async fn get_question_by_id<T: IntoQuestionId>(
        &mut self,
        id: T,
    ) -> Result<Question, AppError> {
        let id = id.into_question_id();

        let row = sqlx::query!(
            r#"
    SELECT * FROM questions WHERE id = $1
    "#,
            id.0,
        )
        .fetch_one(&self.conn_pool)
        .await?;

        let question = Question {
            id: row.id.into(), // Assuming you have a From<u32> for QuestionId
            title: row.title,
            content: row.content,
            tags: row.tags,
        };

        Ok(question)
    }

    pub async fn get_question_comments(&mut self, question_id: i32) -> QuestionResult {
        // return the Question (from its ID, given by the user/frontend like get_question_by_id)
        // Answers given to the Question
        // and Comments to both the Question and any Answers.

        let q_row = sqlx::query!(
            r#"
                select q.id, q.title, q.content, q.tags, q.created_on
                from questions q
                where q.id = $1
            "#,
            question_id,
        )
        .fetch_one(&self.conn_pool)
        .await
        .unwrap();

        let c_rows = sqlx::query!(
            r#"
                select c.id, c.content, c.created_on
                from questions q, comments c
                where q.id = c.applied_to_question_id
                and q.id = $1
                and c.applied_to_answer_id is NULL
                order by c.created_on desc
            "#,
            question_id,
        )
        .fetch_all(&self.conn_pool)
        .await
        .unwrap();

        let a_rows = sqlx::query!(
            r#"
                select a.id, a.content, a.created_on
                from answers a
                where a.id = $1
                order by a.created_on desc
            "#,
            question_id,
        )
        .fetch_all(&self.conn_pool)
        .await
        .unwrap();

        let ac_rows = sqlx::query!(
            r#"
                select c.id, c.content, c.created_on
                from answers a, comments c
                where a.id = c.applied_to_answer_id
                and a.id = $1
                and c.applied_to_question_id is NULL
                order by c.created_on desc
            "#,
            question_id,
        )
        .fetch_all(&self.conn_pool)
        .await
        .unwrap();

        let answer_comments: Vec<CommentResult> = ac_rows
            .into_iter()
            .map(|row1| CommentResult {
                id: row1.id,
                content: row1.content,
                created_on: row1.created_on,
            })
            .collect();

        QuestionResult {
            id: q_row.id,
            title: q_row.title,
            content: q_row.content,
            tags: q_row.tags,
            created_on: Default::default(),
            comments: c_rows
                .into_iter()
                .map(|row| CommentResult {
                    id: row.id,
                    content: row.content,
                    created_on: row.created_on,
                })
                .collect(),
            answers: a_rows
                .into_iter()
                .map(|row| AnswerResult {
                    id: row.id,
                    content: row.content,
                    created_on: Default::default(),
                    comments: answer_comments.clone(),
                })
                .collect(),
        }
    }

    pub async fn add_question(
        &mut self,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Result<Json<Question>, AppError> {
        let res = sqlx::query!(
            r#"INSERT INTO "questions"(title, content, tags)
           VALUES ($1, $2, $3)
           RETURNING *
        "#,
            title,
            content,
            tags.as_deref()
        )
        .fetch_one(&self.conn_pool)
        .await?;

        let new_question = Question {
            id: QuestionId(res.id),
            title: res.title,
            content: res.content,
            tags: res.tags,
        };

        Ok(Json(new_question))
    }

    pub async fn update_question(
        &mut self,
        new_question: UpdateQuestion,
    ) -> Result<Question, AppError> {
        sqlx::query!(
            r#"
    UPDATE questions
    SET title = $1, content = $2, tags = $3
    WHERE id = $4
    "#,
            new_question.title,
            new_question.content,
            new_question.tags.as_deref(),
            new_question.id.0,
        )
        .execute(&self.conn_pool)
        .await?;

        let row = sqlx::query!(
            r#"
SELECT title, content, id, tags FROM questions WHERE id = $1
"#,
            new_question.id.0,
        )
        .fetch_one(&self.conn_pool)
        .await?;

        let question = Question {
            title: row.title,
            content: row.content,
            id: QuestionId(row.id),
            tags: row.tags,
        };

        Ok(question)
    }

    pub async fn delete_question(&mut self, question_id: i32) -> Result<(), AppError> {
        let question_id = question_id.into_question_id();
        println!("DELETE - Question id is {}", &question_id);
        sqlx::query!(
            r#"
    DELETE FROM questions WHERE id = $1
    "#,
            question_id.0,
        )
        .execute(&self.conn_pool)
        .await
        .unwrap();

        Ok(())
    }

    pub async fn add_comment(
        &mut self,
        content: String,
        applied_to_question_id: Option<QuestionId>,
        applied_to_answer_id: Option<AnswerId>,
    ) -> Result<CommentDbResult, AppError> {
        let mut result: CommentDbResult = CommentDbResult {
            id: 0,
            content: "".to_string(),
            applied_to_question_id: None,
            applied_to_answer_id: None,
        };

        if applied_to_question_id.unwrap().0 > 0 {
            result = sqlx::query_as!(
                CommentDbResult,
                r#"INSERT INTO "comments"(content, applied_to_question_id)
                   VALUES ($1, $2)
                   RETURNING id, content, applied_to_question_id, applied_to_answer_id
                "#,
                content,
                applied_to_question_id.unwrap().0 as i32
            )
            .fetch_one(&self.conn_pool)
            .await?;
        } else if applied_to_answer_id.unwrap().0 > 0 {
            result = sqlx::query_as!(
                CommentDbResult,
                r#"INSERT INTO "comments"(content, applied_to_answer_id)
                   VALUES ($1, $2)
                   RETURNING id, content, applied_to_question_id, applied_to_answer_id
                "#,
                content,
                applied_to_answer_id.unwrap().0 as i32
            )
            .fetch_one(&self.conn_pool)
            .await?;
        }

        Ok(result)
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
