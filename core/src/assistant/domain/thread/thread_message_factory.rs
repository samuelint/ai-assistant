#[cfg(test)]
#[path = "./thread_message_factory_test.rs"]
mod thread_message_factory_test;

use std::sync::Arc;

use crate::assistant::domain::{
    dto::{DbCreateThreadMessageDto, ThreadMessageDto},
    message::MessageRepository,
};

pub struct ThreadMessageFactory {
    message_repository: Arc<dyn MessageRepository>,
}

impl ThreadMessageFactory {
    pub fn new(message_repository: Arc<dyn MessageRepository>) -> Self {
        Self { message_repository }
    }

    pub async fn create_assistant(
        &self,
        thread_id: &str,
        run_id: &str,
    ) -> Result<ThreadMessageDto, Box<dyn std::error::Error + Send>> {
        let message = self
            .message_repository
            .create(DbCreateThreadMessageDto {
                role: "assistant".to_string(),
                thread_id: Some(thread_id.to_string()),
                run_id: Some(run_id.to_string()),
                ..DbCreateThreadMessageDto::default()
            })
            .await?;

        Ok(message)
    }
}
