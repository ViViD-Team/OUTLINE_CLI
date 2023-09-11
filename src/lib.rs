use std::{path::Path, fs};

use anyhow::{Result, anyhow, bail};
use sample_files::{FullWidget, FullNode};
use serde::{Serialize, Deserialize};

use crate::sample_files::{WidgetFiles, NodeFiles};

pub mod sample_files;


pub struct Arguments {
    pub command: Command
}

#[derive(Debug, Clone)]
pub enum RawCommand {
    Version,
    Help,
    Create,
    Add,
    Bundle,
    Extract
}

impl RawCommand {
    fn new(args: &mut std::env::Args) -> Result<RawCommand> {

        let cmd = args.next().unwrap_or("version".to_string());

        match cmd.as_str() {
            "create" => Ok(RawCommand::Create),
            "add" => Ok(RawCommand::Add),
            "bundle" => Ok(RawCommand::Bundle),
            "extract" => Ok(RawCommand::Extract),
            "help" => Ok(RawCommand::Help),
            "version" => Ok(RawCommand::Version),
            _ => Err(anyhow!("invalid command"))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Version,
    Help(RawCommand),
    Create(Create),
    AddNode(AddNode),
    AddWidget(AddWidget),
    Bundle,
    Extract(Extract),
}

impl Command {
    pub fn new(mut args: std::env::Args) -> Result<Command> {

        let cmd: RawCommand = RawCommand::new(&mut args)?;

        let out: Result<Command>;

        'base: {
            out = Ok(match cmd {
                RawCommand::Create => {
                    let c = Command::Create(Create {
                        name: args.next().unwrap_or("".to_string()),
                        blank: {
                            if let Some(option) = args.next() {
                                if option == "-blank" {true}
                                else {out = Err(anyhow!("invalid option")); break 'base;}
                            } else {false}
                        }
                    });
                    if let Command::Create(b) = c.clone() {
                        if b.name == "" {out = Err(anyhow!("missing plugin name")); break 'base;}
                    }
                    c
                }
                RawCommand::Add => {
                    if let Some(elem_type) = args.next() {
                        if let Some(name) = args.next() {
                            match elem_type.as_str() {
                                "widget" => {
                                    Command::AddWidget(AddWidget { name })
                                }
                                "node" => {
                                    Command::AddNode(AddNode { name })
                                }
                                _ => {out = Err(anyhow!("invalid element type")); break 'base}
                            }
                        } else {
                            out = Err(anyhow!("missing element name"));
                            break 'base;
                        }
                    } else {
                        out = Err(anyhow!("missing element type"));
                        break 'base;
                    }
                }
                RawCommand::Bundle => {
                    Command::Bundle
                }
                RawCommand::Extract => {
                    if let Some(path) = args.next() {
                        Command::Extract(Extract { origin_path: path })
                    } else {out = Err(anyhow!("missing path of .opb")); break 'base}
                }
                RawCommand::Version => {
                    Command::Version
                }
                RawCommand::Help => {
                    Command::Help(RawCommand::new(&mut args)?)
                }
            })
        };
        out
    }
}

#[derive(Debug, Clone)]
pub struct Create {
    pub name: String,
    pub blank: bool
}

#[derive(Debug, Clone)]
pub struct AddNode {
    pub name: String
}

#[derive(Debug, Clone)]
pub struct AddWidget {
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Extract {
    pub origin_path: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Opb {
    #[serde(rename = "pluginName")]
    pub plugin_name: String,
    #[serde(rename = "pluginID")]
    pub plugin_id: String,
    #[serde(rename = "pluginDescription")]
    pub plugin_description: String,
    #[serde(rename = "pluginVersion")]
    pub plugin_version: String,
    #[serde(rename = "pluginAuthor")]
    pub plugin_author: String,
    #[serde(rename = "pluginCategoryLabel")]
    pub plugin_category_label: String,
    pub widgets: Vec<FullWidget>,
    pub nodes: Vec<FullNode>,
    pub icon: IconFileContents
}

impl Opb {
    pub fn bundle() -> Result<Self> {

        if !inside_plugin() {
            bail!("You are not currently editing a plugin! Use opc create to create a new plugin, then run this command from the plugin folder.")
        }

        let settings: sample_files::PluginJson = serde_json::from_str(
            &fs::read_to_string("plugin.json").expect("Error reading plugin.json")
        ).expect("Error deserializing plugin.json");

        let mut widgets: Vec<FullWidget> = Vec::new();
        let mut nodes: Vec<FullNode> = Vec::new();

        for widget in settings.widgets {
            widgets.push(FullWidget {
                widget_name: widget.widget_name,
                widget_id: widget.widget_id.clone(),
                prototype: widget.prototype,
                file_contents: WidgetFiles {
                    html: read_to_string(widget.widget_id.clone() + "/" + &widget.widget_id + ".html"),
                    css: read_to_string(widget.widget_id.clone() + "/" + &widget.widget_id + ".css"),
                    js: read_to_string(widget.widget_id.clone() + "/" + &widget.widget_id + ".js"),
                    svg: read_to_string(widget.widget_id.clone() + "/" + &widget.widget_id + ".svg")
                }
            });
        }

        for node in settings.nodes {
            nodes.push(FullNode {
                node_name: node.node_name,
                node_id: node.node_id.clone(),
                js: NodeFiles { js: read_to_string(node.node_id.clone() + ".js") }
            });
        }

        Ok(Opb {
            plugin_name: settings.plugin_name,
            plugin_id: settings.plugin_id,
            plugin_description: settings.plugin_description,
            plugin_version: settings.plugin_version,
            plugin_author: settings.plugin_author,
            plugin_category_label: settings.plugin_category_label,
            widgets,
            nodes,
            icon: IconFileContents { svg: read_to_string("icon.svg".to_string()) }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconFileContents {
    pub svg: String
}

pub fn read_to_string(path: String) -> String {

    fs::read_to_string(&path).expect(&format!("Error reading file {}", path))
}

pub fn inside_plugin() -> bool {

    Path::new("plugin.json").exists()
}