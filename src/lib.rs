use std::fs;
use anyhow::Result;

use models::PluginJson;

pub mod sample_files;
pub mod models;


pub fn read_to_string(path: String) -> String {

    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Error reading file {}", path))
}

pub fn get_settings() -> PluginJson {
    inside_plugin!(panic);
    serde_json::from_str::<PluginJson>(
        &read_to_string("plugin.json".to_string())
    ).expect("Error deserializing plugin.json")
}

pub fn set_settings(settings: &PluginJson) {
    inside_plugin!(panic);
    fs::write("plugin.json", serde_json::to_string_pretty(&settings).expect("Error serializing settings")).expect("Error writing to file");
}

#[macro_export]
macro_rules! name_is_conform {
    ($name: ident) => {
        if $name.chars().next().unwrap().is_ascii_uppercase() || $name.contains('_') || $name.contains('-') {
            return "Please provide a name in lowerCamelCase".to_string()
        }
    }
}

#[macro_export]
macro_rules! inside_plugin {
    (return) => {
        if !std::path::Path::new("plugin.json").exists() {
            return "You are not currently editing a plugin! Use opc create to create a new plugin, then run this command from the plugin folder.".to_string()
        }
    };
    ($mac:ident) => {
        if !std::path::Path::new("plugin.json").exists() {
            $mac!("You are not currently editing a plugin! Use opc create to create a new plugin, then run this command from the plugin folder.")
        }
    };
}

pub trait OpcCommand {
    fn run(&self) -> String;
    fn help() -> String;
}

pub trait SuperOpcCommand: OpcCommand {
    fn parse(args: Vec<String>) -> Option<Result<Self>> where Self: Sized;
}