const NodeData = require("./../.dependencies/NodeData");
const NodeInputTether = require("./../.dependencies/NodeInputTether");
const NodeOutputTether = require("./../.dependencies/NodeOutputTether");

/**
 * Container for all data associated with your
 * custom node.
 */
class SampleNodeData extends NodeData {

    constructor(outputRefs, context, rawNodeData) {
        let inputs = [
        ];
        let outputs = [
            new SampleNodeDataOutput("Sample", inputs, outputRefs[0], context),
        ];

        super("Sample", inputs, outputs, rawNodeData);
    }

}

/**
 * NodeOutputTether containing custom behavior for
 * resolving the Promise emitted by the process() function.
 */
class SampleNodeDataOutput extends NodeOutputTether {

    constructor(reqInputs, puts, id, context) {
        super(reqInputs, puts, id, context);

        this.process = function() {
            return new Promise(async (resolve, reject) => {
                resolve(0);
            });
        }
    }

}

module.exports = SampleNodeData;