#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub url: Option<String>,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse app environment");

    info!("Loading configs from {}", environment.as_str());

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(&environment_filename),
        ))
        .add_source(config::Environment::default().separator("_"))
        .build()?;

    settings.try_deserialize::<Settings>()
}

// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
    Docker,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
            Environment::Docker => "docker",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "docker" => Ok(Self::Docker),
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
Use either `local`, `docker`, or `production`.",
                other
            )),
        }
    }
}

impl Settings {
    pub fn connection_string(&self) -> String {
        match &self.database.url {
            Some(url) => {
                info!("Pulling db connection string from .env");
                url.to_string()
            }
            None => {
                info!("Pulling db connection string from config file");
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    self.database.username,
                    self.database.password,
                    self.application.host,
                    5432,
                    self.database.database_name
                )
            }
        }
    }

    pub fn port(&self) -> u16 {
        self.application.port
    }
}

#[test]
fn test_parse_int() {
    use config::{Config, Environment};
    use std::path::PathBuf;

    temp_env::with_var("DATABASE_URL", Some("database@databaseurl"), || {
        let environment = Environment::default().separator("_");

        let config = Config::builder()
            .add_source(config::File::from(PathBuf::from("configuration/base.yaml")))
            .add_source(config::File::from(PathBuf::from(
                "configuration/docker.yaml",
            )))
            .add_source(environment)
            .build()
            .unwrap();

        let config: Settings = config.try_deserialize().unwrap();

        assert_eq!(
            config.database.url.unwrap(),
            String::from("database@databaseurl")
        );
    })
}
