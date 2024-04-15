use std::path::Path;

#[derive(serde::Deserialize)]
pub struct Config {
    pub version: String,
    pub server_name: String,
    pub num_worlds: usize,
    pub num_channels: u16,
    pub base_port: u16,
    pub client_version: usize,
    pub bind_ip: String,
    pub tuf_repo_port: u16,
    pub external_ip: Option<String>,
}

pub fn get_configuration(data_dir: impl AsRef<Path>) -> Result<Config, config::ConfigError> {
    let configuration_directory = data_dir.as_ref().to_path_buf().join("config");
    let environment: Environment = get_environment();
    let environment_filename = format!("{}.toml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.toml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Config>()
}

pub fn get_environment() -> Environment {
    std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.")
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
    Staging,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            other => anyhow::bail!(
                "{other} is not a supported environment. Use either `local`, `production`, or `staging`.",
            ),
        }
    }
}
