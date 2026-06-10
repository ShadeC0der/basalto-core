use basalto_shared::BasaltoPlugin;

// 1. Definimos la estructura del plugin
struct PluginBuses;

// 2. Implementamos las reglas del Contrato
impl BasaltoPlugin for PluginBuses {
    fn name(&self) -> &str {
        "Sistema de Telemetría de Buses V1"
    }

    fn on_load(&self) {
        println!("[Plugin Buses] Inicializando sensores GPS...");
    }

    fn execute(&self) {
        println!("[Plugin Buses] Procesando datos del recorrido 210...");
    }
}

// 3. LA PUERTA DE ENTRADA (El "Apretón de Manos")
#[unsafe(no_mangle)]
pub extern "C" fn _basalto_create_plugin() -> *mut dyn BasaltoPlugin {
    // Creamos el plugin, lo metemos en una caja y lo entregamos como puntero crudo
    let boxed = Box::new(PluginBuses);
    Box::into_raw(boxed)
}
