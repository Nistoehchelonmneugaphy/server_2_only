use std::time::Duration;
mod defaults;
use defaults::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerConfig {
    pub rust_log_directory_path: String,
    pub port: u64,
    pub ip: String,

    #[serde(default = "default_rust_log")]
    pub rust_log: String,

    #[serde(default = "default_server_global_request_token_bucket_capacity")]
    pub server_global_request_token_bucket_capacity: f64,
    #[serde(default = "default_server_global_request_token_bucket_refill_rate_per_millis")]
    pub server_global_request_token_bucket_refill_rate_per_millis: f64,
    #[serde(
        default = "default_server_time_credits_refill_rate_per_millis",
        with = "humantime_serde"
    )]
    pub server_time_credits_refill_rate_per_millis: Duration,
    #[serde(
        default = "default_server_time_credits_capacity",
        with = "humantime_serde"
    )]
    pub server_time_credits_capacity: Duration,

    #[serde(
        default = "default_per_ip_time_credits_refill_rate_per_millis",
        with = "humantime_serde"
    )]
    pub per_ip_time_credits_refill_rate_per_millis: Duration,
    #[serde(
        default = "default_per_ip_time_credits_capacity",
        with = "humantime_serde"
    )]
    pub per_ip_time_credits_capacity: Duration,
    #[serde(default = "default_per_ip_token_bucket_capacity")]
    pub per_ip_token_bucket_capacity: f64,
    #[serde(default = "default_per_ip_token_bucket_refill_rate_per_millis")]
    pub per_ip_token_bucket_refill_rate_per_millis: f64,

    #[serde(default = "default_keep_alive_enabled")]
    pub keep_alive_enabled: bool,
    #[serde(default = "default_keep_alive_max_requests_per_connection")]
    pub keep_alive_max_requests_per_connection: u64,
    //    #[serde(with = "humantime_serde")] automatically converts values like 5000ms or 60s or 1h into Duration
    #[serde(default = "default_keep_alive_idle_timeout", with = "humantime_serde")]
    pub keep_alive_idle_timeout: Duration,

    #[serde(default = "default_common_stream_timeout", with = "humantime_serde")]
    pub common_stream_timeout: Duration,

    #[serde(default = "default_max_total_headers_size_bytes")]
    pub max_total_headers_size_bytes: u64,
    #[serde(default = "default_max_headers_count")]
    pub max_headers_count: u64,
    #[serde(default = "default_max_header_name_length")]
    pub max_header_name_length: u64,
    #[serde(default = "default_max_header_value_length")]
    pub max_header_value_length: u64,
    #[serde(default = "default_max_total_body_size_bytes")]
    pub max_total_body_size_bytes: u64,
    #[serde(default = "default_max_request_path_length")]
    pub max_request_path_length: u64,
    #[serde(default = "default_max_cookies_count")]
    pub max_cookies_count: u64,
}

impl ServerConfig {
    pub fn new(ip: String, port: u64, rust_log_directory_path: String) -> Self {
        Self {
            rust_log_directory_path,
            port,
            ip,
            rust_log: default_rust_log(),
            server_global_request_token_bucket_capacity:
                default_server_global_request_token_bucket_capacity(),
            server_global_request_token_bucket_refill_rate_per_millis:
                default_server_global_request_token_bucket_refill_rate_per_millis(),
            server_time_credits_refill_rate_per_millis:
                default_server_time_credits_refill_rate_per_millis(),
            server_time_credits_capacity: default_server_time_credits_capacity(),
            per_ip_time_credits_refill_rate_per_millis:
                default_per_ip_time_credits_refill_rate_per_millis(),
            per_ip_time_credits_capacity: default_per_ip_time_credits_capacity(),
            per_ip_token_bucket_capacity: default_per_ip_token_bucket_capacity(),
            per_ip_token_bucket_refill_rate_per_millis:
                default_per_ip_token_bucket_refill_rate_per_millis(),
            keep_alive_enabled: default_keep_alive_enabled(),
            keep_alive_max_requests_per_connection: default_keep_alive_max_requests_per_connection(
            ),
            keep_alive_idle_timeout: default_keep_alive_idle_timeout(),
            common_stream_timeout: default_common_stream_timeout(),
            max_total_headers_size_bytes: default_max_total_headers_size_bytes(),
            max_headers_count: default_max_headers_count(),
            max_header_name_length: default_max_header_name_length(),
            max_header_value_length: default_max_header_value_length(),
            max_total_body_size_bytes: default_max_total_body_size_bytes(),
            max_request_path_length: default_max_request_path_length(),
            max_cookies_count: default_max_cookies_count(),
        }
    }
}
