use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use crate::environment::Environment;
use crate::error;
use std::error::Error as StdError;
use {
    config::{
        builder::DefaultState, Config as ExtConfig, ConfigError, Environment as ConfigEnvironment,
        File,
    },
    std::{fs::OpenOptions, io::Write},
};

lazy_static! {
    static ref DEFAULT_FOLDER: PathBuf = PathBuf::from(".config");
}

#[async_trait::async_trait]
pub trait Config: Serialize + for<'de> Deserialize<'de> + Default + Clone + Debug {
    // fn application_name(&self) -> String;

    fn enable(&self) -> bool {
        false
    }

    fn to_yaml(&self) -> error::Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    fn print(&self) -> error::Result<()> {
        println!("{}", self.to_yaml()?);
        Ok(())
    }

    fn file_names(env: &Environment, app_name: &str, path: Option<&Path>) -> [PathBuf; 2] {
        let path = path.unwrap_or(DEFAULT_FOLDER.as_path().into());
        [
            path.join(format!("{env}-{app_name}.local.yaml")),
            path.join(format!("{env}-{app_name}.yaml")),
        ]
    }

    fn generate(&self, env: &Environment, app_name: &str) -> error::Result<()> {
        // let files = Self::file_names(env, &self.application_name(), None);
        let files = Self::file_names(env, app_name, None);
        let file_name = files[0].clone();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true) // <--------- this
            .create(true)
            .open(&file_name)?;
        file.write_all(self.to_yaml()?.as_bytes())?;
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait ConfigLoader {
    type Config: Config;
    type Error: StdError + Send + Sync;

    fn from_folder(
        env: &Environment,
        app_name: &str,
        path: Option<&Path>,
    ) -> error::Result<Self::Config> {
        let files = Self::Config::file_names(env, app_name, path);

        let config_builder = Self::parse(app_name, files)?;
        let config = Self::raw_from_folder(env, app_name, path)?;
  
        // You can deserialize (and thus freeze) the entire configuration as
        Ok(config.try_deserialize()?)
    }

    fn raw_from_folder(
        env: &Environment,
        app_name: &str,
        path: Option<&Path>,
    ) -> error::Result<ExtConfig> {
        let files = Self::Config::file_names(env, app_name, path);

        let config_builder = Self::parse(app_name, files)?;
        let config = config_builder
            .set_override("application_name", app_name.to_string())?
            .build()?;
        Ok(config)
    }

    fn from_key(
        key: &str,
        env: &Environment,
        app_name: &str,   
    ) -> error::Result<Self::Config> {
        let raw_config = Self::raw_from_folder(env, app_name, None)?;
        let config = raw_config.get::<Self::Config>(key)?;
        Ok(config)
    }

    fn parse(
        app_name: &str,
        files: [PathBuf; 2],
    ) -> error::Result<config::ConfigBuilder<DefaultState>> {
        let config = ExtConfig::builder();

        // Load Defaults
        let mut config = config.add_source(File::from_str(
            Self::Config::default()
                .to_yaml()
                .map_err(|err| ConfigError::Foreign(Box::new(err)))?
                .as_str(),
            config::FileFormat::Yaml,
        ));

        for file in files.iter() {
            if file.exists() {
                // Merge Config File from Default Location
                config = config.add_source(File::with_name(file.to_str().unwrap()).required(false));
            }
        }

        // Merge Environment Variable Overrides
        let config = config.add_source(
            ConfigEnvironment::with_prefix(&app_name.to_uppercase())
                .separator("_")
                // std::env::set_var("APP_LIST", "Hello World");
                // will be parsed as a list of strings
                .list_separator(" "),
        );

        Ok(config)
    }

    fn file_name(env: &Environment, app_name: &str, path: Option<&Path>) -> [PathBuf; 2] {
        Self::Config::file_names(env, app_name, path)
    }
}
