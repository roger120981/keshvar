use crate::structs::CountryInfo;
use crate::utils;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[allow(clippy::too_many_arguments)]
pub fn generate(
    default_file: &PathBuf,
    destination_file: &PathBuf,
    countries_info_list: &[(String, CountryInfo)],
    country_feature_list: &[String],
    continent_features: &HashMap<String, Vec<String>>,
    region_features: &HashMap<String, Vec<String>>,
    subregion_features: &HashMap<String, Vec<String>>,
    world_region_features: &HashMap<String, Vec<String>>,
) -> Result<()> {
    let default_contents = fs::read_to_string(default_file).context(format!(
        "Could not read and decode Cargo default file {default_file:?}",
    ))?;
    let mut cargo_toml_file = utils::create_new_file(destination_file, "Cargo")?;
    cargo_toml_file.write_all(format!("# Copied from {default_file:?}\n").as_bytes())?;
    cargo_toml_file.write_all(default_contents.as_bytes())?;
    cargo_toml_file.write_all(
        r#"

# Automatically generated via `build.rs`
[features]
serde-derive = ["serde"]
subdivisions = []
constants = []
geo = []
translations = []
chrono-integration = ["chrono"]
"#
        .as_bytes(),
    )?;
    let mut feature_writer =
        |comment: String, feature: &HashMap<String, Vec<String>>| -> Result<()> {
            cargo_toml_file.write_all(format!("\n# {}:\n", comment).as_bytes())?;
            for (feature_name, sub_feature_list) in feature {
                cargo_toml_file.write_all(feature_name.as_bytes())?;
                cargo_toml_file.write_all(b" = [")?;
                cargo_toml_file.write_all(
                    sub_feature_list
                        .iter()
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<_>>()
                        .join(", ")
                        .as_bytes(),
                )?;
                cargo_toml_file.write_all(b"]\n")?;
            }
            Ok(())
        };

    // Regions and Continents have Asia, Europe, and Africa in common!
    let mut new_region_features = HashMap::new();
    for (region, feature_list) in region_features {
        if ["asia", "europe", "africa"].contains(&region.as_str()) {
            new_region_features.insert("region-".to_owned() + region, feature_list.clone());
        } else {
            new_region_features.insert(region.clone(), feature_list.clone());
        }
    }
    // Subregions and Continents have SouthAmerica in common!
    let mut new_subregion_features = HashMap::new();
    for (subregion, feature_list) in subregion_features {
        if "south-america" == subregion {
            new_subregion_features
                .insert("subregion-south-america".to_string(), feature_list.clone());
        } else {
            new_subregion_features.insert(subregion.clone(), feature_list.clone());
        }
    }
    feature_writer("Continents".to_string(), continent_features)?;
    feature_writer("Regions".to_string(), &new_region_features)?;
    feature_writer("Subregions".to_string(), &new_subregion_features)?;
    feature_writer("World Regions".to_string(), world_region_features)?;
    cargo_toml_file.write_all(
        r#"
# Default to all countries:
default = ["#
            .as_bytes(),
    )?;
    cargo_toml_file.write_all(
        country_feature_list
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<_>>()
            .join(", ")
            .as_bytes(),
    )?;
    cargo_toml_file.write_all(b"]\n")?;
    cargo_toml_file.write_all(b"\n#Countries:\n")?;
    for (_, info) in countries_info_list.iter() {
        cargo_toml_file.write_all(
            format!(
                "{} = []  # {} {}\n",
                info.feature_name,
                info.iso_long_name,
                if let Some(ref region) = info.region {
                    format!("({})", region)
                } else {
                    "".to_string()
                }
            )
            .as_bytes(),
        )?;
    }
    Ok(())
}
