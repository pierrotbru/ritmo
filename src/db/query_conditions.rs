// File: query_conditions.rs

#[derive(Debug, Clone, Copy)]
pub enum LikeType {
    Contains,
    StartsWith,
    EndsWith,
}

#[derive(Debug, Clone)]
pub enum ConditionType {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct CombinedCondition {
    pub conditions: Vec<QueryCondition>,
    pub condition_type: ConditionType,
}

#[derive(Debug, Clone)]
pub enum QueryCondition {
    Equals(String, String),
    NotEquals(String, String),
    Like(String, String, LikeType),
    In(String, Vec<String>),
    GreaterThan(String, String),
    LessThan(String, String),
    GreaterThanOrEqual(String, String),
    LessThanOrEqual(String, String),
    Between(String, String, String),
    IsNull(String),
    IsNotNull(String),
    Combined(CombinedCondition),
}

