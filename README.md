# Rustisvn

Una Tui app simple y minimalista para svn

Hecho en rust, rustisvn pretende ser una herramienta moderna y
ligera para usuarios que aún utilicen el sistema de control de versiones SVN.

## Instalación

Por el momento solo está disponible para linux

### Binario

1. Primero debemos descargar el binario de la release
2. Abrir la terminal y ejecutar el siguiente comando

```
cp rsvn ~/.local/bin
```

3. Probar si el binario se puede ejecutar

```
rsvn
```

### Código fuente

Puedes clonar el repositorio y compilarlo manualmente:

```
git clone git@github.com:SpicyDogWings/rustisvn.git
cd rustisvn
cargo build --release
cp target/release/rsvn ~/.local/bin
```

## Tips

Puedes ejecutar el siguiente comando para la ayuda del comando

```
rsvn -h
```

## Menú

El menú de atajos no es visible por ahora.
El programa opera en diferentes por modos(nvim user :p):

- Normal
- Selección
- Commit
  Cada uno tiene sus atajos y formas de interactuar con los archivos.
  A continuación, se presenta una tabla con los distintos modos de funcionamiento y sus atajos de teclado para interactuar con los archivos.

|     Modo      |         Atajo         | Acción                            |
| :-----------: | :-------------------: | :-------------------------------- |
|  **Normal**   |                       |                                   |
|               |       `ESPACIO`       | Alterna un archivo para commit.   |
|               |          `y`          | Copia la ruta (path) del archivo. |
|               | `k` / `FLECHA ARRIBA` | Sube el cursor.                   |
|               | `j` / `FLECHA ABAJO`  | Baja el cursor.                   |
|               |      `q` / `ESC`      | Salir de la aplicación.           |
| **Selección** |          `s`          | Entra al modo selección.          |
|               |       `ESPACIO`       | Alterna un archivo para commit.   |
|               | `k` / `FLECHA ARRIBA` | Sube el cursor.                   |
|               | `j` / `FLECHA ABAJO`  | Baja el cursor.                   |
|               |         `ESC`         | Sale del modo selección.          |
|  **Commit**   |          `c`          | Entra al modo commit.             |
|               |        `ENTER`        | Realiza el commit.                |
|               |         `ESC`         | Sale del modo commit.             |

## Stack

Se trabajó bajo el TUI framework para herramientas basadas en Rust [Ratatui]

[Ratatui]: https://ratatui.rs

## License

Copyright (c) spaceunandev <jocsan.fonseca@di.unanleon.edu.ni>
This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
