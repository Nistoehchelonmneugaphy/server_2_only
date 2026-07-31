use std::time::Duration;

pub fn default_rust_log() -> String {
    "debug".to_string()
}
pub fn default_server_global_request_token_bucket_capacity() -> f64 {
    1000.0
}
pub fn default_server_global_request_token_bucket_refill_rate_per_millis() -> f64 {
    1000.0
}

pub fn default_server_time_credits_capacity() -> Duration {
    Duration::from_secs(50)
}
pub fn default_server_time_credits_refill_rate_per_millis() -> Duration {
    Duration::from_millis(100)
}

pub fn default_per_ip_token_bucket_capacity() -> f64 {
    100.0
}
pub fn default_per_ip_token_bucket_refill_rate_per_millis() -> f64 {
    0.100
}

pub fn default_per_ip_time_credits_refill_rate_per_millis() -> Duration {
    Duration::from_millis(10)
}
pub fn default_per_ip_time_credits_capacity() -> Duration {
    Duration::from_secs(5)
}

pub fn default_keep_alive_enabled() -> bool {
    true
}

pub fn default_keep_alive_max_requests_per_connection() -> u64 {
    10
}

pub fn default_keep_alive_idle_timeout() -> Duration {
    Duration::from_millis(5000)
}

pub fn default_common_stream_timeout() -> Duration {
    Duration::from_millis(2000)
}

pub fn default_max_total_headers_size_bytes() -> u64 {
    8192
}
pub fn default_max_headers_count() -> u64 {
    50
}
pub fn default_max_header_name_length() -> u64 {
    100
}

pub fn default_max_header_value_length() -> u64 {
    2048
}
pub fn default_max_total_body_size_bytes() -> u64 {
    10_485_760
}
pub fn default_max_request_path_length() -> u64 {
    150
}
pub fn default_max_cookies_count() -> u64 {
    50
}
