# Gestor de tareas
En este proyecto, se van a estudiar cuáles son los diferentes gestores de tareas que existen y, siguiendo una serie de criterios que se van a exponer a continuación, se va a elegir el que mejor se adapte a las necesidades del proyecto.
## Criterios de selección
Necesitamos que el gestor de tareas que elijamos cumpla los siguientes criterios:
- **Estándar**: En primer lugar, debemos conocer cuál es el estándar del lenguaje y estudiar los gestores de tareas que siguen el mismo.
- **Eficiencia**: Para la gestión de tareas, necesitamos que se realicen en el menor tiempo posible para evitar tiempos de espera demasiado largos.
- **Mantenimiento**: El software que elijamos como gestor de tareas tiene que recibir actualizaciones con relativa frecuencia. Esto nos evitará depender de un software en desuso y reduciremos la deuda técnica.
- **Seguridad**: Como cualquier elemento que forme parte de nuestro software, es importante que el gestor de tareas que elijamos sea seguro.

## Gestores de tareas
### [Cargo](https://doc.rust-lang.org/cargo/)
Además de ser un gestor de dependencias, Cargo nos da la posibilidad de gestionar tareas. Dispone de multitud de tareas predefinidas como por ejemplo build, run, test, entre otras. Además de eso podemos definir nosotros nuevas tareas. Como se ha nombrado en el fichero de documentación del [gestor de dependencias](gestor-dependencias.md), cumple con todos los criterios que se han establecido.

### [Cargo-make](https://sagiegurari.github.io/cargo-make/)
Cargo make es un gestor de tareas de Rust que se integra con Cargo. Sigue el estándar de Rust y es muy eficiente. Recibe mantenimiento frecuentemente y nos proporciona una serie de herramientas para hacer que nuestras tareas sean más seguras.

## Elección
El gestor de dependencias que se ha decidido usar es Cargo ya que, además de cumplir con todos los criterios, si podemos elegir una única herramienta que realice todas las funciones que necesitamos, reducimos la deuda técnica que nos puede causar el tener más herramientas.