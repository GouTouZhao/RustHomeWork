use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ALLConfig {
    pub discovery: Discovery,
    pub mysql: MySQL,
    pub redis: Redis,
    pub oss: OSS,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Discovery {
    pub zookeeper: Option<Zookeeper>,
    pub kubernetes: Option<Kubernetes>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Zookeeper {
    pub address: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Kubernetes {
    pub namespace: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MySQL {
    pub address: Vec<String>,
    pub username: Vec<String>,
    pub password: Vec<String>,
    pub database: Vec<String>,
    pub max_open_conn: Vec<i32>,
    pub max_idle_conn: Vec<i32>,
    pub max_life_time: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Redis {
    pub address: Vec<String>,
    pub password: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OSS {
    pub endpoint: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub bucket_name: String,
}

pub fn load_config() -> Result<ALLConfig, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("../../config/discovery.yaml").required(false))
        .add_source(config::File::with_name("../../config/mysql.yaml").required(false))
        .add_source(config::File::with_name("../../config/redis.yaml").required(false))
        .add_source(config::File::with_name("../../config/oss.yaml").required(false))
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()?;

    settings.try_deserialize()
}
