use std::env;
use std::fs;
use std::path::Path;

use opc::get_settings;
use opc::models::Command;
use opc::models::Element;
use opc::models::Node;
use opc::models::Opb;
use opc::models::PluginJson;
use opc::models::Prototype;
use opc::models::RawCommand;
use opc::models::Widget;
use opc::sample_files;
use opc::inside_plugin;
use opc::set_settings;

fn main() {

    let version = "0.1";

    let mut args = env::args();

    args.next();

    let command = Command::new(args);

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
        Command::Add(c) => {
            match c {
                Element::Node(name) => add_node(name),
                Element::Widget(name) => add_widget(name),
            }
        },
        Command::Bundle => bundle(),
        Command::Extract(c) => extract_from(c.origin_path),
        Command::Remove(p) => remove_element(p), 
        Command::Help(c) => {
            match c {
                RawCommand::Add => "Adds a new widget or node to the plugin.\nUsage: opc add [ELEMENT TPYE] [ELEMENT NAME]".to_string(),
                RawCommand::Bundle => "Bundles the plugin into a .opb file.\nUsage: opc bundle".to_string(),
                RawCommand::Version => "
OUTLINE's plugin creator

Usage: opc [COMMAND] [ARGUMENTS] [OPTIONS]

Commands:
    create          Create the basic filtree for a new outline plugin
    add             Add a new element to the plugin
    remove          Remove an element from the plugin
    bundle          Bundle the plugin to .opb file
    extract         Unbundle a .opb file

Running opc without any arguments will print version info and exit.

See 'opc help <command>' for more information on a specific command.
                            ".to_string(),
                RawCommand::Help => "Display additional information for specific commands.\nUsage: opc help [COMMAND]".to_string(),
                RawCommand::Create => "Create a new OUTLINE plugin with the provided name.\nCalling with '-blank' omits sample data.\nUsage: opc create [PLUGIN NAME] [-blank]".to_string(),
                RawCommand::Extract => "Extract a .opb file to the corresponding source files.\nUsage: opc extract [FILE PATH]".to_string(),
                RawCommand::Remove => "Remove an element from the plugin. \nUsage: opc remove [ELEMENT TPYE] [ELEMENT NAME]".to_string()
            }
        }
        Command::Version => {
            format!("OUTLINE Plugin Creator {} installed", version)
        }
    })
}

fn remove_element(elem: Element) -> String {

    match elem {
        Element::Node(name) => {
            let mut settings = get_settings();
            settings.remove_node(name.clone()).expect("Error removing Node");
            fs::remove_file(name.clone() + ".js").expect("Node file not found, removed Node from plugin.json");
            format!("Deleted Node {}", &name)
        }
        Element::Widget(name) => {
            let mut settings = get_settings();
            settings.remove_widget(name.clone()).expect("Error removing Widget");
            fs::remove_dir_all(&name).expect("Widget directory not found, removed Widget from plugin.json");
            format!("Deleted Widget {}", name)
        }
    }
}

fn create_plugin(name: &str) -> String {

    let name = name.to_string();
    
    if Path::new(name.as_str()).exists() {
        return "Directory with the same name already exists!".to_string()
    }

    fs::create_dir(&name).expect("Error creating new directory");
    fs::create_dir(name.clone() + "/sampleWidget").expect("Error creating new directory");

    fs::write(name.clone() + "/icon.svg", sample_files::get_sample_icon_svg()).expect("Error creating new file");
    fs::write(name.clone() + "/SampleNode.js", sample_files::get_sample_node_js(None)).expect("Error creating new file");

    let path = name.clone() + "/sampleWidget";

    fs::write(path.clone() + "/sampleWidget.css", sample_files::get_sample_widget_css()).expect("Error creating new file");
    fs::write(path.clone() + "/sampleWidget.html", sample_files::get_sample_widget_html()).expect("Error creating new file");
    fs::write(path.clone() + "/sampleWidget.js", sample_files::get_sample_widget_js()).expect("Error creating new file");
    fs::write(path + "/sampleWidget.svg", sample_files::get_sample_widget_svg()).expect("Error creating new file");

    let mut plugin_name: Vec<String> = name.chars().map(|a| {
        if a.is_ascii_uppercase() {
            " ".to_string() + &a.to_string()
        } else {
            a.to_string()
        }
    }).collect();

    plugin_name[0] = plugin_name[0].to_ascii_uppercase();

    let settings = PluginJson {
        plugin_id: name.clone(),
        plugin_name: plugin_name.into_iter().collect(),
        ..Default::default()
    };

    fs::write(name.clone() + "/plugin.json", serde_json::to_string_pretty(&settings).expect("Error serializing settings")).expect("Error creating new file");
    

    format!("Plugin {} created at ./{}", name.clone(), name)
}

