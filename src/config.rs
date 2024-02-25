use serde::{Deserialize, Serialize};
use crate::editor::Editor;
use crate::project::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration
{
    project_file: String,
    editors: Vec<Editor>,
}

impl Configuration {
    pub fn new() -> Self {
        Self{
            editors: Vec::new(),
            project_file: String::new(),
        }
    }
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let configuration = config::Config::builder()
        .add_source(
            config::File::new("configuration.json", config::FileFormat::Json)
        ).build()?;
    configuration.try_deserialize::<Configuration>()
}