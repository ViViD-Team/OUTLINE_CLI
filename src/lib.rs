use std::{path::Path, fs};

pub mod sample_files;
pub mod models;


pub fn read_to_string(path: String) -> String {

    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Error reading file {}", path))
}

pub fn inside_plugin() -> bool {

    Path::new("plugin.json").exists()
}