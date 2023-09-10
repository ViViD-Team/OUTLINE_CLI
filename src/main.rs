use std::env;
use std::fs;
use std::path::Path;

use opc::Command;
use opc::RawCommand;
use opc::sample_files;

fn main() {

    let version = "0.1";

    let mut args = env::args();

    args.next();

    let command = opc::Command::new(args);

    if let Err(e) = command {
        println!("{}", e);
        return;
    }

    let command = command.unwrap();

    println!("{}", match command {
        Command::Create(c) => {
            if c.blank {
                create_plugin_blank(&c.name)
            } else {
                create_plugin(&c.name)
            }
        }
        Command::AddNode(c) => add_node(c.name),
        Command::AddWidget(c) => add_widget(c.name),
        Command::Bundle => bundle(),
        Command::Extract(c) => extract_from(c.origin_path),
        Command::Help(c) => {
            match c {
                RawCommand::Add => unimplemented!(""),
                RawCommand::Bundle => unimplemented!(""),
                RawCommand::Version => unimplemented!(""),
                RawCommand::Help => unimplemented!(""),
                RawCommand::Create => unimplemented!(""),
                RawCommand::Extract => unimplemented!(""),
            }
        }
        Command::Manual => {
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
        Command::Version => {
            format!("OUTLINE Plugin Creator {} installed", version)
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

    unimplemented!("")
}

pub fn add_node(name: String) -> String {

    unimplemented!("")
}

pub fn bundle() -> String {

    unimplemented!("")
}

pub fn extract_from(origin_path: String) -> String {

    unimplemented!("")
}