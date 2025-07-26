# Rustisvn
Una Tui app simple y minimalista para svn

Hecho en rust, rustisvn pretende ser una herramienta moderna y 
ligera para aquellos usuarios que aún utilizen el manejador de versiones SVN;

## Instalación
Por el momento solo esta disponible para linux
### Binario
1. Primero debemos descargar el binario de la release
2. Abrir la terminal y ejecutar el siguinte comando
```
cp rsvn ~/.local/bin
```
3. Testear si el binario se puede ejecutar
```
rsvn
```

## Tips
Puedes ejecutar el siguiente comando para ayuda con el comando
```
rsvn -h
```

## Menú
Por el momento no es visible el menú de atajos. Puedes ver el Menú en el archivo de [default-atajos]. 
El programa funciona por modos:
- Normal
- Selección
- Commit
Cada uno tiene sus atajos y formas de interactuar con los archivos.

[default-atajos]: ./default-atajos.md

## Stack
Se trabajo bajo el framework TUI para herramientas basadas en rust [Ratatui]

[Ratatui]: https://ratatui.rs

## License
Copyright (c) spaceunandev <jocsan.fonseca@di.unanleon.edu.ni>
This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
