use std::path::PathBuf;

#[derive(Debug)]
pub struct Project {
    name: String,
    path: PathBuf,
    ide: String,
    github_url: Option<String>,
    description: Option<String>
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