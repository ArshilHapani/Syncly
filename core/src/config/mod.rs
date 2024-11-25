use serde_json::{json, Value};

pub const CONFIG_FILE_NAME: &str = ".syncly.json";
pub const SNAPSHOT_FILE_NAME: &str = ".syncly.snapshot.json";

pub fn get_default_config() -> Value {
    json!({
        "exclude":[".git","node_modules"],
        "snapshot_ctn": 0,
    })
}
