use std::{env, io};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use clap::{Arg, ArgMatches, Command};
use crate::config::Configuration;
use crate::project::Project;

pub fn get_add_subcommand() -> Command {
    Command::new("add")
        .about("Add a new project to the manager. If you don't specify a preferred ide, vscode is set by default")
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
}

pub fn add_project(config: &Configuration, projects: &mut Vec<Project>,sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must name your project");
    if let Some(project) = projects.iter().find(|p| { p.name.to_lowercase() == name.to_lowercase() }) {
        println!("A project name '{}' already exists", project.name);
        return;
    }

    println!("Creating project {}", name);

    let desc = match sub_matches.get_one::<String>("desc") {
        None => None,
        Some(value) => Some(value)
    };

    let path = match sub_matches.get_one::<String>("path") {
        None => {
            let mut input = String::new();
            println!("Please enter the path of the project ({}) : ",
                     env::current_dir()
                         .expect("Can't retrieve current path")
                         .to_str().expect("Can't convert current path to string"));
            io::stdin().read_line(&mut input).expect("Invalid input");
            let input = input.trim();
            if input.is_empty() {
                PathBuf::from(env::current_dir()
                    .expect("Can't retrieve current path")
                    .to_str().expect("Can't convert current path to string"))
            } else {
                PathBuf::from(input)
            }
        }
        Some(value) => PathBuf::from(value)
    };

    let ide = match sub_matches.get_one::<String>("ide") {
        None => {
            let mut input = String::new();
            println!("Please enter your preferred ide for this project (vscode) : ");
            io::stdin().read_line(&mut input).expect("Invalid input");
            let input = input.trim();
            if input.is_empty() {
                "vscode".to_string()
            } else {
                input.to_string()
            }
        }
        Some(value) => value.to_string()
    };

    println!("Link your project to a git repo : (type nothing if you don't want to)");
    let mut repo = String::new();
    let github_url = match io::stdin().read_line(&mut repo) {
        Ok(_) => {
            let repo = repo.trim();
            if repo.is_empty() {
                None
            } else {
                Some(repo.to_string())
            }
        },
        Err(_) => None
    };

    let project = Project::new(name.to_string(), path, ide, github_url, desc.cloned());
    println!("{:?}", project);

    projects.push(project);

    config.update_config_file(&projects);

    println!("The project was successfully added")
}