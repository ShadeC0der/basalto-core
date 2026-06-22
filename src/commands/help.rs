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
            ("--active",   "Solo plugins activos"),
            ("--inactive", "Solo plugins inactivos"),
            ("--check",    "Verifica actualizaciones disponibles"),
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
     * Imprime un unico arbol con el core como raiz
     * Los built-ins y plugins son ramas del mismo nivel
     * Los comandos del plugin y sus flags son subramas
     */

    let active: Vec<&PluginConf> = plugins.iter().filter(|p| p.enabled).collect();

    println!("basalto v{}", env!("CARGO_PKG_VERSION"));

    for (i, cmd) in BUILTINS.iter().enumerate() {
        let is_last = i + 1 == BUILTINS.len() && active.is_empty();
        print_row("", is_last, cmd.name, cmd.description);
        let cont = if is_last { "    " } else { "│   " };
        for (fi, (fname, fdesc)) in cmd.flags.iter().enumerate() {
            let flag_last = fi + 1 == cmd.flags.len();
            print_row(cont, flag_last, fname, fdesc);
        }
    }

    for (pi, plugin_conf) in active.iter().enumerate() {
        let name = plugin_conf
            .source
            .split('/')
            .next_back()
            .unwrap()
            .trim_end_matches(".git");

        let is_last_plugin = pi + 1 == active.len();
        let plugin_prefix = if is_last_plugin { "└──" } else { "├──" };
        println!("{} {}", plugin_prefix, name);

        let cont = if is_last_plugin { "    " } else { "│   " };

        let instance = map.values().find(|p| {
            let so_name = name.replace('-', "_");
            p.name() == so_name || p.name() == name
        });

        let Some(plugin) = instance else { continue };
        let commands = plugin.command_help();

        for (ci, cmd) in commands.iter().enumerate() {
            let is_last_cmd = ci + 1 == commands.len();
            print_row(cont, is_last_cmd, cmd.name, cmd.description);
            let cmd_cont = if is_last_cmd {
                format!("{}    ", cont)
            } else {
                format!("{}│   ", cont)
            };
            for (fi, flag) in cmd.flags.iter().enumerate() {
                let flag_last = fi + 1 == cmd.flags.len();
                print_row(&cmd_cont, flag_last, flag.name, flag.description);
            }
        }
    }
}

fn print_row(indent: &str, is_last: bool, name: &str, desc: &str) {
    let prefix = if is_last { "└──" } else { "├──" };
    println!("{}{} {:<18} {}", indent, prefix, name, desc);
}
