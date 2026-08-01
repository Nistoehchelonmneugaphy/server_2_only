pub struct HandlerOnly;
pub struct WithTasks;

use server_config::ServerConfig;
use std::future::Future;
use std::pin::Pin;
use tokio_util::sync::CancellationToken;

use crate::{HandlerFn, server_run_inner};

pub type BackgroundTasks = Vec<Pin<Box<dyn Future<Output = ()> + Send>>>;

pub struct Server<State> {
    config: ServerConfig,
    version: String,
    handler: HandlerFn,
    background_tasks: Option<(BackgroundTasks, CancellationToken)>,
    _state: std::marker::PhantomData<State>,
}

impl Server<HandlerOnly> {
    pub fn new(config: ServerConfig, version: String, handler: HandlerFn) -> Self {
        Self {
            config,
            version,
            handler,
            background_tasks: None,
            _state: std::marker::PhantomData,
        }
    }

    /// Run the server **without** background tasks.
    pub async fn run_handler(self) -> std::io::Result<()> {
        // background is guaranteed to be None
        server_run_inner(self.config, self.version, self.handler, None, None).await
    }

    /// Add background tasks **and** a cancellation token.
    /// Transitions to `Server<WithTasks>`.
    pub fn add_background_tasks(
        self,
        tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send>>>,
        cancellation_token: CancellationToken,
    ) -> Server<WithTasks> {
        Server {
            config: self.config,
            version: self.version,
            handler: self.handler,
            background_tasks: Some((tasks, cancellation_token)),
            _state: std::marker::PhantomData,
        }
    }
}

impl Server<WithTasks> {
    /// Run the server **with** the background tasks and their cancellation token.
    pub async fn run_handler_and_background_tasks(self) -> std::io::Result<()> {
        let (tasks, cancellation_token) =
            self.background_tasks.expect("background guaranteed Some");
        server_run_inner(
            self.config,
            self.version,
            self.handler,
            Some(tasks),
            Some(cancellation_token),
        )
        .await
    }
}
