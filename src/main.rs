mod project;
mod command;
use command::*;

fn main() {
    let matches = create_main_command();

    match matches.get_matches().subcommand() {
        Some(("add", sub_matches)) => {
            add_project(sub_matches);
        },
        _ => unreachable!()
    }
}
