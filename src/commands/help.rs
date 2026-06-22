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
        flags: &[
            ("--clean", "Limpia el cache antes de actualizar"),
        ],
    },
    BuiltinHelp {
        name: "help",
        description: "Muestra este mensaje",
        flags: &[],
    },
    BuiltinHelp {
        name: "clear-cache",
        description: "Borra el cache de plugins y del core",
        flags: &[
            ("--yes", "Confirma el borrado sin pedir confirmacion"),
        ],
    },
];

struct Row {
    indent: String,
    is_last: bool,
    name: String,
    description: String,
}

impl Row {
    // Columnas de pantalla que ocupa "indent + prefix + espacio + name"
    fn name_end_col(&self) -> usize {
        self.indent.chars().count() + 4 + self.name.chars().count()
    }
}

pub fn run(plugins: &[PluginConf], map: &HashMap<String, Rc<dyn BasaltoPlugin>>) {
    /* Resumen de run(plugins, map)
     * Recolecta todas las filas con sus metadatos
     * Calcula el ancho maximo para alinear todas las descripciones a la misma columna
     * Imprime el arbol con alineacion uniforme
     */

    let active: Vec<&PluginConf> = plugins.iter().filter(|p| p.enabled).collect();

    // --- Primera pasada: recolectar filas ---
    let mut rows: Vec<Row> = Vec::new();

    for (i, cmd) in BUILTINS.iter().enumerate() {
        let is_last = i + 1 == BUILTINS.len() && active.is_empty();
        rows.push(Row {
            indent: String::new(),
            is_last,
            name: cmd.name.to_string(),
            description: cmd.description.to_string(),
        });
        let cont = if is_last { "    " } else { "│   " };
        for (fi, (fname, fdesc)) in cmd.flags.iter().enumerate() {
            rows.push(Row {
                indent: cont.to_string(),
                is_last: fi + 1 == cmd.flags.len(),
                name: fname.to_string(),
                description: fdesc.to_string(),
            });
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
        let cont = if is_last_plugin { "    " } else { "│   " };

        let instance = map.values().find(|p| {
            let so_name = name.replace('-', "_");
            p.name() == so_name || p.name() == name
        });

        let Some(plugin) = instance else { continue };
        let commands = plugin.command_help();

        for (ci, cmd) in commands.iter().enumerate() {
            let is_last_cmd = ci + 1 == commands.len();
            rows.push(Row {
                indent: cont.to_string(),
                is_last: is_last_cmd,
                name: cmd.name.to_string(),
                description: cmd.description.to_string(),
            });
            let cmd_cont = if is_last_cmd {
                format!("{}    ", cont)
            } else {
                format!("{}│   ", cont)
            };
            for (fi, flag) in cmd.flags.iter().enumerate() {
                rows.push(Row {
                    indent: cmd_cont.clone(),
                    is_last: fi + 1 == cmd.flags.len(),
                    name: flag.name.to_string(),
                    description: flag.description.to_string(),
                });
            }
        }
    }

    // --- Calcular columna de alineacion ---
    let align_col = rows.iter().map(|r| r.name_end_col()).max().unwrap_or(20) + 2;

    // --- Segunda pasada: imprimir ---
    println!("basalto v{}", env!("CARGO_PKG_VERSION"));

    // Indice de la primera fila de cada plugin para saber donde insertar separadores y headers
    let mut plugin_starts: Vec<(usize, String, bool)> = Vec::new(); // (row_idx, name, is_last)
    {
        let mut row_idx = BUILTINS.iter().map(|b| 1 + b.flags.len()).sum::<usize>();
        for (pi, plugin_conf) in active.iter().enumerate() {
            let name = plugin_conf
                .source
                .split('/')
                .next_back()
                .unwrap()
                .trim_end_matches(".git")
                .to_string();
            let is_last_plugin = pi + 1 == active.len();
            plugin_starts.push((row_idx, name.clone(), is_last_plugin));

            let instance = map.values().find(|p| {
                let so = name.replace('-', "_");
                p.name() == so || p.name() == name.as_str()
            });
            if let Some(plugin) = instance {
                let commands = plugin.command_help();
                for cmd in commands {
                    row_idx += 1 + cmd.flags.len();
                }
            }
        }
    }

    let mut plugin_start_iter = plugin_starts.iter().peekable();
    let mut current_row = 0;

    for row in &rows {
        // Insertar separador y header de plugin cuando corresponde
        if let Some((start, pname, is_last)) = plugin_start_iter.peek() {
            if current_row == *start {
                // Linea separadora con continuacion del arbol
                let sep_cont = if *is_last { "│" } else { "│" };
                println!("{}", sep_cont);
                let prefix = if *is_last { "└──" } else { "├──" };
                println!("{} {}", prefix, pname);
                plugin_start_iter.next();
            }
        }

        let prefix = if row.is_last { "└──" } else { "├──" };
        let used = row.name_end_col();
        let padding = if align_col > used { align_col - used } else { 1 };
        println!(
            "{}{} {}{}{}",
            row.indent,
            prefix,
            row.name,
            " ".repeat(padding),
            row.description
        );
        current_row += 1;
    }
}
