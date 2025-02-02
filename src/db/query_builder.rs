// File: query_builder.rs
use sqlx::SqlitePool;
use crate::db::query_conditions::{
    LikeType, 
    ConditionType, 
    CombinedCondition, 
    QueryCondition, 
};
use crate::errors::{RitmoErr, QueryBuilderError, QueryError};

#[derive(Debug)]
pub struct Join {
    pub table: String,
    pub on_clause: String,
}

#[derive(Debug)]
pub struct QueryBuilder {
    pub table: String,
    pub select_columns: Vec<String>,
    pub conditions: Vec<QueryCondition>,
    pub group_by: Option<String>,
    pub having: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<u32>,
    pub joins: Vec<Join>,
}

impl QueryBuilder {
    pub fn new(table: &str) -> Self {
        QueryBuilder {
            table: table.to_string(),
            select_columns: Vec::new(),
            conditions: Vec::new(),
            group_by: None,
            having: None,
            order_by: None,
            limit: None,
            joins: Vec::new(),
        }
    }

    pub fn join(mut self, join: Join) -> Self {
        self.joins.push(join);
        self
    }

    pub fn select_columns(mut self, columns: &[&str]) -> Self {
        self.select_columns = columns.iter().map(|&s| s.to_string()).collect();
        self
    }

    pub fn add_condition(mut self, condition: QueryCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    fn build_combined_condition(&self, combined: &CombinedCondition) -> Result<(String, Vec<String>), QueryBuilderError> {
        let mut params = Vec::new();
        let mut conditions_str: Vec<String> = Vec::new();

        for condition in &combined.conditions {
            let (sub_condition_str, sub_condition_params) = self.format_condition(condition);
            params.extend(sub_condition_params);
            conditions_str.push(sub_condition_str);
        }

        Ok((format!("({})", conditions_str.join(match combined.condition_type {
            ConditionType::And => " AND ",
            ConditionType::Or => " OR ",
        })), params))
    }

    pub fn group_by(mut self, group_by: &str) -> Self { 
        self.group_by = Some(group_by.to_string());
        self
    }

    pub fn having(mut self, having: &str) -> Self { 
        self.having = Some(having.to_string());
        self
    }

    pub fn order_by(mut self, order_by: &str) -> Self { 
        self.order_by = Some(order_by.to_string());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    fn format_condition(&self, condition: &QueryCondition) -> (String, Vec<String>) {
        let mut params = Vec::new();
        let condition_str = match condition {
            QueryCondition::Equals(col, val) => {
                params.push(val.clone());
                format!("{} = ?", col)
            }
            QueryCondition::NotEquals(col, val) => {
                params.push(val.clone());
                format!("{} != ?", col)
            }
            QueryCondition::Like(col, val, like_type) => {
                let like_pattern = match like_type {
                    LikeType::Contains => format!("%{}%", val),
                    LikeType::StartsWith => format!("{}%", val),
                    LikeType::EndsWith => format!("%{}", val),
                };
                params.push(like_pattern);
                format!("{} LIKE ?", col)
            }
            QueryCondition::In(col, values) => {
                if values.is_empty() {
                    return ("1=0".to_string(), params); // restituisce una condizione sempre falsa
                }
                let placeholders: Vec<String> = values.iter().map(|_| "?".to_string()).collect();
                for v in values {
                    params.push(v.clone());
                }
                format!("{} IN ({})", col, placeholders.join(", "))
            }
            QueryCondition::GreaterThan(col, val) => {
                params.push(val.clone());
                format!("{} > ?", col)
            }
            QueryCondition::LessThan(col, val) => {
                params.push(val.clone());
                format!("{} < ?", col)
            }
            QueryCondition::GreaterThanOrEqual(col, val) => {
                params.push(val.clone());
                format!("{} >= ?", col)
            }
            QueryCondition::LessThanOrEqual(col, val) => {
                params.push(val.clone());
                format!("{} <= ?", col)
            }
            QueryCondition::Between(col, val1, val2) => {
                params.push(val1.clone());
                params.push(val2.clone());
                format!("{} BETWEEN ? AND ?", col)
            }
            QueryCondition::IsNull(col) => format!("{} IS NULL", col),
            QueryCondition::IsNotNull(col) => format!("{} IS NOT NULL", col),
            QueryCondition::Combined(combined) => {
                let combined_conditions_str: Vec<String> = combined.conditions.iter().map(|c| {
                    let (cond_str, cond_params) = self.format_condition(c);
                    params.extend(cond_params);
                    cond_str
                }).collect();
                format!("({})", combined_conditions_str.join(match combined.condition_type {
                    ConditionType::And => " AND ",
                    ConditionType::Or => " OR ",
                }))
            }
        };
        (condition_str, params)
    }

    fn build_where_clause(&self) -> Result<(String, Vec<String>), QueryBuilderError> {
        if self.conditions.is_empty() {
            return Ok((String::new(), Vec::new()));
        }

        let mut params = Vec::new();
        let mut conditions_str: Vec<String> = Vec::new();

        for condition in &self.conditions {
            match condition {
                QueryCondition::Combined(combined) => {
                    let (sub_clause, sub_params) = self.build_combined_condition(combined)?;
                    conditions_str.push(sub_clause);
                    params.extend(sub_params);
                },
                _ => {
                    let (condition_str, condition_params) = self.format_condition(condition);
                    params.extend(condition_params);
                    conditions_str.push(condition_str);
                }
            }
        }

        Ok((format!(" WHERE {}", conditions_str.join(" AND ")), params))
    }

    pub fn build(&self) -> Result<(String, Vec<String>), RitmoErr> {
        if self.select_columns.is_empty() {
            return Err(RitmoErr::QueryBuilderError(QueryBuilderError::NoSelectColumns));
        }

        let mut query = format!("SELECT {} FROM {}", self.select_columns.join(", "), self.table);
        let mut params = Vec::new();

        let (where_clause, where_params) = self.build_where_clause()?;
        query.push_str(&where_clause);
        params.extend(where_params);

        if let Some(group_by) = &self.group_by {
            query.push_str(&format!(" GROUP BY {}", group_by));
        }

        if let Some(having) = &self.having {
            query.push_str(&format!(" HAVING {}", having));
        }

        if let Some(order_by) = &self.order_by {
            query.push_str(&format!(" ORDER BY {}", order_by));
        }

        if let Some(limit) = &self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        for join in &self.joins {
            query.push_str(&format!(" JOIN {} ON {}", join.table, join.on_clause));
        }

        Ok((query, params))
    }

    pub async fn execute<T>(&self, pool: &SqlitePool) -> Result<impl Iterator<Item = Result<T, RitmoErr>>, RitmoErr>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow>,
    {    
        let (query_string, params) = self.build()?;

        let mut query = sqlx::query(&query_string);
        for param in params {
            query = query.bind(param);
        }

        let rows = query.fetch_all(pool).await.map_err(|e| QueryError::DatabaseError(e))?;

        Ok(rows.into_iter().map(|row| {
            T::from_row(&row).map_err(|err| {
                // Assuming `err` is of type `Error`
                RitmoErr::UnknownError(err.to_string())
                })
            }))
    }

    pub async fn execute_count(&self, pool: &SqlitePool) -> Result<usize, RitmoErr>
    {    
        let (query_string, params) = self.build()?;

        let mut query = sqlx::query(&query_string);
        for param in params {
            query = query.bind(param);
        }

        let rows = query.fetch_all(pool).await.map_err(|e| QueryError::DatabaseError(e))?;
        Ok(rows.len())
    }
}
