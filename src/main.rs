use std::env;
use std::fs;
use std::path::Path;

use opc::sample_files;

fn main() {

    let version = "0.1";

    let mut args = env::args().into_iter();

    args.next();

    let command = args.next();

    if command.is_none() {
        println!("OUTLINE Plugin Creator version {} installed", version);
        return;
    }

    let command = command.unwrap();

    println!("{}", match command.clone().as_str() {
        "create" | "c" => {
            let arg = args.next();
            
            if arg.is_none() {
                "Please provide a name in lowerCamelCase for your plugin".to_string()
            } else if let Some(option) = args.next() {
                if option == "-blank" {
                    create_plugin_blank(&arg.unwrap())
                } else {
                    "Invalid option. Only '-blank' available".to_string()
                }
            } else {
                create_plugin(&arg.unwrap())
            }
        }
        "add" | "a" => {
            if let Some(elem_type) = args.next() {
                let name = args.next();
                if name.is_none() {"Missing element name.".to_string()}
                else if elem_type == "widget" {
                    add_widget(name.unwrap())
                } else if elem_type == "node" {
                    add_node(name.unwrap())
                } else {
                    "Invalid argument: element type.\nAvailable types are\n     widget\n     node".to_string()
                }
            } else {
                "Missing argument: element type.\nAvailable types are\n     widget\n     node".to_string()
            }
        }
        "bundle" | "b" => {
            "bundle".to_string()
        }
        "extract" | "e" => {
            "extract".to_string()
        }
        "help" => {
            "help command".to_string()
        }
        _ => {
            "
OUTLINE's plugin creator

Usage: opc [COMMAND] [ARGUMENTS] [OPTIONS]

Commands:
    create, c       Create the basic filtree for a new outline plugin
    add, a          Add a new element to the plugin
    bundle, b       Bundle the plugin to .opb file
    extract, e      Debundle a .opb file

Running opc without any arguments will print version info and exit.

See 'opc help <command>' for more information on a specific command.
            ".to_string()
        }
    })
}


fn create_plugin(name: &str) -> String {

    let name = name.to_string();
    
    if Path::new(name.as_str()).exists() {
        return "Directory with the same name already exists!".to_string()
    }

    fs::create_dir(&name).expect("Error creating new directory");
    fs::create_dir(name.clone() + "/sampleWidget").expect("Error creating new directory");

    fs::write(name.clone() + "/icon.svg", sample_files::get_icon_svg()).expect("Error creating new file");
    fs::write(name.clone() + "/SampleNode.js", sample_files::get_node_js()).expect("Error creating new file");

    let path = name.clone() + "/sampleWidget";

    fs::write(path.clone() + "/sampleWidget.css", sample_files::get_widget_css()).expect("Error creating new file");
    fs::write(path.clone() + "/sampleWidget.html", sample_files::get_widget_html()).expect("Error creating new file");
    fs::write(path.clone() + "/sampleWidget.js", sample_files::get_widget_js()).expect("Error creating new file");
    fs::write(path.clone() + "/sampleWidget.svg", sample_files::get_widget_svg()).expect("Error creating new file");

    let mut settings = sample_files::get_plugin_json();

    settings.plugin_id = name.clone();
    let mut plugin_name: Vec<String> = name.clone().chars().into_iter().map(|a| {
        if a.is_ascii_uppercase() {
            " ".to_string() + &a.to_string()
        } else {
            a.to_string()
        }
    }).collect();

    plugin_name[0] = plugin_name[0].to_ascii_uppercase();

    settings.plugin_name = plugin_name.into_iter().collect();

    fs::write(name.clone() + "/plugin.json", serde_json::to_string_pretty(&settings).expect("Error serializing settings")).expect("Error creating new file");
    

    format!("Plugin {} created at ./{}", name.clone(), name)
}

fn create_plugin_blank(name: &str) -> String {

    let name = name.to_string();

    fs::create_dir(&name).expect("Error creating new directory");

    let mut settings = sample_files::get_plugin_json();

    settings.plugin_id = name.clone();
    let mut plugin_name: Vec<String> = name.clone().chars().into_iter().map(|a| {
        if a.is_ascii_uppercase() {
            " ".to_string() + &a.to_string()
        } else {
            a.to_string()
        }
    }).collect();

    plugin_name[0] = plugin_name[0].to_ascii_uppercase();

    settings.plugin_name = plugin_name.into_iter().collect();

    fs::write(name.clone() + "/plugin.json", serde_json::to_string_pretty(&settings).expect("Error serializing settings")).expect("Error creating new file");

    format!("Plugin {} created at ./{}", name.clone(), name)
}

pub fn add_widget(name: String) -> String {

    "add widget".to_string()
}

pub fn add_node(name: String) -> String {

    "add node".to_string()
}