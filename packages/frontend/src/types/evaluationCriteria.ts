// Enum types
export enum CriterionType {
    YesNo = "YesNo",
    YesNoUncertain = "YesNoUncertain",
    Int = "Int",
    Float = "Float",
    OpenEnded = "OpenEnded"
}

export enum YesNo {
    Yes = "Yes",
    No = "No"
}

export enum YesNoUncertain {
    Yes = "Yes",
    No = "No",
    Uncertain = "Uncertain"
}

// Numerical hard criterion types
export type IntHardCriterion = NumericalHardCriterion;
export type FloatHardCriterion = NumericalHardCriterion;

// Union type for hard criterion
export type HardCriterion = 
    | { YesNo: YesNo }
    | { Int: IntHardCriterion }
    | { Float: FloatHardCriterion };

// Numerical hard criterion 
export type NumericalHardCriterion = 
    | { LessThan: number }
    | { Equal: number }
    | { MoreThan: number }
    | { Between: [number, number] };

// An answer to a criterion.
export type CriterionAnswer = 
  | { YesNo: YesNo }
  | { YesNoUncertain: YesNoUncertain }
  | { Int: number }
  | { Float: number }
  | { OpenEnded: string };

// A criterion to evaluate a gallery upon.
export interface Criterion {
    question: string;
    criterion_type: CriterionType;
    hard_criterion?: HardCriterion;
}

// All criteria for a gallery.
export interface EvaluationCriteria {
  criteria: Criterion[];
}