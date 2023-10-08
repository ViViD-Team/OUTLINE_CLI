use anyhow::{Result, anyhow, bail};
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

use crate::{inside_plugin, read_to_string, get_settings, set_settings};

#[derive(Debug, Clone)]
pub enum RawCommand {
    Version,
    Help,
    Create,
    Add,
    Bundle,
    Extract,
    Remove
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
            "remove" => Ok(RawCommand::Remove),
            _ => Err(anyhow!("invalid command"))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    Node(String),
    Widget(String)
}


#[derive(Debug, Clone)]
pub enum Command {
    Version,
    Help(RawCommand),
    Create(Create),
    Add(Element),
    Bundle,
    Extract(Extract),
    Remove(Element),
}

macro_rules! build_element_with_name {
    ($cmd:ident, $args:ident, $($elem_type:ident),+) => {
        if let Some(elem_type) = $args.next() {
            if let Some(name) = $args.next() {
                $(
                    if elem_type == stringify!($elem_type).to_ascii_lowercase() {
                        return Ok(Command::$cmd(Element::$elem_type(name)))
                    }
                )*
                bail!("invalid element type")
            } else {
                bail!("missing argument");
            }
        } else {
            bail!("missing argument");
        }
    }
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
            RawCommand::Remove => Command::build_remove(&mut args),
        }
    }

    pub fn build_remove(args: &mut std::env::Args) -> Result<Command> {

        inside_plugin!(bail);

        build_element_with_name!(Remove, args, Widget, Node);
    } 

    pub fn build_create(args: &mut std::env::Args) -> Result<Command> {

        let name = args.next().ok_or(anyhow!("missing argument"))?;
        let blank = args.next().unwrap_or_default() == "-blank";

        let c = Command::Create(Create { name, blank });

        Ok(c)
    }

    pub fn build_add(args: &mut std::env::Args) -> Result<Command> {

        build_element_with_name!(Add, args, Widget, Node)
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
#[serde(rename_all = "camelCase")]
pub struct Opb {
    pub plugin_name: String,
    #[serde(rename = "pluginID")]
    pub plugin_id: String,
    pub plugin_description: String,
    pub plugin_version: String,
    pub plugin_author: String,
    pub plugin_category_label: String,
    pub widgets: Vec<FullWidget>,
    pub nodes: Vec<FullNode>,
    pub icon: IconFileContents
}

impl Opb {
    pub fn bundle() -> Result<Self> {

        inside_plugin!(bail);

        let settings = get_settings();

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
#[serde(rename_all = "camelCase")]
pub struct PluginJson {
    pub plugin_name: String,
    #[serde(rename = "pluginID")]
    pub plugin_id: String,
    pub plugin_description: String,
    pub plugin_version: String,
    pub plugin_author: String,
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

impl PluginJson {
    pub fn remove_node(&mut self, id: String) -> Result<()> {

        let ind = self.nodes.iter().position(|node| node.node_id == id).ok_or(anyhow!("Node not found"))?;
        self.nodes.remove(ind);
        set_settings(self);
        Ok(())
    }
    pub fn remove_widget(&mut self, id: String) -> Result<()> {

        let ind = self.widgets.iter().position(|widget| widget.widget_id == id).ok_or(anyhow!("Widget not found"))?;
        self.widgets.remove(ind);
        set_settings(self);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Widget {
    pub widget_name: String,
    #[serde(rename = "widgetID")]
    pub widget_id: String,
    pub prototype: Prototype
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullWidget {
    pub widget_name: String,
    #[serde(rename = "widgetID")]
    pub widget_id: String,
    pub prototype: Prototype,
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
#[serde(rename_all = "camelCase")]
pub struct Prototype {
    pub pos_x: f64,
    pub pos_y: f64,
    pub size_x: f64,
    pub size_y: f64,
    pub sim_x: f64,
    pub sim_y: f64,
    pub sim_resize_x: f64,
    pub sim_resize_y: f64,
    pub size_bounds: Vec<Vec<f64>>,
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
            size_bounds: vec![vec![3., 30.], vec![3., 30.]],
            params: Value::Object(Map::new())
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub node_name: String,
    #[serde(rename = "nodeID")]
    pub node_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullNode {
    pub node_name: String,
    #[serde(rename = "nodeID")]
    pub node_id: String,
    pub js: NodeFiles
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeFiles {
    pub js: String,
}