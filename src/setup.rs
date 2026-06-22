struct Section {
    header: &'static str,
    default: &'static str,
}

const REQUIRED: &[Section] = &[
    Section {
        header: "[library]",
        default: "[library]\nurl = \"\"\nbranch = \"main\"\n",
    },
    Section {
        header: "[editors]",
        default: "[editors]\navailable = [\"nvim\"]\n",
    },
    Section {
        header: "[core]",
        default: "[core]\nsource = \"git@github.com:ShadeC0der/basalto-core.git\"\n",
    },
];

pub fn run() {
    /* Resumen de run()
     * Obtiene la ruta al HOME
     * Crea la estructura de carpetas necesaria si no existe
     * Crea config.toml si no existe
     * Verifica que cada sección requerida esté presente
     * Si falta alguna sección la agrega automáticamente con sus valores por defecto
     */

    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();

    let basalto_dir = format!("{}/.basalto", home);

    if !std::path::Path::new(&basalto_dir).exists() {
        std::fs::create_dir_all(format!("{}/.basalto/plugins", home)).unwrap();
        std::fs::create_dir_all(format!("{}/.basalto/cache/plugins", home)).unwrap();
        std::fs::create_dir_all(format!("{}/.basalto/cache/library", home)).unwrap();
        println!("Inicializando carpetas en ~/.basalto/");
    }

    let config_path = format!("{}/.basalto/config.toml", home);
    if !std::path::Path::new(&config_path).exists() {
        std::fs::write(&config_path, "").unwrap();
    }

    let mut content = std::fs::read_to_string(&config_path).unwrap();

    for section in REQUIRED {
        if !content.contains(section.header) {
            content.push_str(&format!("\n{}\n", section.default));
            std::fs::write(&config_path, &content).unwrap();
        }
    }
}

