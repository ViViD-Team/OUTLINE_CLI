# CLI for creating OUTLINE plugins

## Commands:

### opc
Outputs "OUTLINE Plugin Creator *version* installed".

### opc create *pluginName*
Creates the basic filetree for an outline plugin (see ./basic) in a new subdirectory with the provided name.

The metadata contained within **plugin.json** is to be changed accordingly.

- The provided name is to be treated as the plugin ID (usually lowerCamelCase)
- The name is the same, but words are split and capitalized
- Remaining fields are to be set as seen in *./basic/plugin.json*

Outputs "Plugin *pluginName* created at *fileLocation*".

#### Possible errors
 - Directory with the same name already exists
    - Output "Directory with the same name already exists!"


### opc create *pluginName* -blank
Creates a new plugin without any sample content.

Outputs "Plugin *pluginName* created at *fileLocation*".

#### Possible errors
 - Directory with the same name already exists
    - Output "Directory with the same name already exists!"


### opc add widget *widgetName*
Adds another widget to the plugin. The provided name is to be treated as the ID.

- Generates a subdir named accordingly
- The new subdir contains *ID*.html, *ID*.css, *ID*.js and *ID*.svg.
- The new widget is pushed to plugin.json > widgets
    - The ID is set to the provided ID
    - The Name is set to the ID but wordsplit and capitalized

The newly created widget has no content. Unlike the sampleWidget, it is completely empty. The only file with contents is the .js file, containing the class declaration (class named accordingly), constructor and update method declaration holding only the comment.

Outputs "Generated *widgetName* widget. Make sure to customize the plugin.json".

#### Possible errors
- Not currently inside a plugin
    - Output "You are not currently editing a plugin!"
- Widget with the same ID already exists
    - Output "Widget with the same ID already exists in this plugin!"


### opc add node *nodeName*
Adds another node to the plugin. The provided name is to be treated as the ID.

- Generates *nodeName*.js
- The new widget is pushed to plugin.json > nodes
    - The ID is set to the provided ID
    - The Name is set to the ID but wordsplit and capitalized

Outputs "Generated *nodeName* node. Make sure to customize the plugin.json".

#### Possible errors
- Not currently inside a plugin
    - Output "You are not currently editing a plugin!"
- Node with the same ID already exists
    - Output "Node with the same ID already exists in this plugin!"


### opc bundle
Bundles the plugin to *./pluginID.opb*.

The file stores all contents bundled in a json format:
```json
{
    "pluginName": "Plugin Name",
    "pluginID": "pluginName",
    "pluginDescription": "Plugin Description",
    "pluginVersion": "1.0.0",
    "pluginAuthor": "Plugin Author",
    
    "pluginCategoryLabel": "Category Label",
    
    "widgets": [
        {
            "widgetName": "Sample Widget",
            "widgetID": "sampleWidget",
            "prototype": {
                "posX": 0,
                "posY": 0,
                "sizeX": 8,
                "sizeY": 8,
                "simX": 0,
                "simY": 0,
                "simResizeX": 0,
                "simResizeY": 0,
                "sizeBounds": [],

                // Specific for sampleWidget
                "count": 0
            },
            "fileContents": {
                "html": "HTML_FILE_CONTENTS",
                "css": "CSS_FILE_CONTENTS",
                "js": "JS_FILE_CONTENTS",
                "SVG": "SVG_FILE_CONTENTS"
            }
        }
        ...
    ],

    "nodes": [
        {
            "nodeName": "Test Node",
            "nodeID": "Test",
            "fileContents": "JS_FILE_CONTENTS"
        }
        ...
    ]
}
```

#### Possible errors
- Not currently inside a plugin
    - Output "You are not currently editing a plugin!"
- Files incomplete
    - Output "Could not bundle: Files missing!"
    - List missing (expected) files
- plugin.json contains syntax errors
    - Output "Could not bundle: plugin.json invalid!"
- plugin.json incomplete
    - Output "Could not bundle: plugin.json invalid!"
    - List expected fields


### opc extract *relativeFilepath*
Extracts a .opb file back to the filetree into *./ID/*.

#### Possible errors
- Subdir with the same name already exists
    - Output "Subdir with the same name already exists!"