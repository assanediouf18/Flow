mod project;
mod config;
mod editor;
mod commands;
mod timer;
mod flow_timer;

use commands::*;
use crate::commands::add::add_project;
use crate::commands::info::project_info;
use crate::commands::list::list_projects;
use crate::commands::open::open_project;
use crate::commands::remove::remove_project;
use crate::commands::update::update_project;
use crate::config::get_configuration;
use crate::flow_timer::FlowTimer;
use crate::project::{load_projects};
use crate::timer::{start_timer_command, stop_timer_command};

fn main() {
    let config = get_configuration().expect("Can't read configurations");
    let mut projects = load_projects(&config.get_projects_filepath()).expect("Can't open projects");
    let mut timers: Vec<FlowTimer> = Vec::new();

    let matches = create_main_command();

    match matches.get_matches().subcommand() {
        Some(("add", sub_matches)) => {
            add_project(&config, &mut projects, sub_matches);
        },
        Some(("update", sub_matches)) => {
            update_project(&config, &mut projects, sub_matches);
        },
        Some(("remove", sub_matches)) => {
            remove_project(&config, &mut projects, sub_matches);
        },
        Some(("open", sub_matches)) => {
            open_project(&config, &mut projects, &mut timers, sub_matches);
        },
        Some(("list", _)) => {
            list_projects(&projects);
        },
        Some(("info", sub_matches)) => {
            project_info(&projects, sub_matches);
        },
        Some(("start-timer", sub_matches)) => {
            start_timer_command(&mut projects, &mut timers, sub_matches);
        },
        Some(("stop-timer", sub_matches)) => {
            stop_timer_command(&config, &mut projects, &mut timers, sub_matches);
        },
        _ => println!("Welcome in Flow ! Add a project and start working fast")
    }
}
