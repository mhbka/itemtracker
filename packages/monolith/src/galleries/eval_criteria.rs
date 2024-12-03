use serde::{Serialize, Deserialize};

/// A Vec of user-defined questions to ask the LLM about each item in a gallery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    criteria: Vec<Criterion>
}

impl EvaluationCriteria {
    pub fn new(criteria: Vec<Criterion>) -> Self {
        EvaluationCriteria {
            criteria
        }
    }

    pub fn parse_answers(&mut self, answers: Vec<String>) -> Result<(), String> {
        if answers.len() != self.criteria.len() {
            return Err(format!("Expected {} answers, got {}", self.criteria.len(), answers.len()));
        }

        Ok(())
    }

    /// A string to pass to the LLM to explain question types,
    /// and how to answer them so that they can be parsed correctly.
    pub fn question_description() -> &'static str {
        "
        TODO: figure this out
        "
    }
}

/// Different types of criterion, each having different possible answer types.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Criterion {
    YesNo(YesNoQuestion),
    YesNoUncertain(YesNoUncertainQuestion),
    Numerical(NumericalQuestion),
    OpenEnded(OpenEndedQuestion)
}

impl Criterion {
    /// Parses an answer string and sets the criterion's answer parameter to a `Some` if it's valid.
    pub fn parse_answer(&mut self, answer: String) -> Result<(), String> {
        match self {
            Criterion::YesNo(qn) => {
                match answer.as_str() {
                    "Y" | "y" => qn.answer = Some(YesNo::Yes),
                    "N" | "n" => qn.answer = Some(YesNo::No),
                    _ => return Err(format!("Answer to question: '{}', is not Y/N ({})", qn.question, answer))
                }
            },
            Criterion::YesNoUncertain(qn) => {
                match answer.as_str() {
                    "Y" | "y" => qn.answer = Some(YesNoUncertain::Yes),
                    "N" | "n" => qn.answer = Some(YesNoUncertain::No),
                    "U" | "u" => qn.answer = Some(YesNoUncertain::Uncertain),
                    _ => return Err(format!("Answer to question: '{}', is not Y/N/U ({})", qn.question, answer))
                }
            },
            Criterion::Numerical(qn) => {
                qn.answer = Some(
                    answer
                        .parse::<f32>()
                        .map_err(|_| format!("Answer to question: '{}', is not parsable to float ({})", qn.question, answer))?
                );
            },
            Criterion::OpenEnded(qn) => {
                qn.answer = Some(answer);
            },
        };
        Ok(())
    }
}

/// A Yes/No question. Useful for finding out things that should be easy, like "is this a shirt?".
/// 
/// Uniquely comes with a `pass_condition`; if set, the item must pass this criterion to be considered "valid".
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YesNoQuestion {
    pub question: String,
    pub pass_condition: Option<YesNo>,
    pub answer: Option<YesNo>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum YesNo { Yes, No }

/// Similar to Yes/No, but allows the LLM to pick Uncertain if the answer is not obvious, like "is this mug from before 2010?".
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YesNoUncertainQuestion {
    pub question: String,
    pub answer: Option<YesNoUncertain>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum YesNoUncertain { Yes, No, Uncertain }

/// A numerical question, like "what would you rate the condition from 0-10"?
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NumericalQuestion {
    pub question: String,
    pub answer: Option<f32>
}

/// An open-ended question.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenEndedQuestion {
    pub question: String,
    pub answer: Option<String>
}
