use serde::Deserialize;
use std::path::PathBuf;

pub fn read_config<T: for<'a> Deserialize<'a>>(config_file_path: PathBuf) -> T {
    dotenvy::from_path(config_file_path).ok();

    config::Config::builder()
        .add_source(
            config::Environment::default(), //.separator("__")  // default is ".". I guess that's fine?
        )
        .build()
        .expect("Failed to build config")
        .try_deserialize::<T>()
        .expect("Invalid or missing config values")
}
