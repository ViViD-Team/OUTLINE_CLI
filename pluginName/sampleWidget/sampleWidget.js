class sampleWidget {
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

module.exports = sampleWidget;