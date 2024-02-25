use std::{env, io};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use clap::{Arg, ArgMatches, Command};
use crate::config::Configuration;
use crate::project::Project;

pub fn create_main_command(config: &Configuration, projects: &mut Vec<Project>) -> Command {
    Command::new("flow")
        .author("Assane Diouf")
        .about("Little cli to manage your projects")
        .subcommand(get_list_subcommand())
        .subcommand(Command::new("open").about("Open the project. If the path is not found, tries a git clone"))
        .subcommand(get_add_subcommand())
        .subcommand(Command::new("remove").about("Remove a project"))
        .subcommand(Command::new("update").about("Modify the description, name, path and git repository"))
        .subcommand(Command::new("info").about("Display information about the project"))
}

fn get_list_subcommand() -> Command {
    Command::new("list")
        .about("Lists all the projects")
}

pub fn get_add_subcommand() -> Command {
    Command::new("add")
        .about("Add a new project to the manager. If you don't specify a preferred ide, vscode is set by default")
        .arg(Arg::new("name").help("Specifies the name of the project"))
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

const CONFIG_FILEPATH: &'static str = "config";

pub fn add_project(sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must name your project");
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
            if input == "\r\n" {
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
            if (input.is_empty() || input == "\r\n") {
                "vscode".to_string()
            } else {
                input
            }
        }
        Some(value) => value.to_string()
    };

    println!("Link your project to a git repo : (type nothing if you don't want to)");
    let mut repo = String::new();
    let github_url = match io::stdin().read_line(&mut repo) {
        Ok(_) => {
            if repo == "\r\n" {
                None
            } else {
                Some(repo)
            }
        },
        Err(_) => None
    };

    let project = Project::new(name.to_string(), path, ide, github_url, desc.cloned());
    println!("{:?}", project);
    let mut config_file = get_config_file();
    config_file.write_all(serde_json::to_string(&project).unwrap().as_bytes()).expect("Can't persist new configuration");

}

fn get_config_file() -> File {
    match File::open(CONFIG_FILEPATH) {
        Ok(file) => file,
        Err(_) => {
            let mut file = File::create("config.test").expect("Error : can't save the project");
            file
        }
    }
}