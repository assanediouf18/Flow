mod project;
mod command;
mod config;
mod editor;

use command::*;
use crate::config::get_configuration;

fn main() {
    let config = get_configuration().expect("Can't read configurations");
    let matches = create_main_command();

    match matches.get_matches().subcommand() {
        Some(("add", sub_matches)) => {
            add_project(sub_matches);
        },
        _ => unreachable!()
    }
}
