use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    id: String,
    parameters: HashMap<String, String>,
    result: Option<String>,
    name: String,
}

impl ToolCall {
    pub fn format(&self) -> String {
        format!(
            "Called {} with ({:?}) and got {:?}",
            self.name, self.parameters, self.result
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    text: Option<String>,
    role: Role,
    tool_call: Option<ToolCall>,
    images: Option<Vec<Image>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
    format: String, // "PNG", "JPEG"
} // Placeholder for image type

fn main() {
    let tool_call = ToolCall {
        id: "1".to_string(),
        parameters: HashMap::new(),
        result: None,
        name: "Tool".to_string(),
    };
    let message = Message {
        text: Some("Who let the dogs out ?".to_string()),
        role: Role::User,
        tool_call: Some(tool_call),
        images: Some(vec![Image {
            data: vec![],
            width: 100,
            height: 100,
            format: "PNG".to_string(),	
        }]),
    };
    println!("{:?}", message);
}