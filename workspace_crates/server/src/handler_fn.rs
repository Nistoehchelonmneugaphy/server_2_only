use std::{pin::Pin, sync::Arc};

use http_stuff::{http_request_parsing::HttpRequestData, http_response::HttpResponseData};
use server_config::ServerConfig;

pub type HandlerFnInner = Arc<
    dyn Fn(HttpRequestData, ServerConfig) -> Pin<Box<dyn Future<Output = HttpResponseData> + Send>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct HandlerFn {
    pub inner: HandlerFnInner,
}

impl HandlerFn {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(HttpRequestData, ServerConfig) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HttpResponseData> + Send + 'static,
    {
        Self {
            inner: Arc::new(move |request_data, server_config| Box::pin(f(request_data, server_config))),
        }
    }

    pub fn call(
        &self,
        request_data: HttpRequestData,
        server_config: ServerConfig,
    ) -> Pin<Box<dyn Future<Output = HttpResponseData> + Send>> {
        (self.inner)(request_data, server_config)
    }
}
