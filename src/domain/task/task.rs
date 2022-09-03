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

impl Task {
    fn new(description: &str) -> Result<Self> {
        let task = Self {
            id: Ulid::new().to_string(),
            description: description.to_string(),
            completed: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        task.validate()?;
        Ok(task)
    }

    fn done(&mut self) {
        self.completed = true;
        self.updated_at = Utc::now();
    }

    fn undone(&mut self) {
        self.completed = false;
        self.updated_at = Utc::now();
    }
}
