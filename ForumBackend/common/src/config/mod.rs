use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ALLConfig {
    pub discovery: Discovery,
    pub mysql: MySQL,
    pub redis: Redis,
    pub oss: OSS,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Discovery {
    pub enable: String,
    pub kubenetes: Option<Kubernetes>,
    #[serde(rename = "rpcService")]
    pub rpc_service: Option<RpcService>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Kubernetes {
    pub namespace: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RpcService {
    pub api: String,
    pub user: String,
    #[serde(rename = "static")]
    pub static_svc: Option<String>,
    pub bmanager: String,
    pub admin: String,
    pub image: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MySQL {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub dbname: i32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OSS {
    pub endpoint: String,
    pub internal_endpoint: Option<String>,
    pub bucket: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub role_arn: String,
}

pub fn load_config() -> Result<ALLConfig, config::ConfigError> {
    // 优先使用环境变量传入的配置路径，如果在本地测试则默认使用 ../../config
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "../../config".to_string());

    // 针对每个配置文件单独加载并解析，避免 config 库合并根节点导致字段丢失
    let discovery_cfg = config::Config::builder()
        .add_source(config::File::with_name(&format!("{}/discovery.yaml", config_path)).required(false))
        .build()?;
    let discovery: Discovery = discovery_cfg.try_deserialize().unwrap_or_default();

    let mysql_cfg = config::Config::builder()
        .add_source(config::File::with_name(&format!("{}/mysql.yaml", config_path)).required(false))
        .build()?;
    let mysql: MySQL = mysql_cfg.try_deserialize().unwrap_or_default();

    let redis_cfg = config::Config::builder()
        .add_source(config::File::with_name(&format!("{}/redis.yaml", config_path)).required(false))
        .build()?;
    let redis: Redis = redis_cfg.try_deserialize().unwrap_or_default();

    let oss_cfg = config::Config::builder()
        .add_source(config::File::with_name(&format!("{}/oss.yaml", config_path)).required(false))
        // 自动读取环境变量中以 OSS_ 开头的配置覆盖本地的 placeholder
        .add_source(config::Environment::with_prefix("OSS").separator("_"))
        .build()?;
    let oss: OSS = oss_cfg.try_deserialize().unwrap_or_default();

    Ok(ALLConfig {
        discovery,
        mysql,
        redis,
        oss,
    })
}
