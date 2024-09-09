use super::Export;
use crate::data::ShardDoc;
use color_eyre::eyre::Result;

pub struct StreamExporter {}

impl StreamExporter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Export for StreamExporter {
    fn write(&self, docs: Vec<ShardDoc>) -> Result<()> {
        log::debug!("Writing {} docs to stdout", docs.len());
        for doc in docs {
            serde_json::to_writer(std::io::stdout(), &doc)?;
            println!();
        }
        Ok(())
    }

    fn is_connected(&self) -> bool {
        true
    }
}

impl std::fmt::Display for StreamExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stdout")
    }
}
