mod project;
mod command;
mod config;
mod editor;

use command::*;
use crate::config::get_configuration;
use crate::project::load_projects;

fn main() {
    let config = get_configuration().expect("Can't read configurations");
    let mut projects = load_projects(&config.get_projects_filepath()).expect("Can't open projects");
    let matches = create_main_command(&config, &mut projects);

    match matches.get_matches().subcommand() {
        Some(("add", sub_matches)) => {
            add_project(sub_matches);
        },
        _ => println!("Let's start !")//unreachable!()
    }
}
