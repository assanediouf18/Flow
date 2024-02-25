use clap::Command;
use crate::project::Project;

pub fn get_list_subcommand() -> Command {
    Command::new("list")
        .about("List all the projects")
}

pub fn list_projects(projects: &Vec<Project>) {
    println!("Here is a list of your projects");
    for project in projects.iter() {
        println!(" - {}", project.name);
    }
}