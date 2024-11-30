
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;
use chrono::{Utc, DateTime};

/// Represents the various statuses a run can have.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, Copy, Default)]
pub enum RunStatus {
    #[default]
    NotStarted,
    Running,
    Completed,
    Failed,
    Expired,
    RequiredAction,
}

impl fmt::Display for RunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents metadata for a specific run instance.
#[derive(Debug, Serialize, Deserialize)]
pub struct RunMetadata {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: RunStatus,
}

impl RunMetadata {
    /// Creates a new RunMetadata instance with default values.
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            status: RunStatus::NotStarted,
        }
    }

    /// Updates the status of the run and modifies the timestamp.
    pub fn update_status(&mut self, new_status: RunStatus) {
        self.status = new_status;
        self.updated_at = Utc::now();
    }
}

/// Manages the state of multiple runs in a thread-safe way.
pub struct RunManager {
    runs: Arc<Mutex<Vec<RunMetadata>>>,
}

impl RunManager {
    /// Creates a new RunManager instance.
    pub fn new() -> Self {
        Self {
            runs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds a new run to the manager.
    pub fn add_run(&self) -> Uuid {
        let mut runs = self.runs.lock().unwrap();
        let new_run = RunMetadata::new();
        let id = new_run.id;
        runs.push(new_run);
        id
    }

    /// Updates the status of a run by ID.
    pub fn update_run_status(&self, run_id: Uuid, status: RunStatus) -> Result<(), String> {
        let mut runs = self.runs.lock().unwrap();
        if let Some(run) = runs.iter_mut().find(|r| r.id == run_id) {
            run.update_status(status);
            Ok(())
        } else {
            Err(format!("Run with ID {} not found.", run_id))
        }
    }

    /// Fetches metadata for a specific run by ID.
    pub fn get_run(&self, run_id: Uuid) -> Option<RunMetadata> {
        let runs = self.runs.lock().unwrap();
        runs.iter().find(|r| r.id == run_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_metadata_creation() {
        let metadata = RunMetadata::new();
        assert_eq!(metadata.status, RunStatus::NotStarted);
    }

    #[test]
    fn test_run_status_update() {
        let mut metadata = RunMetadata::new();
        metadata.update_status(RunStatus::Running);
        assert_eq!(metadata.status, RunStatus::Running);
    }

    #[test]
    fn test_run_manager_add_run() {
        let manager = RunManager::new();
        let run_id = manager.add_run();
        assert!(manager.get_run(run_id).is_some());
    }

    #[test]
    fn test_run_manager_update_status() {
        let manager = RunManager::new();
        let run_id = manager.add_run();
        assert!(manager.update_run_status(run_id, RunStatus::Completed).is_ok());
        assert_eq!(manager.get_run(run_id).unwrap().status, RunStatus::Completed);
    }

    #[test]
    fn test_run_manager_get_run() {
        let manager = RunManager::new();
        let run_id = manager.add_run();
        let run = manager.get_run(run_id);
        assert!(run.is_some());
    }
}
