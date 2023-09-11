use serde;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;


pub fn get_sample_widget_css() -> String {
".centeredLayout {
    width: 100%;
    height: 100%;

    display: grid;
    place-items: center;
}

button {
    width: calc(5 * var(--unit));
    height: calc(1 * var(--unit));

    font-size: calc(.5 * var(--unit));;
}".to_string()
}

pub fn get_sample_widget_js() -> String {
r#"class sampleWidget {
    constructor(_main, _projectData, _widgetData) {
        this._main = _main;
        this._projectData = _projectData;
        this._widgetData = _widgetData;

        this.update = function() {
            // Called every time _widgetData changes
            // Use this to set all displays, eg. text elements

            if (this._widgetData.count !== 0) {
                this.mainButton.textContent = `Clicked ${this._widgetData.count} times.`;
            }
        }


        // Get objects by querying inside of _main
        this.mainButton = this._main.getElementsByClassName("mainButton")[0];


        // Functions need to be defined outside of addEventListener.
        // Else "this" will not be the object, but the DOM node.
        let callback = () => {
            this._widgetData.count++;
            this.update();
        }
        this.mainButton.addEventListener("click", callback);
    }
}

module.exports = sampleWidget;"#.to_string()
}

pub fn get_widget_js(name: &str) -> String {

    format!("class {} {{
        constructor(_main, _projectData, _widgetData) {{
            this._main = _main;
            this._projectData = _projectData;
            this._widgetData = _widgetData;
    
            this.update = function() {{
                // Called every time _widgetData changes
                // Use this to set all displays, eg. text elements
            }}
        }}
    }}
    
    module.exports = {};", name, name)
}

pub fn get_sample_widget_svg() -> String {
r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
<!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->
    <path d="M234.5 5.7c13.9-5 29.1-5 43.1 0l192 68.6C495 83.4 512 107.5 512 134.6V377.4c0 27-17 51.2-42.5 60.3l-192 68.6c-13.9 5-29.1 5-43.1 0l-192-68.6C17 428.6 0 404.5 0 377.4V134.6c0-27 17-51.2 42.5-60.3l192-68.6zM256 66L82.3 128 256 190l173.7-62L256 66zm32 368.6l160-57.1v-188L288 246.6v188z"/>
</svg>"#.to_string()
}

pub fn get_sample_widget_html() -> String {
r#"<div class="centeredLayout">
<button class="mainButton">Click Me!</button>
</div>"#.to_string()
}

pub fn get_sample_icon_svg() -> String {
r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
<!--! Font Awesome Free 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->
    <path fill="var(--red)" d="M234.5 5.7c13.9-5 29.1-5 43.1 0l192 68.6C495 83.4 512 107.5 512 134.6V377.4c0 27-17 51.2-42.5 60.3l-192 68.6c-13.9 5-29.1 5-43.1 0l-192-68.6C17 428.6 0 404.5 0 377.4V134.6c0-27 17-51.2 42.5-60.3l192-68.6zM256 66L82.3 128 256 190l173.7-62L256 66zm32 368.6l160-57.1v-188L288 246.6v188z"/>
</svg>"#.to_string()
}

pub fn get_sample_node_js(name: Option<String>) -> String {
    let name = name.unwrap_or_default();
format!("const NodeData = require(\"./../.dependencies/NodeData\");
const NodeInputTether = require(\"./../.dependencies/NodeInputTether\");
const NodeOutputTether = require(\"./../.dependencies/NodeOutputTether\");

/**
 * Container for all data associated with your
 * custom node.
 */
class {} extends NodeData {{

    constructor(outputRefs, context, rawNodeData) {{
        let inputs = [
        ];
        let outputs = [
            new {}Output(\"Sample\", inputs, outputRefs[0], context),
        ];

        super(\"Sample\", inputs, outputs, rawNodeData);
    }}

}}

/**
 * NodeOutputTether containing custom behavior for
 * resolving the Promise emitted by the process() function.
 */
class {}Output extends NodeOutputTether {{

    constructor(reqInputs, puts, id, context) {{
        super(reqInputs, puts, id, context);

        this.process = function() {{
            return new Promise(async (resolve, reject) => {{
                resolve(0);
            }});
        }}
    }}

}}

module.exports = {};", &name, &name, &name, &name)
}

pub fn get_plugin_json() -> PluginJson {

    PluginJson::new()
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

impl PluginJson {
    pub fn new() -> PluginJson {
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