use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::editor::Editor;
use crate::project::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration
{
    project_file: String,
    editors: Vec<Editor>,
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let configuration = config::Config::builder()
        .add_source(
            config::File::new("configuration.json", config::FileFormat::Json)
        ).build()?;
    configuration.try_deserialize::<Configuration>()
}

impl Configuration {
    pub fn new() -> Self {
        Self{
            editors: Vec::new(),
            project_file: String::new(),
        }
    }

    pub fn get_projects_filepath(&self) -> String {
        match File::open(&self.project_file) {
            Ok(file) => {},
            Err(_) => {
                let mut file = File::create(&self.project_file).expect("Error : can't save the project");
                file.write_all("[]".as_ref()).expect("Can't initialize project file");
            }
        };
        self.project_file.clone()
    }
}