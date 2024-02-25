use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::editor::Editor;

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

    pub fn get_projects_filepath(&self) -> String {
        match File::open(&self.project_file) {
            Ok(_) => {},
            Err(_) => {
                let mut file = File::create(&self.project_file).expect("Error : Can't create project file");
                file.write_all("[]".as_ref()).expect("Can't initialize project file");
            }
        };
        self.project_file.clone()
    }
}