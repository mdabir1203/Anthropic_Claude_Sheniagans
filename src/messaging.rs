// messaging.rs

use crate::tool_call::ToolCall;
use crate::image_processing::Image;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub text: Option<String>,
    pub role: Role,
    pub tool_call: Option<ToolCall>,
    pub images: Option<Vec<Image>>,
}
