use basalto_shared::BasaltoPlugin;
use libloading::Symbol;

fn main() {
    let ruta = "target/debug/libplugin_buses.so";

    let lib = unsafe { libloading::Library::new(ruta).expect("No se pudo cargar la lib") };

    let contructor: Symbol<fn() -> *mut dyn BasaltoPlugin> = unsafe {
        lib.get(b"_basalto_create_plugin")
            .expect("No encontre el timbre")
    };

    let plugin_raw = contructor();

    let plugin = unsafe { Box::from_raw(plugin_raw) };

    println!("Plugin cargado con exito {}", plugin.name());
}
