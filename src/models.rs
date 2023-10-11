use anyhow::{Result, anyhow, bail};
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

use crate::{inside_plugin, read_to_string, get_settings, set_settings};

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