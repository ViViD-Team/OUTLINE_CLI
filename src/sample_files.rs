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
                this.mainButton.textContent = `Clicked ${this._widgetData.params.count} times.`;
            }
        }


        // Get objects by querying inside of _main
        this.mainButton = this._main.getElementsByClassName("mainButton")[0];


        // Functions need to be defined outside of addEventListener.
        // Else "this" will not be the object, but the DOM node.
        let callback = () => {
            this._widgetData.params.count++;
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