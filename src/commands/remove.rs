use std::fs::File;
use std::io;
use std::io::Write;
use clap::{Arg, ArgMatches, Command};
use crate::config::Configuration;
use crate::project::Project;

pub fn get_remove_subcommand() -> clap::Command {
    Command::new("remove")
        .about("Remove a project")
        .arg(
            Arg::new("name").help("The name of the project")
        )
}

pub fn remove_project(config: &Configuration, projects: &mut Vec<Project>, sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must specify a project name");

    if let Some((index, _)) = projects.iter()
        .enumerate()
        .find(|(_i, p)| {
            p.name.to_lowercase() == name.to_lowercase()
        }) {

        let mut answer = String::new();
        println!("Are you sure you want to remove the project '{name}' ? (Y/N)");
        io::stdin().read_line(&mut answer).expect("Can't read user's input");
        let answer = answer.trim();

        if answer.to_lowercase() != "y" {
            println!("Operation aborted");
            return;
        }

        projects.remove(index);
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
        ).expect("Can't delete definitively the project");

        println!("The project was successfully removed")
    }
    else {
        println!("The project {name} does not exist.")
    }
}