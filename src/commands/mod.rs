pub mod add;
pub mod list;
pub mod info;
pub mod remove;

use clap::{Command};
use crate::commands::add::get_add_subcommand;
use crate::commands::info::{get_info_subcommand};
use crate::commands::list::get_list_subcommand;
use crate::commands::remove::get_remove_subcommand;

pub fn create_main_command() -> Command {
    Command::new("flow")
        .author("Assane Diouf")
        .about("Flow is a ittle cli to manage your projects")
        .subcommand(get_list_subcommand())
        .subcommand(Command::new("open").about("Open the project. If the path is not found, tries a git clone"))
        .subcommand(get_add_subcommand())
        .subcommand(get_remove_subcommand())
        .subcommand(Command::new("update").about("Modify the description, name, path and git repository"))
        .subcommand(get_info_subcommand())
}