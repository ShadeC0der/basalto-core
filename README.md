# basalto-core

El núcleo del ecosistema Basalto. Carga plugins, despacha comandos y gestiona el entorno `~/.basalto/`.

## El ecosistema

| Componente | Descripción |
|---|---|
| [basalto-shared](https://github.com/ShadeC0der/basalto-shared) | El contrato — trait `BasaltoPlugin` que conecta Core y plugins |
| [basalto-library](https://github.com/ShadeC0der/basalto-library) | Plugin oficial — gestiona una biblioteca personal de archivos con `add`, `show`, `edit` y `push` |

## Qué hace

basalto-core es un microkernel — nunca agrega funcionalidad por sí solo. Todo es un plugin.

1. Crea `~/.basalto/` en el primer arranque
2. Lee los plugins declarados en `~/.basalto/plugins/*.toml`
3. Clona y compila cada plugin si no está instalado
4. Verifica compatibilidad de versión de `basalto-shared` antes de cargar cada plugin
5. Despacha el comando del usuario al plugin correspondiente

## Comandos built-in

| Comando | Descripción |
|---|---|
| `basalto version` | Muestra la versión del core y los plugins activos |
| `basalto version --check` | Verifica si hay actualizaciones disponibles |
| `basalto update` | Actualiza todos los plugins y el core |
| `basalto update --clean` | Limpia el cache antes de actualizar (reinstalación completa) |
| `basalto help` | Muestra todos los comandos disponibles con sus flags |
| `basalto clear-cache` | Muestra el tamaño del cache y pide confirmación para borrarlo |
| `basalto clear-cache --yes` | Borra el cache sin pedir confirmación |

## Uso

```
basalto <comando> [args]
```

## Agregar un plugin

Crea un archivo `.toml` en `~/.basalto/plugins/`:

```toml
source = "git@github.com:usuario/mi-plugin.git"
branch = "main"
enabled = true
```

basalto-core lo clona y compila automáticamente en el próximo arranque.

## Instalación

```
cargo install --path .
```

Compila el binario en modo release y lo instala en `~/.cargo/bin/basalto`.

Para actualizar el core desde el propio sistema:

```
basalto update
```

## Compilar sin instalar

```
cargo build --release
```

Requiere `git` y `cargo`.
