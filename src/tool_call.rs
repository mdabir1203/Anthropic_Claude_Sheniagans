// tool_call.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub parameters: HashMap<String, String>,
    pub result: Option<String>,
    pub name: String,
}

impl ToolCall {
    pub fn format(&self) -> String {
        format!(
            "Called {} with ({:?}) and got {:?}",
            self.name, self.parameters, self.result
        )
    }
}
