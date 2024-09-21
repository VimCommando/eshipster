use super::INDEX_TEMPLATE;
use crate::exporter::Exporter;
use color_eyre::eyre::{eyre, Result};

pub async fn elasticsearch(exporter: &Exporter) -> Result<()> {
    match exporter {
        Exporter::Elasticsearch(client) => {
            let index_template = &*INDEX_TEMPLATE;
            let response = client
                .send(
                    "PUT",
                    "_index_template/eshipster-shards",
                    Some(index_template),
                )
                .await?;
            if response.status_code().is_success() {
                log::info!("Succesfully setup index template");
                Ok(())
            } else {
                log::warn!("Failed to setup index template");
                log::debug!("{:?}", &response);
                let body = response.text().await?;
                Err(eyre!("Failed to setup index template: {}", &body))
            }
        }
        _ => Err(eyre!("Can only setup an Elasticsearch host")),
    }
}
