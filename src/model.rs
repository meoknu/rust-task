use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NodeModel {
    pub id: i32,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NodeModelResponse {
    pub id: i32,
    pub parent_id: Option<i32>
}
