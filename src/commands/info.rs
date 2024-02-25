use clap::{Arg, ArgMatches, Command};
use crate::project::Project;

pub fn get_info_subcommand() -> clap::Command {
    Command::new("info")
        .about("Display information about the project")
        .arg(
            Arg::new("name")
                .help("The name of the project")
        )
}

pub fn project_info(projects: &Vec<Project>, sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must enter a project name");
    match projects.iter().find(|p| { p.name.to_lowercase() == name.to_lowercase() }) {
        Some(project) => {
            println!("Name: {}", project.name);
            if let Some(path)= &project.path.to_str() {
                println!("Path: {}", path);
            }
            if let Some(desc)= &project.description {
                println!("Description: {}", desc);
            }
            if let Some(git)= &project.github_url {
                println!("VCS: {}", git);
            }
            println!("IDE: {}", project.ide);
        },
        None => {
            println!("This project does not exist");
            println!("To create it run : flow add {}", name)
        }
    };
}