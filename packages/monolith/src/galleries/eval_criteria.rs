use std::iter::zip;

use serde::{Serialize, Deserialize};

/// A Vec of user-defined questions to ask the LLM about each item in a gallery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    criteria: Vec<Criterion>
}

impl EvaluationCriteria {
    /// Initialize with a list of criteria.
    pub fn new(criteria: Vec<Criterion>) -> Self {
        EvaluationCriteria {
            criteria
        }
    }

    /// A string that describes each question and how to answer it.
    /// This is passed to the LLM in item analysis, to ensure a correctly structured response.
    /// 
    /// If there are no criteria, the description simply states that there are no questions.
    pub fn describe_criteria(&self) -> String {
        if self.criteria.len() == 0 {
            return "THERE ARE NO QUESTIONS".into();
        }
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
                    format!("- {} (ONLY ANSWER WITH AN INTEGER NUMBER AS A STRING) \n", criterion.question)
                },
                CriterionType::Float => {
                    format!("- {} (ONLY ANSWER WITH A FLOATING POINT NUMBER AS A STRING) \n", criterion.question)
                },
                CriterionType::OpenEnded => {
                    format!("- {} (ANSWER WITH ANYTHING UNDER 200 CHARACTERS) \n", criterion.question) // arbitrary character limit, should be okay
                },
            })
            .collect()
    }

    /// Parses a list of answer strings into `CriterionAnswer`s and checks whether all answers satisfy hard criteria (if any).
    /// 
    /// This is just a combination of the `parse_answers` and `satisfies_hard_criteria` methods, for convenience.
    /// 
    /// Check them for details on return values/error cases.
    pub fn parse_answers_and_check_hard_criteria(&self, answers: Vec<String>) -> Result<(Vec<CriterionAnswer>, bool), String> {
        let parsed_answers = self.parse_answers(answers)?;
        let satisfies_hard_criteria = self.satisfies_hard_criteria(&parsed_answers)?;
        Ok((parsed_answers, satisfies_hard_criteria))
    }
    
    /// Parse a list of answers for the evaluation criteria and returns them.
    /// 
    /// The list should be in the same order as the criteria.
    /// 
    /// Returns an `Err` if `answers` is not the same length as the criteria,
    /// or an answer is not in the expected format for its criterion.
    pub fn parse_answers(&self, answers: Vec<String>) -> Result<Vec<CriterionAnswer>, String> {
        if answers.len() != self.criteria.len() {
            return Err(format!("Expected {} answers, got {}", self.criteria.len(), answers.len()));
        }
        let result: Result<Vec<_>, _> = zip(&self.criteria, answers)
            .map(|(criterion, answer)| criterion.parse_answer(answer))
            .collect();
        result
    }

    /// Returns whether all the answers satisfy all `HardCriterion`s present in the criteria.
    /// 
    /// If none of the criteria have `HardCriterion`, simply returns `true`.
    /// 
    /// Returns an `Err` if any answer type doesn't match the corresponding criterion type.
    /// If you directly pass the successful output from `parse_answers`, this will never occur.
    pub fn satisfies_hard_criteria(&self, answers: &Vec<CriterionAnswer>) -> Result<bool, String> {
        zip(&self.criteria, answers)
            .try_fold(true, |prev, (criterion, answer)| {
                match criterion.satisfies_hard_criterion(&answer) {
                    Ok(val) => Ok(val && prev),
                    Err(err) => Err(err)
                }
            })
    } 
}

/// A criterion, consisting of:
/// - Its question,
/// - The type of question it is (ie, how it should be answered), and
/// - An optional "hard filter" for the answer called the `HardCriterion`.
/// 
/// The hard criterion is available for `YesNo`, `Float` and `Int` criterion.
/// 
/// It specifies a mandatory answer or answer range that, if not satisfied,
/// means the item should be discarded from the gallery.
/// 
/// It is most useful for filtering out things that you objectively don't want,
/// and can be (almost) objectively answered by the LLM, such as "is this a shirt?".
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Criterion {
    question: String,
    criterion_type: CriterionType,
    hard_criterion: Option<HardCriterion>
}

impl Criterion {
    /// Parses an answer string and returns the corresponding `CriterionAnswer`.
    /// 
    /// Returns an `Err` if the answer cannot be parsed into the criterion.
    fn parse_answer(&self, answer: String) -> Result<CriterionAnswer, String> {
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

    /// Returns whether the answer satisfies the hard criterion.
    /// 
    /// Returns `true` if the hard criterion is `None`, or the answer type doesn't have a hard criterion.
    /// 
    /// Returns an `Err` if the answer type has a hard criterion type, but they don't match (ie `YesNo` and `Int`).
    fn satisfies_hard_criterion(&self, answer: &CriterionAnswer) -> Result<bool, String> {
        match &self.hard_criterion {
            Some(hard_criterion) => hard_criterion.is_satisfied(answer),
            None => Ok(true),
        }
    }
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

/// The possible answers for each criterion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CriterionAnswer {
    YesNo(YesNo),
    YesNoUncertain(YesNoUncertain),
    Int(usize),
    Float(f64),
    OpenEnded(String)
}

/// The possible types of hard criterion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HardCriterion {
    YesNo(YesNo),
    Int(IntHardCriterion),
    Float(FloatHardCriterion),
}

impl HardCriterion {
    /// Returns whether the answer satisfies the hard criterion.
    /// 
    /// Returns `true` if the answer type is not `YesNo`/`Int`/`Float`.
    /// 
    /// Returns an `Err` if it is, but it doesn't match the hard criterion type.
    fn is_satisfied(&self, answer: &CriterionAnswer) -> Result<bool, String> {
        match (self, answer) {
            (HardCriterion::YesNo(criterion), CriterionAnswer::YesNo(answer)) => {
                Ok(criterion == answer)
            },
            (HardCriterion::Int(criterion), CriterionAnswer::Int(answer)) => {
                Ok(criterion.is_satisfied(answer))
            },
            (HardCriterion::Float(criterion), CriterionAnswer::Float(answer)) => {
                Ok(criterion.is_satisfied(answer))
            },
            (_, CriterionAnswer::YesNoUncertain(_)) => {
                Ok(true)
            },
            (_, CriterionAnswer::OpenEnded(_)) => {
                Ok(true)
            },
            _ => Err(format!("Answer ({answer:?}) doesn't match hard criterion ({self:?}) type"))
        }
    }
}

/// The hard criterion for an `Int` question.
pub type IntHardCriterion = NumericalHardCriterion<usize>;

/// The hard criterion for a `Float` question.
pub type FloatHardCriterion = NumericalHardCriterion<f64>;

/// Generic for a numerical hard criterion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NumericalHardCriterion<T> 
where 
    T: PartialOrd + Copy,
{
    LessThan(T),
    Equal(T),
    MoreThan(T),
    Between(T, T),
}

impl<T> NumericalHardCriterion<T> 
where 
    T: PartialOrd + Copy,
{   
    /// Returns whether `answer` satisfies the hard criterion.
    pub fn is_satisfied(&self, answer: &T) -> bool {
        match self {
            NumericalHardCriterion::LessThan(threshold) => answer < threshold,
            NumericalHardCriterion::Equal(target) => answer == target,
            NumericalHardCriterion::MoreThan(threshold) => answer > threshold,
            NumericalHardCriterion::Between(min, max) => answer >= min && answer <= max,
        }
    }
}

/// The response types for a `YesNo` question.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum YesNo { Yes, No }

/// The response types for a `YesNoUncertain` question.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum YesNoUncertain { Yes, No, Uncertain }
