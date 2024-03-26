use std::fs;
use std::path::PathBuf;
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub ide: String,
    pub github_url: Option<String>,
    pub description: Option<String>,
    pub time: Option<ProjectTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTime {
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

pub fn load_projects(filepath: &str) -> serde_json::Result<Vec<Project>> {
    let projects = fs::read_to_string(filepath).expect("Can't open project file");
    serde_json::from_str(&projects)
}

impl Project {
    pub fn new(
        name: String, path: PathBuf, ide: String, github_url: Option<String>, description: Option<String>) -> Self {
        Self {
            name,
            path,
            ide,
            github_url,
            description,
            time: Some(ProjectTime::new())
        }
    }

    pub fn add_time(&mut self, elapsed_time: Option<TimeDelta>) {
        let timer = match &self.time {
            Some(time) => time.to_time_delta(),
            _ => TimeDelta::new(0, 0)
        };
        match (elapsed_time, timer) {
            (Some(elapsed), Some(current)) => {
                let total_time = current.checked_add(&elapsed);
                if let Some(time) = total_time {
                    self.time = Some(ProjectTime::from_time_delta(&time));
                }
            },
            (Some(elapsed), None) => {
                self.time = Some(ProjectTime::from_time_delta(&elapsed));
            },
            _ => return
        }
    }

    pub fn print_timer(&self) -> String {
        return match &self.time {
            Some(time) => format!("{} days, {}:{}:{}", time.days, time.hours, time.minutes, time.seconds),
            _ => "0 days, 0:0:0".to_string()
        }
    }
}

impl ProjectTime {
    pub fn new() -> Self {
        Self {
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0
        }
    }

    pub fn from_time_delta(delta: &TimeDelta) -> Self {
        Self {
            days: delta.num_days(),
            hours: delta.num_hours(),
            minutes: delta.num_minutes(),
            seconds: delta.num_seconds()
        }
    }

    pub fn to_time_delta(&self) -> Option<TimeDelta> {
        let d = TimeDelta::try_days(self.days);
        let h = TimeDelta::try_hours(self.hours);
        let m = TimeDelta::try_minutes(self.minutes);
        let s = TimeDelta::try_seconds(self.seconds);
        d?.checked_add(&h.unwrap())?.checked_add(&m.unwrap())?.checked_add(&s.unwrap())
    }
}