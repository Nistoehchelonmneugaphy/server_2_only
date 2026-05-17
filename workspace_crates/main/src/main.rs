use std::path::PathBuf;

use config_reader::ServerConfig;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let _server_config = config_reader::read_config::<ServerConfig>(PathBuf::from(".env"));
    dotenvy::dotenv().ok();

    let handler: server::run::HandlerFn =
        std::sync::Arc::new(move |request_data, server_config| {
            Box::pin(async move {
		http_stuff::http_response::ok_html(format!("<b>Hello world</b><br>{:#?}, {:#?}", request_data.path, server_config.ip))
            })
        });
    server::run(handler).await
}
