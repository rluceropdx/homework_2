-- Add up migration script here
INSERT INTO comments(content, applied_to_question_id) VALUES ('comment to question 1', 1);

INSERT INTO comments(content, applied_to_answer_id) VALUES ('comment to answer 1', 1);
