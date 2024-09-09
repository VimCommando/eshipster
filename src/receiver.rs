mod file;

use crate::data::IndicesStats;
use color_eyre::eyre::{eyre, Result};
use file::FileReceiver;
use std::path::Path;

trait Receive {
    fn is_connected(&self) -> bool;
    fn read_indices_stats(&self) -> Result<IndicesStats>;
}

pub enum Receiver {
    File(FileReceiver),
}

impl Receiver {
    pub fn parse(input: &str) -> Result<Self> {
        let path = Path::new(&input);
        if path.is_file() {
            let file_receiver = FileReceiver::new(path.to_path_buf())?;
            return Ok(Self::File(file_receiver));
        }
        Err(eyre!("Could not parse input"))
    }

    pub fn read_indices_stats(&self) -> Result<IndicesStats> {
        match self {
            Receiver::File(file_receiver) => file_receiver.read_indices_stats(),
        }
    }
}

impl std::fmt::Display for Receiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Receiver::File(file_receiver) => write!(f, "file {}", file_receiver),
        }
    }
}
