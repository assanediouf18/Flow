pub mod add;
pub mod list;
pub mod info;
pub mod remove;
pub mod update;
pub mod open;

use clap::{Command};
use crate::commands::add::get_add_subcommand;
use crate::commands::info::{get_info_subcommand};
use crate::commands::list::get_list_subcommand;
use crate::commands::open::get_open_subcommand;
use crate::commands::remove::get_remove_subcommand;
use crate::commands::update::get_update_command;

pub fn create_main_command() -> Command {
    Command::new("flow")
        .author("Assane Diouf")
        .about("Flow is a ittle cli to manage your projects")
        .subcommand(get_list_subcommand())
        .subcommand(get_open_subcommand())
        .subcommand(get_add_subcommand())
        .subcommand(get_remove_subcommand())
        .subcommand(get_update_command())
        .subcommand(get_info_subcommand())
}