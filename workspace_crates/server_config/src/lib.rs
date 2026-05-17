use std::time::Duration;


#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerConfig {
    pub rust_log: String,
    pub rust_log_directory_path: String,

    pub port: u64,
    pub ip: String,

    pub server_global_request_token_bucket_capacity: f64,
    pub server_global_request_token_bucket_refill_rate_per_millis: f64,
    pub per_ip_token_bucket_capacity: f64,
    pub per_ip_token_bucket_refill_rate_per_millis: f64,

    pub keep_alive_enabled: bool,
    //    #[serde(with = "humantime_serde")] automatically converts values like 5000ms or 60s or 1h into Duration
    #[serde(with = "humantime_serde")]
    pub keep_alive_idle_timeout: Duration,
    pub keep_alive_max_requests_per_connection: u64,

    #[serde(with = "humantime_serde")]
    pub common_stream_timeout: Duration,

    pub max_total_headers_size_bytes: u64,
    pub max_headers_count: u64,
    pub max_header_name_length: u64,
    pub max_header_value_length: u64,
    pub max_total_body_size_bytes: u64,
    pub max_request_path_length: u64,
    pub max_cookies_count: u64,

    #[serde(with = "humantime_serde")]
    pub server_time_credits_refill_rate_per_millis: Duration,
    #[serde(with = "humantime_serde")]
    pub server_time_credits_capacity: Duration,
    #[serde(with = "humantime_serde")]
    pub per_ip_time_credits_refill_rate_per_millis: Duration,
    #[serde(with = "humantime_serde")]
    pub per_ip_time_credits_capacity: Duration,
}
