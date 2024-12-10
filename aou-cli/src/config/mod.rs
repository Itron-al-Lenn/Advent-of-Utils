use advent_of_utils::Parts;
use std::{error::Error, path::PathBuf};
use tokio::fs::{read_dir, DirEntry};

#[derive(Clone)]
pub struct Config {
    pub year: i32,
    pub day: Option<u8>,
    pub part: Option<Parts>,
    pub test_mode: bool,
    pub input_dir: String,
    pub workspace_dir: PathBuf,
    pub exclude_parse_time: bool,
}

impl Config {
    pub async fn loader_paths(&self) -> Result<Vec<DirEntry>, Box<dyn Error>> {
        let mut matching_files = Vec::new();

        let mut dir = read_dir(&self.workspace_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.contains(&self.year.to_string()) {
                    matching_files.push(entry);
                }
            }
        }

        Ok(matching_files)
    }

    pub fn new(
        year: i32,
        day: Option<u8>,
        part: Option<Parts>,
        test_mode: bool,
        input_dir: String,
        workspace_dir: String,
        exclude_parse_time: bool,
    ) -> Self {
        Self {
            year,
            day,
            part,
            test_mode,
            input_dir,
            workspace_dir: (workspace_dir + "/target/release").into(),
            exclude_parse_time,
        }
    }
}
