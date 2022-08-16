use crate::util;
use dirs::home_dir;
use std::collections::HashMap;

pub struct Registries(HashMap<String, String>);

impl Registries {
    pub fn new() -> Self {
        let mut registries = HashMap::new();

        if let Ok(cargo_config_toml) =
            util::toml_from_file(home_dir().unwrap().join(".cargo").join("config"))
        {
            cargo_config_toml
                .get("registries")
                .iter()
                .for_each(|registry| {
                    if let Some(registry) = registry.as_table() {
                        registry.iter().for_each(|(name, table)| {
                            registries.insert(
                                table.get("index").unwrap().as_str().unwrap().to_owned(),
                                name.to_owned(),
                            );
                        })
                    }
                });
        }

        Self(registries)
    }

    pub fn from_source(&self, source: &str) -> Option<String> {
        self.0.get(&source.replace("registry+", "")).cloned()
    }
}
