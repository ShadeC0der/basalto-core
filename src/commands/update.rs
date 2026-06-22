use crate::plugins;
use crate::config;
use crate::commands::clear_cache;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn run(args: &[&str]) {
    /* Resumen de run(args)
     * Con --clean limpia el cache antes de actualizar (reinstalacion completa)
     * Muestra un spinner por cada plugin y por el core
     * Si un plugin no esta en cache lo clona en lugar de saltarlo
     * Silencia el output de git y cargo para mostrar solo el resumen
     */

    if args.contains(&"--clean") {
        clear_cache::run(&["--yes"]);
        println!();
    }

    let plugins = plugins::read_plugins();
    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap().to_string();

    for plugin in &plugins {
        let name = plugin
            .source
            .split('/')
            .next_back()
            .unwrap()
            .trim_end_matches(".git");

        let plugin_dir = format!("{}/.basalto/cache/plugins/{}", home, name);

        let pb = spinner();

        if !std::path::Path::new(&plugin_dir).exists() {
            pb.set_message(format!("{}  clonando...", name));
            let ok = correr_silencioso("git", &["clone", &plugin.source, &plugin_dir], &home);
            if !ok {
                pb.finish_with_message(format!("{} {}  error al clonar", style("✗").red(), name));
                continue;
            }
        } else {
            pb.set_message(format!("{}  buscando actualizacion...", name));
            correr_silencioso("git", &["fetch"], &plugin_dir);
            correr_silencioso("git", &["reset", "--hard", "origin/HEAD"], &plugin_dir);
        }

        let version_antes = leer_version(&format!("{}/Cargo.toml", plugin_dir));

        // Releer version despues del reset para detectar cambio
        correr_silencioso("git", &["fetch"], &plugin_dir);
        correr_silencioso("git", &["reset", "--hard", "origin/HEAD"], &plugin_dir);
        let version_despues = leer_version(&format!("{}/Cargo.toml", plugin_dir));

        let hay_cambio = version_antes != version_despues;

        pb.set_message(format!("{}  compilando...", name));
        let ok = correr_silencioso("cargo", &["build", "--release"], &plugin_dir);

        if !ok {
            pb.finish_with_message(format!("{} {}  error al compilar", style("✗").red(), name));
            continue;
        }

        if hay_cambio {
            pb.finish_with_message(format!(
                "{} {}  {} {} {}",
                style("✓").green(),
                name,
                style(format!("v{}", version_antes.as_deref().unwrap_or("?"))).cyan(),
                style("→").yellow(),
                style(format!("v{}", version_despues.as_deref().unwrap_or("?"))).cyan(),
            ));
        } else {
            pb.finish_with_message(format!(
                "{} {}  {}",
                style("✓").green(),
                name,
                style(format!("v{}", version_despues.as_deref().unwrap_or("?"))).cyan(),
            ));
        }
    }

    actualizar_bibliotecas(&home);
    actualizar_core(&home);
    actualizar_tui(&home);
}

fn actualizar_bibliotecas(home: &str) {
    let conf = config::read_config();
    if conf.libraries.list.is_empty() { return; }

    let libs_dir = format!("{}/.basalto/cache/libraries", home);
    std::fs::create_dir_all(&libs_dir).ok();

    for lib in &conf.libraries.list {
        let lib_dir = format!("{}/{}", libs_dir, lib.name);
        let pb = spinner();
        pb.set_message(format!("biblioteca:{}  buscando actualizacion...", lib.name));

        match &lib.source {
            Some(src) if !std::path::Path::new(&lib_dir).exists() => {
                pb.set_message(format!("biblioteca:{}  clonando...", lib.name));
                let ok = correr_silencioso("git", &["clone", src, &lib_dir], home);
                if ok {
                    pb.finish_with_message(format!("{} biblioteca:{}  clonada", style("✓").green(), lib.name));
                } else {
                    pb.finish_with_message(format!("{} biblioteca:{}  error al clonar", style("✗").red(), lib.name));
                }
            }
            Some(_) => {
                correr_silencioso("git", &["fetch"], &lib_dir);
                correr_silencioso("git", &["pull", "--ff-only"], &lib_dir);
                pb.finish_with_message(format!("{} biblioteca:{}  actualizada", style("✓").green(), lib.name));
            }
            None => {
                std::fs::create_dir_all(&lib_dir).ok();
                pb.finish_with_message(format!("{} biblioteca:{}  local", style("✓").green(), lib.name));
            }
        }
    }
}

