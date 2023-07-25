use http::{Request, StatusCode};
use hyper::Body;
use sqlx::PgPool;
use tower::ServiceExt;

use backend::answer::CreateAnswer;
use backend::question::{CreateQuestion, Question};
use backend::routes::app;

#[sqlx::test(fixtures("questions"))]
async fn test_add_question(db_pool: PgPool) {
    let mut app = app(db_pool).await;

    let question = CreateQuestion {
        title: "New Title".into(),
        content: "Test content2".into(),
        tags: None,
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/question")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&question).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("questions"))]
async fn test_get_questions(db_pool: PgPool) {
    let app = app(db_pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/questions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let questions: Vec<Question> = serde_json::from_slice(&body).unwrap();
    assert!(!questions.is_empty());
}

#[sqlx::test(fixtures("questions"))]
async fn test_get_question_by_id(db_pool: PgPool) {
    let app = app(db_pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/question/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let question: Question = serde_json::from_slice(&body).unwrap();
    assert_eq!(question.id.0, 1);
}

#[sqlx::test(fixtures("questions"))]
async fn test_update_question(db_pool: PgPool) {
    let mut app = app(db_pool).await;

    let updated_question = Question {
        id: 1.into(),
        title: "Updated Title".into(),
        content: "Updated content".into(),
        tags: None,
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PUT)
                .uri("/question")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&updated_question).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("questions"))]
async fn test_delete_question(db_pool: PgPool) {
    println!("In test delete");
    let app = app(db_pool).await;

    let query_uri = format!("/question?question_id=1");

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(query_uri)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    dbg!("DELETED QUESTION RESPONSE");
    dbg!(&response);
    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("questions", "answers"))]
async fn test_create_answer(db_pool: PgPool) {
    let app = app(db_pool).await;

    let answer = CreateAnswer {
        content: "New Answer".into(),
        question_id: 1i32,
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/answer")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&answer).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
