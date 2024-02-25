use std::fmt::Error;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    path: PathBuf,
    ide: String,
    github_url: Option<String>,
    description: Option<String>
}

pub fn load_projects(filepath: &str) -> serde_json::Result<Vec<Project>> {
    let projects = fs::read_to_string(filepath).expect("Can't open project file");    serde_json::from_str(&projects)
}

impl Project {
    pub fn new(
        name: String, path: PathBuf, ide: String, github_url: Option<String>, description: Option<String>) -> Self {
        Self {
            name,
            path,
            ide,
            github_url,
            description,
        }
    }
}