fn actualizar_core(home: &str) {
    /* Resumen de actualizar_core(home)
     * Clona o actualiza el cache del core con un spinner
     * Compara versiones y reinstala si hay cambio
     */

    let core_conf = config::read_core();
    let core_dir = format!("{}/.basalto/cache/core", home);

    let pb = spinner();
    pb.set_message("basalto-core  buscando actualizacion...");

    if !std::path::Path::new(&core_dir).exists() {
        pb.set_message("basalto-core  clonando...");
        correr_silencioso("git", &["clone", &core_conf.source, &core_dir], home);
    } else {
        correr_silencioso("git", &["fetch"], &core_dir);
        correr_silencioso("git", &["reset", "--hard", "origin/HEAD"], &core_dir);
    }

    let cache_version = leer_version(&format!("{}/Cargo.toml", core_dir));
    let instalada = env!("CARGO_PKG_VERSION");

    match cache_version {
        Some(ref nueva) if nueva != instalada => {
            pb.set_message(format!(
                "basalto-core  {} → {}  instalando...",
                instalada, nueva
            ));

            let ok = correr_silencioso("cargo", &["install", "--path", &core_dir], home);

            if ok {
                pb.finish_with_message(format!(
                    "{} basalto-core  {} {} {}",
                    style("✓").green(),
                    style(format!("v{}", instalada)).cyan(),
                    style("→").yellow(),
                    style(format!("v{}", nueva)).cyan(),
                ));
            } else {
                pb.finish_with_message(format!("{} basalto-core  error al instalar", style("✗").red()));
            }
        }
        _ => {
            pb.finish_with_message(format!(
                "{} basalto-core  {}",
                style("✓").green(),
                style(format!("v{}", instalada)).cyan(),
            ));
        }
    }
}

fn spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", ""]),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

fn correr_silencioso(cmd: &str, args: &[&str], dir: &str) -> bool {
    std::process::Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn actualizar_tui(home: &str) {
    let conf = config::read_config();
    let tui_source = match conf.tui {
        Some(c) => c.source,
        None => return,
    };

    let tui_dir = format!("{}/.basalto/cache/tui", home);
    let pb = spinner();
    pb.set_message("basalto-tui  buscando actualizacion...");

    if !std::path::Path::new(&tui_dir).exists() {
        pb.set_message("basalto-tui  clonando...");
        correr_silencioso("git", &["clone", &tui_source, &tui_dir], home);
    } else {
        correr_silencioso("git", &["fetch"], &tui_dir);
    }

    let version_antes = leer_version(&format!("{}/Cargo.toml", tui_dir));
    correr_silencioso("git", &["reset", "--hard", "origin/HEAD"], &tui_dir);
    let version_despues = leer_version(&format!("{}/Cargo.toml", tui_dir));

    let hay_cambio = version_antes != version_despues;
    let no_instalado = !std::path::Path::new(&format!("{}/.cargo/bin/basalto-tui", home)).exists();

    if !hay_cambio && !no_instalado {
        pb.finish_with_message(format!(
            "{} basalto-tui  {}",
            style("✓").green(),
            style(format!("v{}", version_despues.as_deref().unwrap_or("?"))).cyan(),
        ));
        return;
    }

    pb.set_message("basalto-tui  instalando...");
    let ok = correr_silencioso("cargo", &["install", "--path", &tui_dir], home);

    if !ok {
        pb.finish_with_message(format!("{} basalto-tui  error al instalar", style("✗").red()));
        return;
    }

    if hay_cambio {
        pb.finish_with_message(format!(
            "{} basalto-tui  {} {} {}",
            style("✓").green(),
            style(format!("v{}", version_antes.as_deref().unwrap_or("?"))).cyan(),
            style("→").yellow(),
            style(format!("v{}", version_despues.as_deref().unwrap_or("?"))).cyan(),
        ));
    } else {
        pb.finish_with_message(format!(
            "{} basalto-tui  {}",
            style("✓").green(),
            style(format!("v{}", version_despues.as_deref().unwrap_or("?"))).cyan(),
        ));
    }
}

fn leer_version(cargo_toml: &str) -> Option<String> {
    std::fs::read_to_string(cargo_toml).ok().and_then(|content| {
        content
            .lines()
            .find(|l| l.starts_with("version"))
            .and_then(|l| l.split('"').nth(1))
            .map(|v| v.to_string())
    })
}
