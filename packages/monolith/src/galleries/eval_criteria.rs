use std::iter::zip;

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
    
    /// Parse a list of answers for the evaluation criteria and returns them.
    /// 
    /// The list should be in the same order as the criteria.
    /// 
    /// Returns an `Err` if `answers` is not the same length as the criteria,
    /// or an answer is not in the expected format for its criterion.
    pub fn parse_answers(&mut self, answers: Vec<String>) -> Result<Vec<CriterionAnswer>, String> {
        if answers.len() != self.criteria.len() {
            return Err(format!("Expected {} answers, got {}", self.criteria.len(), answers.len()));
        }
        let result: Result<Vec<_>, _> = zip(&mut self.criteria, answers)
            .map(|(criterion, answer)| criterion.parse_answer(answer))
            .collect();
        result
    }

    /// A string that describes each question and how to answer it.
    /// 
    /// This is passed to the LLM in item analysis, to ensure a correctly structured response.
    pub fn describe_criteria(&self) -> String {
        self.criteria
            .iter()
            .map(|criterion| return match criterion.criterion_type {
                CriterionType::YesNo => {
                    format!("- {} (ONLY ANSWER WITH 'Y' for Yes, or 'N' for No) \n", criterion.question)
                },
                CriterionType::YesNoUncertain => {
                    format!("- {} (ONLY ANSWER WITH 'Y' for Yes, 'N' for No, or 'U' for Uncertain) \n", criterion.question)
                },
                CriterionType::Int => {
                    format!("- {} (ONLY ANSWER WITH AN INTEGER NUMBER) \n", criterion.question)
                },
                CriterionType::Float => {
                    format!("- {} (ONLY ANSWER WITH A FLOATING POINT NUMBER) \n", criterion.question)
                },
                CriterionType::OpenEnded => {
                    format!("- {} (ANSWER WITH ANYTHING UNDER 200 CHARACTERS) \n", criterion.question) // arbitrary character limit, should be okay
                },
            })
            .collect()
    }
}

/// A criterion, consisting of its question and the type of question it is (ie, how it should be answered).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Criterion {
    question: String,
    criterion_type: CriterionType
}

/// The different possible types of criterion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CriterionType {
    YesNo,
    YesNoUncertain,
    Int,
    Float,
    OpenEnded
}

impl Criterion {
    /// Parses an answer string and sets the criterion's answer parameter to a `Some` if it's valid.
    /// 
    /// Returns an `Err` if the answer cannot be parsed into the criterion.
    pub fn parse_answer(&mut self, answer: String) -> Result<CriterionAnswer, String> {
        Ok(
            match self.criterion_type {
                CriterionType::YesNo => {
                    match answer.as_str() {
                        "Y" | "y" => CriterionAnswer::YesNo(YesNo::Yes),
                        "N" | "n" => CriterionAnswer::YesNo(YesNo::No),
                        _ => return Err(format!("Answer to question: '{}', is not Y/N or y/n ({})", self.question, answer))
                    }
                },
                CriterionType::YesNoUncertain => {
                    match answer.as_str() {
                        "Y" | "y" => CriterionAnswer::YesNoUncertain(YesNoUncertain::Yes),
                        "N" | "n" => CriterionAnswer::YesNoUncertain(YesNoUncertain::No),
                        "U" | "u" => CriterionAnswer::YesNoUncertain(YesNoUncertain::Uncertain),
                        _ => return Err(format!("Answer to question: '{}', is not Y/N/U or y/n/u ({})", self.question, answer))
                    }
                },
                CriterionType::Float => {
                    let val = answer.parse::<f64>()
                        .map_err(|_| format!("Answer to question: '{}', cannot be parsed to float ({})", self.question, answer))?;
                    CriterionAnswer::Float(val)
                },
                CriterionType::Int => {
                    let val = answer.parse::<usize>()
                        .map_err(|_| format!("Answer to question: '{}', cannot be parsed to integer ({})", self.question, answer))?;
                    CriterionAnswer::Int(val)
                },
                CriterionType::OpenEnded => {
                    CriterionAnswer::OpenEnded(answer)
                }
            }
        )
    }
}

/// The possible answers for each criterion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CriterionAnswer {
    YesNo(YesNo),
    YesNoUncertain(YesNoUncertain),
    Int(usize),
    Float(f64),
    OpenEnded(String)
}

/// A Yes/No question. Useful for finding out things that should be obvious, like "is this a shirt?".
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YesNoQuestion {
    pub question: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum YesNo { Yes, No }

/// Similar to Yes/No, but allows the LLM to pick `Uncertain` if the answer is not obvious, like "is this mug from before 2010?".
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct YesNoUncertainQuestion {
    pub question: String
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
