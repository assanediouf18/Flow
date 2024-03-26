use std::sync::mpsc::channel;
use chrono::TimeDelta;
use clap::{Arg, ArgMatches, Command};
use crate::config::Configuration;
use crate::flow_timer::FlowTimer;
use crate::project::Project;

pub fn get_start_time_subcommand() -> Command {
    Command::new("start-timer")
        .about("Starts the timer for a project")
        .arg(Arg::new("name").help("The name of the project"))
}

pub fn get_stop_time_subcommand() -> Command {
    Command::new("stop-timer")
        .about("Stops the timer for a project")
        .arg(Arg::new("name").help("The name of the project"))
}

pub fn start_timer_command(projects: &mut Vec<Project>, timers: &mut Vec<FlowTimer>, sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must enter a project name");
    if let Some(project) = projects.iter_mut().find(|p| { p.name.to_lowercase() == name.to_lowercase() }) {
        start_time(project.name.clone(), timers);
    }
    else {
        println!("The project '{name}' does not exist");
    }
}

pub fn stop_timer_command(config: &Configuration, projects: &mut Vec<Project>, timers: &mut Vec<FlowTimer>, sub_matches: &ArgMatches) {
    let name = sub_matches.get_one::<String>("name").expect("You must enter a project name");
    if let Some(project) = projects.iter_mut().find(|p| { p.name.to_lowercase() == name.to_lowercase() }) {
        let duration = stop_time(project.name.clone(), timers);
        project.add_time(duration);
        config.update_config_file(&projects);
    }
    else {
        println!("The project '{name}' does not exist");
    }
}

pub fn start_time(project_name: String, timers: &mut Vec<FlowTimer>) {
    let (tx, rx) = channel();
    timers.push(FlowTimer::new(project_name));
    println!("Flow started a timer to measure your work time.");
    println!("You can stop the execution (Ctrl + C) if you want and then Flow will stop timing you.");
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
    rx.recv().expect("Could not receive from channel.");
    println!("Updating the timer of the project");
}

pub fn stop_time(project_name: String, timers: &mut Vec<FlowTimer>) -> Option<TimeDelta> {
    let position = timers.iter().position(|timer| { timer.project == project_name });
    return match position {
        Some(position) => {
            let timer = timers.get(position);
            let delta = timer?.get_duration();
            if let Some(_) = delta {
                timers.swap_remove(position);
            }
            delta
        },
        _ => {
            println!("No timer found for the project {project_name}.");
            None
        }
    }
}