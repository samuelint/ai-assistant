use std::pin::Pin;

use app_core::assistant::domain::dto::{
    ApiCreateRunDto, ApiCreateThreadAndRunDto, ApiCreateThreadDto, ApiCreateThreadMessageDto,
    MessageContent, RunDto, ThreadDto, ThreadEventData, ThreadMessageDto,
};
use futures::{Stream, StreamExt};
use hyper::StatusCode;

use super::router_client::RouterClient;

pub static DEFAULT_LLM_MODEL: &str = "openai:gpt-4o-mini";

#[allow(dead_code)]
pub struct AssistantApiClient {
    client: RouterClient,
}

impl AssistantApiClient {
    #[allow(dead_code)]
    pub async fn new() -> Self {
        let client = RouterClient::from_app("/openai/v1").await;

        Self { client }
    }
    #[allow(dead_code)]
    pub async fn create_thread(&self, request: &ApiCreateThreadDto) -> (ThreadDto, StatusCode) {
        let (dto, status) = self
            .client
            .post::<ApiCreateThreadDto, ThreadDto>("/threads", request)
            .await
            .unwrap();

        (dto.unwrap(), status)
    }

    #[allow(dead_code)]
    pub async fn create_empty_thread(&self) -> (ThreadDto, StatusCode) {
        self.create_thread(&ApiCreateThreadDto::default()).await
    }

    #[allow(dead_code)]
    pub async fn create_thread_with_prompt(&self, prompt: &str) -> ThreadDto {
        let r = self
            .create_thread(&ApiCreateThreadDto {
                messages: vec![ApiCreateThreadMessageDto {
                    role: "user".to_string(),
                    content: vec![MessageContent::text(prompt)],
                    ..ApiCreateThreadMessageDto::default()
                }],
                ..ApiCreateThreadDto::default()
            })
            .await
            .0;

        r
    }

    #[allow(dead_code)]
    pub async fn create_message(
        &self,
        thread_id: &str,
        request: &ApiCreateThreadMessageDto,
    ) -> (ThreadMessageDto, StatusCode) {
        let (dto, status) = self
            .client
            .post::<ApiCreateThreadMessageDto, ThreadMessageDto>(
                &format!("/threads/{}/messages", thread_id),
                request,
            )
            .await
            .unwrap();

        (dto.unwrap(), status)
    }

    #[allow(dead_code)]
    pub async fn create_user_message_from_prompt(
        &self,
        thread_id: &str,
        prompt: &str,
    ) -> (ThreadMessageDto, StatusCode) {
        self.create_message(
            thread_id,
            &ApiCreateThreadMessageDto {
                content: vec![MessageContent::text(prompt)],
                ..ApiCreateThreadMessageDto::user()
            },
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn stream_new_thread_with_prompt(
        &self,
        prompt: &str,
    ) -> impl Stream<Item = Result<(String, ThreadEventData), axum::Error>> {
        let create_thread_run_dto = ApiCreateThreadAndRunDto {
            model: DEFAULT_LLM_MODEL.to_string(),
            thread: ApiCreateThreadDto {
                messages: vec![ApiCreateThreadMessageDto {
                    content: vec![MessageContent::text(prompt)],
                    ..ApiCreateThreadMessageDto::user()
                }],
                ..ApiCreateThreadDto::default()
            },
            stream: Some(true),
            ..ApiCreateThreadAndRunDto::default()
        };

        self.stream_create_thread_and_run(&create_thread_run_dto)
            .await
    }

    #[allow(dead_code)]
    pub async fn stream_new_thread_with_prompt_as_chunks_vec(
        &self,
        prompt: &str,
    ) -> Vec<(String, ThreadEventData)> {
        let stream = self.stream_new_thread_with_prompt(prompt).await;

        Self::stream_to_events_array(Box::pin(stream)).await
    }

    #[allow(dead_code)]
    pub async fn stream_create_thread_and_run(
        &self,
        request: &ApiCreateThreadAndRunDto,
    ) -> impl Stream<Item = Result<(String, ThreadEventData), axum::Error>> {
        let stream = self
            .client
            .post_json_stream::<ApiCreateThreadAndRunDto, ThreadEventData>(
                "/threads/runs",
                request,
            );

        stream
    }

    #[allow(dead_code)]
    pub async fn create_run(
        &self,
        thread_id: &str,
        request: &ApiCreateRunDto,
    ) -> (RunDto, StatusCode) {
        let (dto, status) = self
            .client
            .post::<ApiCreateRunDto, RunDto>(
                format!("/threads/{}/runs", thread_id).as_str(),
                &request,
            )
            .await
            .unwrap();

        (dto.unwrap(), status)
    }

    #[allow(dead_code)]
    pub async fn stream_run(
        &self,
        thread_id: &str,
        request: &ApiCreateRunDto,
    ) -> impl Stream<Item = Result<(String, ThreadEventData), axum::Error>> {
        let stream = self
            .client
            .post_json_stream::<ApiCreateRunDto, ThreadEventData>(
                format!("/threads/{}/runs", thread_id).as_str(),
                &ApiCreateRunDto {
                    model: if request.model.is_empty() {
                        DEFAULT_LLM_MODEL.to_string()
                    } else {
                        request.model.clone()
                    },
                    stream: Some(true),
                    ..request.clone()
                },
            );

        stream
    }

    #[allow(dead_code)]
    pub async fn stream_run_as_chunks_array(
        &self,
        thread_id: &str,
        request: &ApiCreateRunDto,
    ) -> Vec<(String, ThreadEventData)> {
        let stream = self.stream_run(thread_id, request).await;

        Self::stream_to_events_array(Box::pin(stream)).await
    }

    #[allow(dead_code)]
    pub async fn get_run(&self, thread_id: &str, run_id: &str) -> (RunDto, StatusCode) {
        let (dto, status) = self
            .client
            .get::<RunDto>(&format!("/threads/{}/runs/{}", thread_id, run_id))
            .await
            .unwrap();

        (dto.unwrap(), status)
    }

    #[allow(dead_code)]
    pub async fn list_thread_messages(
        &self,
        thread_id: &str,
    ) -> (Vec<ThreadMessageDto>, StatusCode) {
        let (dto, status) = self
            .client
            .get::<Vec<ThreadMessageDto>>(&format!("/threads/{}/messages", thread_id))
            .await
            .unwrap();

        (dto.unwrap(), status)
    }

    #[allow(dead_code)]
    pub async fn list_thread_runs(&self, thread_id: &str) -> (Vec<RunDto>, StatusCode) {
        let (dto, status) = self
            .client
            .get::<Vec<RunDto>>(&format!("/threads/{}/runs", thread_id))
            .await
            .unwrap();

        (dto.unwrap(), status)
    }

    pub async fn stream_to_events_array(
        mut stream: Pin<Box<dyn Stream<Item = Result<(String, ThreadEventData), axum::Error>>>>,
    ) -> Vec<(String, ThreadEventData)> {
        let mut responses = Vec::new();
        while let Some(chunk) = stream.next().await {
            let response = chunk.unwrap();
            responses.push(response);
        }

        responses
    }
}
