# basalto-core

El núcleo del ecosistema Basalto. Carga plugins, despacha comandos y gestiona el entorno `~/.basalto/`.

## El ecosistema

| Componente | Descripción |
|---|---|
| [basalto-shared](https://github.com/ShadeC0der/basalto-shared) | El contrato — trait `BasaltoPlugin` que conecta Core y plugins |

## Qué hace

basalto-core es un microkernel — nunca agrega funcionalidad por sí solo. Todo es un plugin.

1. Crea `~/.basalto/` en el primer arranque
2. Lee los plugins declarados en `~/.basalto/plugins/*.toml`
3. Clona y compila cada plugin si no está instalado
4. Despacha el comando del usuario al plugin correspondiente

## Uso

```
basalto <comando> [args]
basalto --time <comando> [args]   # muestra el tiempo de cada fase
```

## Agregar un plugin

Crea un archivo `.toml` en `~/.basalto/plugins/`:

```toml
source = "https://github.com/usuario/mi-plugin"
branch = "main"
enabled = true
```

basalto-core lo clona y compila automáticamente en el próximo arranque.

## Instalación

```
cargo install --path .
```

Compila el binario en modo release y lo instala en `~/.cargo/bin/basalto`. A partir de ahí se puede usar desde cualquier terminal.

Para actualizar después de cambios en el Core, correr el mismo comando — sobreescribe el binario anterior.

## Compilar sin instalar

```
cargo build --release
```

Requiere `git` y `cargo`.
