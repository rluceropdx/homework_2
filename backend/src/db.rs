use std::sync::{Arc, Mutex};
use crate::error::{AppError, QuestionError};
use crate::question::{Question, QuestionId};

#[derive(Clone, Default)]
pub struct Store {
    pub questions: Arc<Mutex<Vec<Question>>>,
}

impl Store {
    pub fn new() -> Self {
        Store::default()
    }

    pub fn init(&mut self) -> Result<(), AppError> {

        let tags = Some(vec!["general".to_string()]).unwrap();
        self.add_question("How do i?".to_string(), "Help me!".to_string(), Some(tags) )?;
        Ok(())
    }

    pub fn add_question(&mut self, title: String, content: String, tags: Option<Vec<String>>) -> Result<Question, AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");

        let len = questions.len();

        let new_question = Question::new(QuestionId(len as u32), title, content, tags);
        questions.push(new_question.clone());
        Ok(new_question)

    }

    pub fn get_all_questions(&self) -> Vec<Question> {
        self.questions.lock().unwrap().clone()
    }

    pub fn get_question_by_id(&self, id: QuestionId) -> Result<Question, AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");
        let question = questions.iter().find( |q| q.id == id).cloned();

        question.ok_or(AppError::Question(QuestionError::InvalidId))
    }

    pub fn update_question(&mut self, new_question: Question) -> Result<Question, AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");

        let index = new_question.id.0;

        if index as usize >= questions.len() {
            return Err(AppError::Question(QuestionError::InvalidId))
        }

        questions[index as usize] = new_question.clone();

        Ok(new_question)
    }

    pub fn delete_question(&mut self, question_id: QuestionId) -> Result<(), AppError> {
        let mut questions = self.questions.lock().expect("Poisoned mutex");

        questions.retain( |q| q.id.0 != question_id);

        Ok(())
    }
}

