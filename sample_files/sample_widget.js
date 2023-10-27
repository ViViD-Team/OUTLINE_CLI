class sampleWidget {
    constructor(_main, _projectData, _widgetData) {
        this._main = _main;
        this._projectData = _projectData;
        this._widgetData = _widgetData;

        this.update = function() {
            // Makes the component react to outside changes.
            // Update all displays here.

            if (this._widgetData.params.count !== 0) {
                this.mainButton.textContent = `Clicked ${this._widgetData.params.count} times.`;
            }

            // Resync widgetData.
            push();
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

module.exports = sampleWidget;