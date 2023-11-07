use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNodeSchema {
    pub parent_id: Option<i32>
}