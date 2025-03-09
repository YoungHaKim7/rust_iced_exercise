use serde::{Deserialize, Serialize};

use super::{filter::Filter, task_states::Task};

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    pub input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
}
