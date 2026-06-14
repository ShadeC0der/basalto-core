use crate::installer;
use crate::plugins::PluginConf;
use basalto_shared::BasaltoPlugin;
use std::collections::HashMap;
use std::rc::Rc;

pub fn build(
    plugins: &[PluginConf],
) -> (
    HashMap<String, Rc<dyn BasaltoPlugin>>,
    Vec<libloading::Library>,
) {
    let mut map = HashMap::new();
    let mut libs: Vec<libloading::Library> = Vec::new();
    let home = std::env::var("HOME").unwrap();

    for input in plugins {
        let name = input
            .source
            .split('/')
            .next_back()
            .unwrap()
            .trim_end_matches(".git");

        let path = format!(
            "{}/.basalto/cache/plugins/{}/target/release/lib{}.so",
            home, name, name
        );

        installer::ensure(name, &input.source, &path, &input.branch);
        let lib = unsafe { libloading::Library::new(&path).unwrap() };

        let plugin: Rc<dyn BasaltoPlugin> = {
            let constructor: libloading::Symbol<fn() -> *mut dyn BasaltoPlugin> =
                unsafe { lib.get(b"_basalto_create_plugin").unwrap() };
            Rc::from(unsafe { Box::from_raw(constructor()) })
        };

        for command in plugin.plugin_commands() {
            map.insert(command.to_string(), Rc::clone(&plugin));
        }

        libs.push(lib);
    }

    (map, libs)
}
