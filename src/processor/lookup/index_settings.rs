use super::{Lookup, LookupDisplay};
use crate::data::{IndexSettings, IndicesSettings};

impl From<IndicesSettings> for Lookup<IndexSettings> {
    fn from(mut indices_settings: IndicesSettings) -> Self {
        let mut lookup = Lookup::<IndexSettings>::new();
        indices_settings.drain().for_each(|(name, settings)| {
            let id = settings.settings.index.uuid.clone();
            lookup
                .add(settings.settings.index)
                .with_name(&name)
                .with_id(&id);
        });
        lookup
    }
}

impl LookupDisplay for IndexSettings {
    fn display() -> &'static str {
        "index_settings"
    }
}
