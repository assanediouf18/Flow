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

    pub fn get_editor(&self, editor_name: &str) -> Option<&Editor> {
        self.editors.iter().find(|e| { e.name.to_lowercase() == editor_name.to_lowercase() })
    }

    pub fn update_config_file(&self, projects: &&mut Vec<Project>) {
        let mut config_file = File::options()
            .write(true)
            .truncate(true)
            .open(
                self.get_projects_filepath()
            ).expect("Error : can't open project file");

        config_file.write_all(
            serde_json::to_string(&projects)
                .unwrap()
                .as_bytes()
        ).expect("Can't persist new project");
    }
}