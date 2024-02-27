use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Editor
{
    pub name: String,
    pub command: String,
}