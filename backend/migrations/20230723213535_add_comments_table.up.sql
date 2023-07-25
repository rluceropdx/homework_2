-- Add up migration script here
CREATE TABLE IF NOT EXISTS comments
(
    id         serial PRIMARY KEY,
    content    TEXT         NOT NULL,
    created_on TIMESTAMP    NOT NULL DEFAULT NOW(),
    applied_to_question_id integer REFERENCES questions NULL,
    applied_to_answer_id integer REFERENCES answers NULL
);
