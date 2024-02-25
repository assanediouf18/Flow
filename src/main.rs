mod project;
mod config;
mod editor;
mod commands;

use commands::*;
use crate::commands::add::add_project;
use crate::config::get_configuration;
use crate::project::load_projects;

fn main() {
    let config = get_configuration().expect("Can't read configurations");
    let mut projects = load_projects(&config.get_projects_filepath()).expect("Can't open projects");

    let matches = create_main_command();

    match matches.get_matches().subcommand() {
        Some(("add", sub_matches)) => {
            add_project(&config, &mut projects,sub_matches);
        },
        _ => println!("Let's start !")//unreachable!()
    }
}
