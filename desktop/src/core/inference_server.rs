use std::{error::Error, sync::Arc};

use async_stream::try_stream;
use openai_server_api::{self, serve, ServeParameters, StreamData};

#[derive(Clone)]
pub struct InferenceServer {
    port: u16,
}

impl InferenceServer {
    pub fn new(port: u16) -> Self {
        InferenceServer { port }
    }

    pub async fn serve(&self) {
        serve(ServeParameters {
            port: self.port,
            invoke_fn: Arc::new(|_messages| Ok("It is something".to_string())),
            stream_fn: Arc::new(|_messages| {
                let stream = try_stream! {
                    yield StreamData::new(
                        serde_json::json!({}),
                        "Hello world".to_string(),
                        );
                };

                Box::pin(stream)
            }),
            use_trace: false,
            ..ServeParameters::default()
        })
        .await
    }

    pub fn is_core_server_up(&self) -> Result<bool, Box<dyn Error>> {
        let response = ureq::get(&format!("http://localhost:{}", self.port)).call()?;
        let status = response.status();

        return Ok(status == 200);
    }
}
