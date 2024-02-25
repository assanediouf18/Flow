use std::{env, io};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use clap::{Arg, ArgMatches, Command};
use crate::config::Configuration;
use crate::project::Project;

pub fn get_update_command() -> clap::Command {
    Command::new("update")
        .about("Modify the description, name, path and git repository")
        .arg(Arg::new("name").help("The name of the project"))
        .arg(
            Arg::new("desc")
                .short('d').long("desc")
                .required(false)
                .default_value(None)
                .help("Provide a description to your project in case you want to remember something")
        )
        .arg(
            Arg::new("path")
                .short('p').long("path")
                .required(false)
                .default_value(None)
                .help("Provide the path of your project")
        )
        .arg(
            Arg::new("ide")
                .short('i')
                .long("ide")
                .required(false)
                .default_value(None)
                .help("Set the preferred ide for the project, by default it is vscode")
        )
        .arg(
            Arg::new("vcs")
                .required(false)
                .default_value(None)
                .help("Link the project to a repository")
        )
        .arg(
            Arg::new("new-name")
                .short('n')
                .required(false)
                .default_value(None)
                .help("Defines a new name for the project")
        )
}

pub fn update_project(config: &Configuration, projects: &mut Vec<Project>,sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must name your project");
    if let Some(project) = projects.iter_mut().find(|p| { p.name.to_lowercase() == name.to_lowercase() }) {
        println!("Updating project {}", name);

        let desc = match(sub_matches.get_one::<String>("desc")) {
            None => project.description.clone(),
            Some(d) => Some(d.to_string())
        };

        let path = match(sub_matches.get_one::<String>("path")) {
            None => project.path.clone(),
            Some(p) => p.into()
        };

        let ide = match(sub_matches.get_one::<String>("ide")) {
            None => project.ide.clone(),
            Some(d) => d.to_string()
        };

        let github_url: Option<String> = match(sub_matches.get_one::<String>("vcs")) {
            None => project.github_url.clone(),
            Some(d) => Some(d.to_string())
        };

        let new_name = match(sub_matches.get_one::<String>("new-name")) {
            None => project.name.clone(),
            Some(d) => d.to_string()
        };

        project.name = new_name;
        project.path = path;
        project.ide = ide;
        project.github_url = github_url;
        project.description = desc;

        let mut config_file = File::options()
            .write(true)
            .truncate(true)
            .open(
                config.get_projects_filepath()
            ).expect("Error : can't open project file");

        config_file.write_all(
            serde_json::to_string(&projects)
                .unwrap()
                .as_bytes()
        ).expect("Can't persist new project");

        println!("The project was successfully updated")
    }
    else {
        println!("There is no project called '{name}'")
    }
}