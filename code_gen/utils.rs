use crate::structs::CountryInfo;
use anyhow::{Context, Result};
use std::{fs, fs::File, io::Write, path::PathBuf};

pub const LOG_FILE: &str = "build.log";

#[macro_export]
macro_rules! log {
    ($text:expr) => {
        {$crate::utils::append_to_log_file(format!("{}:{} | {}\n", file!(), line!(), $text)); true};
    };
    ($text:expr, $($parameters:expr),+) => {
        {$crate::utils::append_to_log_file(format!("{}:{} | {}\n", file!(), line!(), format!($text, $($parameters),+))); true}
    }
}
pub use log;

pub fn remove_log_file() {
    let _ = fs::remove_file(LOG_FILE);
}

#[inline]
pub fn append_to_log_file(text: String) {
    File::options()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap()
}

pub fn create_new_file(destination_file: &PathBuf, title: &str) -> Result<File> {
    File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(destination_file.clone())
        .context(format!(
            "Could not open {title} destination file {destination_file:?}",
        ))
}

#[inline]
pub fn set_none_if_empty_string(maybe_text: &mut Option<String>) {
    if let Some(ref text) = maybe_text {
        if text.trim().is_empty() {
            *maybe_text = None;
        }
    }
}

pub fn to_module_name(text: &str) -> String {
    let text = text.trim().to_lowercase();
    if ["as", "in", "do"].contains(&&*text) {
        format!("r#{}", text)
    } else {
        text
    }
}

pub fn write_first_comments(file: &mut File, creator: &str) -> std::io::Result<()> {
    file.write_all(
        format!("// DO NOT TOUCH THIS FILE. (Auto-generated via `{creator}`)\n\n").as_bytes(),
    )
}

#[inline]
pub fn to_float_string(x: f64) -> String {
    let mut string = format!("{}", x);
    if !string.contains('.') {
        string += ".0"
    }
    string
}

pub fn option_string_to_string(maybe_text: &Option<String>) -> String {
    if let Some(text) = maybe_text {
        format!("Some({:?})", text)
    } else {
        "None".to_string()
    }
}

pub fn option_f64_to_string(maybe_float: &Option<f64>) -> String {
    if let Some(float) = maybe_float {
        format!("Some({})", to_float_string(*float))
    } else {
        "None".to_string()
    }
}

pub fn capitalize(text: &str) -> String {
    // Copied from https://stackoverflow.com/a/38406885
    fn uppercase_first_letter(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
    let text = text.replace(['-', '_'], " ");
    let mut result = String::new();
    for word in text.split(' ') {
        result += uppercase_first_letter(word).as_str();
    }
    result
}

// pub fn commented_country_name(info: &CountryInfo) -> String {
//     format!(
//         "// {} ({})\n",
//         info.iso_long_name,
//         if let Some(ref region_name) = info.region {
//             region_name
//         } else {
//             ""
//         }
//     )
// }

pub fn country_cfg_feature(info: &CountryInfo) -> String {
    format!("#[cfg(feature = {:?})]\n", info.feature_name)
}

pub fn country_cfg_features(
    country_feature_list: Vec<String>,
    condition: &str,
    indent: usize,
) -> String {
    let mut cfg = "    ".repeat(indent) + "#[\n";
    cfg += &("    ".repeat(indent + 1) + "cfg(\n");
    cfg += &("    ".repeat(indent + 2) + condition + "(\n");
    country_feature_list.iter().for_each(|country_feature| {
        cfg += &("    ".repeat(indent + 3) + &format!("feature = {:?},\n", country_feature))
    });
    cfg += &("    ".repeat(indent + 2) + ")\n");
    cfg += &("    ".repeat(indent + 1) + ")\n");
    cfg += &("    ".repeat(indent) + "]\n");
    cfg
}

pub fn country_cfg_not_features(
    country_feature_list: Vec<String>,
    condition: &str,
    indent: usize,
) -> String {
    let mut cfg = "    ".repeat(indent) + "#[\n";
    cfg += &("    ".repeat(indent + 1) + "cfg(\n");
    cfg += &("    ".repeat(indent + 2) + "not(\n");
    cfg += &("    ".repeat(indent + 3) + condition + "(\n");
    country_feature_list.iter().for_each(|country_feature| {
        cfg += &("    ".repeat(indent + 4) + &format!("feature = {:?},\n", country_feature))
    });
    cfg += &("    ".repeat(indent + 3) + ")\n");
    cfg += &("    ".repeat(indent + 2) + ")\n");
    cfg += &("    ".repeat(indent + 1) + ")\n");
    cfg += &("    ".repeat(indent) + "]\n");
    cfg
}

pub fn country_cfg_feature_and_commented_name(info: &CountryInfo, indent: usize) -> String {
    format!(
        "{}#[cfg(feature = {:?})] // {}\n",
        "    ".repeat(indent),
        info.feature_name,
        country_name(info),
    )
}

pub fn country_name(info: &CountryInfo) -> String {
    format!(
        "{} {}",
        info.iso_long_name,
        if let Some(ref region_name) = info.region {
            format!("({})", region_name)
        } else {
            String::new()
        }
    )
}

pub fn country_cfg_feature_and_doc_commented_name(info: &CountryInfo, indent: usize) -> String {
    country_cfg_feature_and_commented_name(info, indent).replace("//", "///")
}

pub fn to_cargo_feature_name(text: &str) -> String {
    text.to_lowercase().replace(' ', "-")
}
