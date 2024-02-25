use clap::Command;

pub fn get_list_subcommand() -> Command {
    Command::new("list")
        .about("Lists all the projects")
}