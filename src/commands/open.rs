
use clap::{Arg, ArgMatches, Command};
use crate::config::Configuration;
use crate::project::Project;
use std::io;
use std::path::PathBuf;

pub fn get_open_subcommand() -> clap::Command {
    Command::new("open")
        .about("Open the project. If the path is not found, tries a git clone")
        .arg(Arg::new("name").help("The name of the project"))
}

pub fn open_project(config: &Configuration, projects: &mut Vec<Project>,sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must enter a project name");
    if let Some(project) = projects.iter_mut().find(|p| { p.name.to_lowercase() == name.to_lowercase() }) {
        match config.get_editor(&project.ide) {
            Some(editor) => {
                if let Some(path) = get_project_or_clone(project) {
                    let mut editor_command = std::process::Command::new("cmd");
                    println!("> Opening the project");
                    editor_command
                        .arg("/C")
                        .arg(&editor.command)
                    ;

                    editor_command
                        .arg(&path)
                        .output()
                        .expect("Can't open project")
                        .stdout;

                    config.update_config_file(&projects);
                }
                else {
                    println!("No correct path found. Operation aborted")
                }
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

fn get_project_or_clone(project: &mut Project) -> Option<PathBuf> {
    if !project.path.exists() {
        println!("Flow can't find the project at {}, would you like to clone it ? (Y/n)", project.path.to_str().expect("Incorrect Path"));
        let mut should_clone = String::new();
        io::stdin().read_line(&mut should_clone).expect("Invalid input");
        let should_clone = should_clone.trim();
        if should_clone.to_lowercase() == "y" {
            return clone_repo(project);
        };
        return None;
    }
    Some(project.path.clone())
}

fn clone_repo(project: &mut Project) -> Option<PathBuf> {
    println!("Please enter the path to clone the project ({}) : ",
             project.path
                 .to_str().expect("Can't convert current path to string"));
    let mut path;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    let input = input.trim();
    if input.is_empty() {
        path = PathBuf::from(&project.path);
    } else {
        path = PathBuf::from(input);
    }
    path.push(PathBuf::from(&project.name));

    if let Some(vcs) = project.github_url.clone() {
        let mut git_command = std::process::Command::new("git");

        println!("> git clone");
        let result = git_command
            .args(["clone"])
            .arg(&vcs)
            .arg(&path)
            .status()
            .expect("Can't clone project")
        ;

        if !result.success() {
            println!("An error occured while cloning the project");
            return None;
        }

        println!("Do you want to update the path of your project ? (Y/n)");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Invalid input");
        let answer = answer.trim();
        if answer.to_lowercase() == "y" {
            project.path = path.clone();
        };

        return Some(path);
    }
    println!("You don't have any repository linked to that project.");
    None
}