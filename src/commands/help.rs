use basalto_shared::BasaltoPlugin;
use crate::plugins::PluginConf;
use std::collections::HashMap;
use std::rc::Rc;

struct BuiltinHelp {
    name: &'static str,
    description: &'static str,
    flags: &'static [(&'static str, &'static str)],
}

const BUILTINS: &[BuiltinHelp] = &[
    BuiltinHelp {
        name: "version",
        description: "Muestra la version del core y los plugins",
        flags: &[
            ("--active", "Solo plugins activos"),
            ("--inactive", "Solo plugins inactivos"),
            ("--check", "Verifica actualizaciones disponibles"),
        ],
    },
    BuiltinHelp {
        name: "update",
        description: "Actualiza todos los plugins y el core",
        flags: &[],
    },
    BuiltinHelp {
        name: "help",
        description: "Muestra este mensaje",
        flags: &[],
    },
];

pub fn run(plugins: &[PluginConf], map: &HashMap<String, Rc<dyn BasaltoPlugin>>) {
    /* Resumen de run(plugins, map)
     * Imprime la version del core
     * Muestra los comandos built-in con sus flags en formato arbol
     * Para cada plugin activo busca su instancia en el map y muestra command_help()
     */

    let active_plugins: Vec<&PluginConf> = plugins.iter().filter(|p| p.enabled).collect();

    println!("basalto v{}\n", env!("CARGO_PKG_VERSION"));

    let total_builtins = BUILTINS.len();
    for (i, cmd) in BUILTINS.iter().enumerate() {
        let is_last = i + 1 == total_builtins && active_plugins.is_empty();
        let prefix = if is_last { "└──" } else { "├──" };
        println!("{} {:<18} {}", prefix, cmd.name, cmd.description);
        print_flags(cmd.flags.iter().copied(), is_last);
    }

    for plugin_conf in &active_plugins {
        let name = plugin_conf
            .source
            .split('/')
            .next_back()
            .unwrap()
            .trim_end_matches(".git");

        println!("\n{}:", name);

        // Obtiene la instancia ya cargada buscando cualquier comando del plugin en el map
        let instance = map.values().find(|p| {
            let so_name = name.replace('-', "_");
            p.name() == so_name || p.name() == name
        });

        let Some(plugin) = instance else { continue };

        let commands = plugin.command_help();
        let total = commands.len();

        for (ci, cmd) in commands.iter().enumerate() {
            let is_last = ci + 1 == total;
            let prefix = if is_last { "└──" } else { "├──" };
            println!("{} {:<18} {}", prefix, cmd.name, cmd.description);
            print_flags(cmd.flags.iter().map(|f| (f.name, f.description)), is_last);
        }
    }
}

fn print_flags<'a>(flags: impl Iterator<Item = (&'a str, &'a str)>, parent_is_last: bool) {
    let flags: Vec<_> = flags.collect();
    let indent = if parent_is_last { "    " } else { "│   " };

    for (i, (name, desc)) in flags.iter().enumerate() {
        let is_last = i + 1 == flags.len();
        let prefix = if is_last { "└──" } else { "├──" };
        println!("{}{}  {:<14} {}", indent, prefix, name, desc);
    }
}
