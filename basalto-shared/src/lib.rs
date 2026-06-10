// Contrato de comportamiento para todos los plugin
pub trait BasaltoPlugin {
    // Devuelve el nombre unico del plugin
    fn name(&self) -> &str;

    // Prepara los recursos del plugin
    fn on_load(&self);

    // Ejecuta la tarea principal del plugin
    fn execute(&self);
}
