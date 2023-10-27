use opc_macros::prov_sample_fn;

prov_sample_fn!(get_sample_widget_css() "sample_files/sample_widget.css");
prov_sample_fn!(get_sample_widget_js() "sample_files/sample_widget.js");
prov_sample_fn!(get_widget_js(name: &str) "sample_files/widget.js" __CLASSNAME__ -> name);
prov_sample_fn!(get_sample_widget_svg() "sample_files/sample_widget.svg");
prov_sample_fn!(get_sample_widget_html() "sample_files/sample_widget.html");
prov_sample_fn!(get_sample_icon_svg() "sample_files/sample_icon.svg");
prov_sample_fn!(get_sample_node_js(name: &str) "sample_files/sample_node.js" __CLASSNAME__ -> name);