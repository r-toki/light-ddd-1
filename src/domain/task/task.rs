use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct Task {
    pub id: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewTask {
    pub description: String,
}

impl Task {
    pub fn new(new_task: NewTask) -> Result<Self> {
        let now = Utc::now();
        let task = Self {
            id: Ulid::new().to_string(),
            description: new_task.description,
            completed: false,
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        task.validate()?;
        Ok(task)
    }

    pub fn done(&mut self) {
        self.completed = true;
        self.updated_at = Utc::now();
    }

    pub fn undone(&mut self) {
        self.completed = false;
        self.updated_at = Utc::now();
    }
}