fn create_plugin_blank(name: &str) -> String {

    let name = name.to_string();

    fs::create_dir(&name).expect("Error creating new directory");

    let settings: PluginJson = PluginJson {
        plugin_name: name.clone(),
        plugin_id: split_l_camel_case(&name),
        widgets: Vec::new(),
        nodes: Vec::new(),
        ..Default::default()
    };

    fs::write(name.clone() + "/plugin.json", serde_json::to_string_pretty(&settings).expect("Error serializing settings")).expect("Error creating new file");

    format!("Plugin {} created at ./{}", name.clone(), name)
}

pub fn add_widget(name: String) -> String {

    inside_plugin!(no_res);

    if Path::new(&name).exists() {
        return "Widget with this ID already exists in this plugin!".to_string()
    }

    fs::create_dir(&name).expect("Error creating new directory");

    fs::write(name.clone() + "/" + &name + ".css", "").expect("Error creating new file");
    fs::write(name.clone() + "/" + &name + ".html", "").expect("Error creating new file");
    fs::write(name.clone() + "/" + &name + ".js", sample_files::get_widget_js(&name)).expect("Error creating new file");
    fs::write(name.clone() + "/" + &name + ".svg", "").expect("Error creating new file");

    let mut settings: PluginJson = get_settings();

    settings.widgets.push(Widget { widget_name: split_l_camel_case(&name), widget_id: name.clone(), prototype: Prototype::default() });

    set_settings(&settings);

    format!("Generated {} widget. Make sure to customize the plugin.json!", name)
}

pub fn add_node(name: String) -> String {

    inside_plugin!(no_res);

    // Maybe rather check plugin.json?
    if Path::new(&(name.clone() + ".js")).exists() {
        return "Node with this ID already exists in this plugin!".to_string()
    }

    fs::write(name.clone() + ".js", sample_files::get_sample_node_js(Some(name.clone()))).expect("Error creating new file");

    let mut settings: PluginJson = get_settings();

    settings.nodes.push(Node { node_name: split_l_camel_case(&name), node_id: name.clone() });

    set_settings(&settings);

    format!("Generated {} node. Make sure to customize the plugin.json", name)
}

pub fn bundle() -> String {

    inside_plugin!(no_res);

    let settings: PluginJson = get_settings();

    let res = Opb::bundle();

    if let Err(e) = res {
        return format!("{}", e)
    }

    fs::write("../".to_string() + &settings.plugin_id + ".opb", serde_json::to_string(&res.unwrap()).expect("Error serializing bundle")).expect("Error writing to file");

    format!("Plugin bundled to {}.opb", settings.plugin_id)
}

pub fn extract_from(origin_path: String) -> String {

    let origin_path = origin_path + ".opb";

    if !Path::new(&origin_path).exists() {
        return "File not found".to_string()
    }

    let opb: Opb = serde_json::from_str(&fs::read_to_string(&origin_path).expect("Error reading file")).expect("Error deserializing bundle");
    let plugin: PluginJson = serde_json::from_str(&fs::read_to_string(&origin_path).expect("Error reading file")).expect("Error deserializing bundle");

    fs::create_dir(&plugin.plugin_id).expect("A subdirectory with this name already exists");

    fs::write(plugin.plugin_id.clone() + "/plugin.json", serde_json::to_string_pretty(&plugin).expect("Error serializing plugin.json")).expect("Error writing to file");

    fs::write(plugin.plugin_id.clone() + "/icon.svg", opb.icon.svg).expect("Error writing to file");

    for node in opb.nodes {
        fs::write(plugin.plugin_id.clone() + "/" + &node.node_id + ".js", node.js.js).expect("Error writing to file");
    }

    for widget in opb.widgets {
        fs::create_dir(plugin.plugin_id.clone() + "/" + &widget.widget_id).expect("Error creating directory");
        let path = plugin.plugin_id.clone() + "/" + &widget.widget_id + "/" + &widget.widget_id;
        fs::write(path.clone() + ".js", widget.file_contents.js).expect("Error writing to file");
        fs::write(path.clone() + ".css", widget.file_contents.css).expect("Error writing to file");
        fs::write(path.clone() + ".html", widget.file_contents.html).expect("Error writing to file");
        fs::write(path.clone() + ".svg", widget.file_contents.svg).expect("Error writing to file");
    }

    format!("Extracted plugin {}", opb.plugin_id)
}



fn split_l_camel_case(s: &str) -> String {

    let mut s: Vec<String> = s.to_string().chars().map(|a| {
        if a.is_ascii_uppercase() {
            " ".to_string() + &a.to_string()
        } else {
            a.to_string()
        }
    }).collect();

    s[0] = s[0].to_ascii_uppercase();

    s.into_iter().collect()
}