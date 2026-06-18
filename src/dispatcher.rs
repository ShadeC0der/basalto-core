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
