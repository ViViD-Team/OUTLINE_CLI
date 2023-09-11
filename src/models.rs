use anyhow::{Result, anyhow, bail};
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

use crate::{inside_plugin, read_to_string};

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
    pub fn new(args: &mut std::env::Args) -> Result<RawCommand> {

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

        match RawCommand::new(&mut args)? {
            RawCommand::Create => Command::build_create(&mut args),
            RawCommand::Add => Command::build_add(&mut args),
            RawCommand::Bundle => Ok(Command::Bundle),
            RawCommand::Extract => Command::build_extract(&mut args),
            RawCommand::Version => Ok(Command::Version),
            RawCommand::Help => Ok(Command::Help(RawCommand::new(&mut args)?)),
        }
    }

    pub fn build_create(args: &mut std::env::Args) -> Result<Command> {

        let c = Command::Create(Create {
            name: args.next().unwrap_or("".to_string()),
            blank: {
                if let Some(option) = args.next() {
                    if option == "-blank" {true}
                    else {bail!("invalid option")}
                } else {false}
            }
        });
        if let Command::Create(b) = c.clone() {
            if b.name.is_empty() {bail!("missing plugin name")}
        }
        Ok(c)
    }

    pub fn build_add(args: &mut std::env::Args) -> Result<Command> {

        if let Some(elem_type) = args.next() {
            if let Some(name) = args.next() {
                match elem_type.as_str() {
                    "widget" => {
                        Ok(Command::AddWidget(AddWidget { name }))
                    }
                    "node" => {
                        Ok(Command::AddNode(AddNode { name }))
                    }
                    _ => {bail!("invalid element type")}
                }
            } else {
                bail!("missing element name");
            }
        } else {
            bail!("missing element type");
        }
    }

    pub fn build_extract(args: &mut std::env::Args) -> Result<Command> {

        if let Some(path) = args.next() {
            return Ok(Command::Extract(Extract { origin_path: path }))
        }
        bail!("missing path of .opb")
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

        let settings: PluginJson = serde_json::from_str(
            &read_to_string("plugin.json".to_string())
        ).expect("Error deserializing plugin.json");

        let mut widgets: Vec<FullWidget> = Vec::new();
        let mut nodes: Vec<FullNode> = Vec::new();

        
        for widget in settings.widgets {
            let path = widget.widget_id.clone() + "/" + &widget.widget_id;
            widgets.push(FullWidget {
                widget_name: widget.widget_name,
                widget_id: widget.widget_id.clone(),
                prototype: widget.prototype,
                file_contents: WidgetFiles {
                    html: read_to_string(path.clone() + ".html"),
                    css: read_to_string(path.clone() + ".css"),
                    js: read_to_string(path.clone() + ".js"),
                    svg: read_to_string(path + ".svg")
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginJson {
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
    pub widgets: Vec<Widget>,
    pub nodes: Vec<Node>,
}

impl Default for PluginJson {
    fn default() -> Self {
        PluginJson {
            plugin_name: "Plugin Name".to_string(),
            plugin_id: "Plugin ID".to_string(),
            plugin_description: "Plugin Description".to_string(),
            plugin_version: "1.0.0".to_string(),
            plugin_author: "Plugin Author".to_string(),
            plugin_category_label: "Category Label".to_string(),
            widgets: vec![Widget {
                widget_name: "Sample Widget".to_string(),
                widget_id: "sampleWidget".to_string(),
                prototype: Prototype::default()
            }],
            nodes: vec![Node {
                node_name: "Test Node".to_string(),
                node_id: "Test".to_string()
            }]
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Widget {
    #[serde(rename = "widgetName")]
    pub widget_name: String,
    #[serde(rename = "widgetID")]
    pub widget_id: String,
    pub prototype: Prototype
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullWidget {
    #[serde(rename = "widgetName")]
    pub widget_name: String,
    #[serde(rename = "widgetID")]
    pub widget_id: String,
    pub prototype: Prototype,
    #[serde(rename = "fileContents")]
    pub file_contents: WidgetFiles
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WidgetFiles {
    pub html: String,
    pub css: String,
    pub js: String,
    pub svg: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prototype {
    #[serde(rename = "posX")]
    pub pos_x: f64,
    #[serde(rename = "posY")]
    pub pos_y: f64,
    #[serde(rename = "sizeX")]
    pub size_x: f64,
    #[serde(rename = "sizeY")]
    pub size_y: f64,
    #[serde(rename = "simX")]
    pub sim_x: f64,
    #[serde(rename = "simY")]
    pub sim_y: f64,
    #[serde(rename = "simResizeX")]
    pub sim_resize_x: f64,
    #[serde(rename = "simResizeY")]
    pub sim_resize_y: f64,
    #[serde(rename = "sizeBounds")]
    pub size_bounds: Vec<f64>,
    pub params: Value,
}

impl Default for Prototype {
    fn default() -> Self {
        Prototype {
            pos_x: 0.,
            pos_y: 0.,
            size_x: 8.,
            size_y: 8.,
            sim_x: 0.,
            sim_y: 0.,
            sim_resize_x: 0.,
            sim_resize_y: 0.,
            size_bounds: Vec::new(),
            params: Value::Object(Map::new())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "nodeID")]
    pub node_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullNode {
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "nodeID")]
    pub node_id: String,
    #[serde(rename = "fileContents")]
    pub js: NodeFiles
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeFiles {
    pub js: String,
}