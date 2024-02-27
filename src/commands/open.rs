use std::path::PathBuf;
use clap::{Arg, ArgMatches, Command};
use crate::config::Configuration;
use crate::project::Project;

pub fn get_open_subcommand() -> clap::Command {
    Command::new("open")
        .about("Open the project. If the path is not found, tries a git clone")
        .arg(Arg::new("name").help("The name of the project"))
}

pub fn open_project(config: &Configuration, projects: &mut Vec<Project>,sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must enter a project name");
    if let Some(project) = projects.iter().find(|p| { p.name.to_lowercase() == name.to_lowercase() }) {
        match config.get_editor(&project.ide) {
            Some(editor) => {
                let mut editor_command = std::process::Command::new("cmd");
                editor_command
                    .arg("/C")
                    .arg("code")
                ;

                editor_command
                    .arg(&project.path)
                    .output()
                    .expect("Can't open project")
                    .stdout;
            },
            None => {
                println!("The editor '{}' is unknown. Operation aborted.", project.ide)
            }
        }
    }
    else {
        println!("The project '{name}' does not exist");
    }
}