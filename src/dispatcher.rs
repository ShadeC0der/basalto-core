use crate::installer; // Clona repo del plugin y compila el .so en caso que no exista
use crate::plugins::PluginConf; // Accede al vector con los plugins
use basalto_shared::BasaltoPlugin; // Accede al la librería del contrato entre los plugins y el core
use std::collections::HashMap; // Estructura de datos que guarda pares clave → valor ej: "help"  →  plugin_cli
use std::rc::Rc; // Permite que múltiples partes del código apunten al mismo valor sin copiarlo

/* fn build
 * Obtiene el vector de todos los plugins declarados como parámetro
 * Retorna un HashMap["command1" -> "pluginA"]
 * Gracias a Rc otro comando puede estar bajo el mismo plugin
 * Ej: HashMap["command2" -> "pluginA"]
 * También retorna Vec<Library> para mantener los .so cargados en memoria
 */

pub fn build(
    plugins: &[PluginConf],
) -> (
    HashMap<String, Rc<dyn BasaltoPlugin>>,
    Vec<libloading::Library>,
) {
    /* Resumen de build()
     * map: Creamos un HashMap vacío
     * libs: Creamos un vector vacío, libloading sirve para cargar y guardar los .so
     * Se obtiene la ruta de HOME
     *
     * El bucle for itera sobre todos los plugins obtenidos
     *  Si enabled es false omite ese plugin desactivado
     *  Para obtener el nombre se divide la ruta por cada '/'
     *  Toma el ultimo elemento
     *  Abre el resultado y si tiene .git lo quita
     *  De manera que queda solo el nombre del plugin y quita el resto de la ruta
     *
     *  path: Usamos el nombre obtenido para construir la ruta a su .so
     *  Se usa installer para clonar y compilar el plugin si no existe
     *  Se carga el .so en memoria con libloading y se guarda en lib
     *
     *  plugin: Se conforma en 3 pasos
     *      Busca la función _basalto_create_plugin dentro del .so
     *      Llama la función constructor() para crear un puntero del plugin (raw pointer)
     *      Envuelve el puntero, le asigna un dueño y lo convierte en un Rc
     *      para que varios comandos del mapa apunten a la misma instancia
     *
     *  Por cada comando del plugin se agrega una entrada al mapa
     *  La clave es el nombre del comando y el valor es un Rc::clone apuntando a la misma instancia
     *
     *  Se mueve lib al vector libs — solo al final porque plugin lo necesita antes
     */

    let mut map = HashMap::new();
    let mut libs: Vec<libloading::Library> = Vec::new();
    let home = dirs::home_dir().unwrap();
    let home = home.to_str().unwrap();

    for input in plugins {
        if !input.enabled {
            continue;
        }

        let name = input
            .source
            .split('/')
            .next_back()
            .unwrap()
            .trim_end_matches(".git");

        let so_name = name.replace('-', "_");
        let path = format!(
            "{}/.basalto/cache/plugins/{}/target/release/lib{}.so",
            home, name, so_name
        );

        installer::ensure(name, &input.source, &path, &input.branch);
        let lib = unsafe { libloading::Library::new(&path).unwrap() };

        // Verifica compatibilidad de basalto-shared antes de cargar el plugin
        let plugin_shared_version: &str = unsafe {
            match lib.get::<unsafe extern "C" fn() -> &'static str>(b"_basalto_shared_version") {
                Ok(f) => f(),
                Err(_) => {
                    println!(
                        "Plugin '{}' no declara version de basalto-shared. Corre: basalto update",
                        name
                    );
                    continue;
                }
            }
        };

        let core_major = basalto_shared::SHARED_VERSION
            .split('.')
            .next()
            .unwrap_or("0");
        let plugin_major = plugin_shared_version.split('.').next().unwrap_or("0");

        if core_major != plugin_major {
            println!(
                "Plugin '{}' usa basalto-shared v{} (core usa v{}). Corre: basalto update",
                name, plugin_shared_version, basalto_shared::SHARED_VERSION
            );
            continue;
        }

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
