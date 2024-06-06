use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::{Message, Image};

pub struct AI {
    model_name: String,
    max_new_tokens: Option<u16>,
    temperature: u8,
    timeout: u16,
    client: Client,
}

impl AI {
    pub fn new(
        model_name: &str,
        max_new_tokens: Option<u16>,
        temperature: u8,
        timeout: u16,
    ) -> Self {
        AI {
            model_name: model_name.to_string(),
            max_new_tokens,
            temperature,
            timeout,
            client: Client::new(),
        }
    }

    pub async fn call(
        &self,
        message: &str,
        max_new_tokens: Option<u16>,
        temperature: Option<u8>,
        timeout: Option<u16>,
        images: Option<Vec<Image>>,
    ) -> String {
        let max_tokens = max_new_tokens.unwrap_or(self.max_new_tokens.unwrap_or(1024));
        let temperature = temperature.unwrap_or(self.temperature);
        let timeout = timeout.unwrap_or(self.timeout);

        let msg = Message {
            text: Some(message.to_string()),
            role: "user".to_string(),
            tool_call: None,
            images,
        };

        let request_body = serde_json::json!({
            "model": self.model_name,
            "max_tokens": max_tokens,
            "messages": [msg]
        });

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", "ANTHROPIC_API_KEY") // replace ANTHROPIC_API_KEY with your actual API key
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .body(request_body.to_string())
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    match res.text().await {
                        Ok(text) => text,
                        Err(_) => "Failed to get response".to_string(),
                    }
                } else {
                    "Request failed".to_string()
                }
            },
            Err(_) => "Request timed out".to_string(),
        }
    }
}
