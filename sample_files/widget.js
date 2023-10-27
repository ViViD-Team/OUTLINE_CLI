class __CLASSNAME__ {
    constructor(_main, _projectData, _widgetData) {
        this._main = _main;
        this._projectData = _projectData;
        this._widgetData = _widgetData;

        this.update = function() {
            // Makes the component react to outside changes.
            // Update all displays here.

            // Resync widgetData.
            push();
        }
    }
}

module.exports = __CLASSNAME__;