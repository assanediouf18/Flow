use chrono::{DateTime, Local, TimeDelta};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowTimer {
    pub project: String,
    pub started: Option<DateTime<Local>>,
}

impl FlowTimer {
    pub fn new(project_name: String) -> Self {
        Self {
            project: project_name,
            started: Some(Local::now()),
        }
    }

    pub fn get_duration(&self) -> Option<TimeDelta> {
        if let Some(started_time) = self.started {
            return Some(Local::now().signed_duration_since(started_time));
        }
        return None;
    }
}