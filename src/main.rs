// main.rs

mod ai_client;
mod image_processing;
mod messaging;
mod tool_call;

use ai_client::{AI, AsyncAI};
use image_processing::Image;
use messaging::{Message, Role};
use std::collections::HashMap;
use tool_call::ToolCall;

#[tokio::main]
async fn main() {
    let tool_call = ToolCall {
        id: "1".to_string(),
        parameters: HashMap::new(),
        result: None,
        name: "example_tool".to_string(),
    };

    let img = image_processing::open_image("tiny_ai_rs\dino.jpeg")
        .expect("Failed to open image");
    let mut image = Image::new(img);

    image.resize(100, 100);

    image.save("tiny_ai_rs\dino1.jpeg")
        .expect("Failed to save image");

    let message = Message {
        text: Some("Example message".to_string()),
        role: Role::User,
        tool_call: Some(tool_call),
        images: Some(vec![image]),
    };

    println!("{:?}", message);

    let ai_client = AI::new("gpt-3.5-turbo", None, 1, None, 30, None);
    let response = ai_client
        .call(
            "Hello, AI!",
            Some(100),
            Some(1),
            Some(30),
            Some(vec![image_processing::load_image("tiny_ai_rs\dino.jpeg").unwrap()]),
        )
        .await;
    
    println!("AI Response: {}", response);
}
