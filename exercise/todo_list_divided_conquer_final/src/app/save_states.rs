use std::path::PathBuf;

use async_std::fs::{self, File};
use iced::futures::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};

use crate::error::{loaderror::LoadError, saveerror::SaveError};

use super::{filter::Filter, task_states::Task};

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    pub input_value: String,
    pub filter: Filter,
    pub tasks: Vec<Task>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    fn path() -> PathBuf {
        let mut path =
            if let Some(project_dirs) = directories::ProjectDirs::from("rs", "Iced", "Todos") {
                project_dirs.data_dir().into()
            } else {
                std::env::current_dir().unwrap_or_default()
            };
        path.push("todos.json");
        path
    }

    pub async fn load() -> Result<SavedState, LoadError> {
        let mut contents = String::new();
        let mut file = File::open(Self::path())
            .await
            .map_err(|_| LoadError::File)?;
        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::File)?;
        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;
        let path = Self::path();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).await.map_err(|_| SaveError::File)?;
        }
        {
            let mut file = File::create(path).await.map_err(|_| SaveError::File)?;
            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }
        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
impl SavedState {
    fn storage() -> Option<web_sys::Storage> {
        let window = web_sys::window()?;
        window.local_storage().ok()?
    }

    async fn load() -> Result<SavedState, LoadError> {
        let storage = Self::storage().ok_or(LoadError::File)?;
        let contents = storage
            .get_item("state")
            .map_err(|_| LoadError::File)?
            .ok_or(LoadError::File)?;
        serde_json::from_str(&contents).map_err(|_| LoadError::Format)
    }

    async fn save(self) -> Result<(), SaveError> {
        let storage = Self::storage().ok_or(SaveError::File)?;
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::Format)?;
        storage
            .set_item("state", &json)
            .map_err(|_| SaveError::Write)?;
        wasmtimer::tokio::sleep(std::time::Duration::from_secs(2)).await;
        Ok(())
    }
}
