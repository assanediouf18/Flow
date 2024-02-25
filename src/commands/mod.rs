pub mod add;
pub mod list;
pub mod info;

use std::io::{Read, Write};
use clap::{Command};
use crate::commands::add::get_add_subcommand;
use crate::commands::info::{get_info_subcommand};
use crate::commands::list::get_list_subcommand;

pub fn create_main_command() -> Command {
    Command::new("flow")
        .author("Assane Diouf")
        .about("Little cli to manage your projects")
        .subcommand(get_list_subcommand())
        .subcommand(Command::new("open").about("Open the project. If the path is not found, tries a git clone"))
        .subcommand(get_add_subcommand())
        .subcommand(Command::new("remove").about("Remove a project"))
        .subcommand(Command::new("update").about("Modify the description, name, path and git repository"))
        .subcommand(get_info_subcommand())
}