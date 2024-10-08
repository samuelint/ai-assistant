#[cfg(test)]
#[path = "./message_delta_test.rs"]
mod message_delta_test;

use serde::{Deserialize, Serialize};

use crate::chat_completion::{ApiTextContent, ImageUrl};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum MessageDeltaContent {
    Text {
        index: i32,
        #[serde(rename = "type")]
        type_: String,
        text: ApiTextContent,
    },
    ImageUrl {
        index: i32,
        #[serde(rename = "type")]
        type_: String,
        image_url: ImageUrl,
    },
}

impl MessageDeltaContent {
    pub fn text(text: &str) -> Self {
        Self::Text {
            index: 0,
            type_: "text".to_string(),
            text: ApiTextContent::annotated(text),
        }
    }

    pub fn image_url(url: &str) -> Self {
        Self::ImageUrl {
            index: 0,
            type_: "image_url".to_string(),
            image_url: ImageUrl::url(url),
        }
    }
}

impl Default for MessageDeltaContent {
    fn default() -> Self {
        Self::Text {
            index: 0,
            type_: "text".to_string(),
            text: ApiTextContent::Annotated {
                value: "".to_string(),
                annotations: vec![],
            },
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MessageDeltaDto {
    pub role: String,
    pub content: Vec<MessageDeltaContent>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ThreadMessageDeltaDto {
    pub id: String,
    pub delta: MessageDeltaDto,
}
