# Gestor de dependencias
En este proyecto, se van a estudiar cuáles son los diferentes gestores de dependencias que existen y, siguiendo una serie de criterios que se van a exponer a continuación, se va a elegir el que mejor se adapte a las necesidades del proyecto.
## Criterios de selección
Necesitamos que el gestor de dependencias que elijamos cumpla los siguientes criterios:
- **Estándar**: En primer lugar, debemos conocer cuál es el estándar del lenguaje y estudiar los gestores de dependencias que siguen el mismo.
- **Eficiencia**: Aunque nuestro proyecto no sea muy grande por ahora, necesitamos que las dependencias se gestionen de forma rápida para evitar tiempos de espera demasiado largos cuando se instalen.
- **Mantenimiento**: El gestor de dependencias que elijamos tiene que recibir mantenimiento con la mayor frecuencia posible. Esto nos asegura que es un software que no va a dejar de recibir actualizaciones a corto plazo y que, por tanto, no va a quedar obsoleto y no va a aumentar nuestra deuda técnica.
- **Seguridad**: Es importante que nuestro software sea seguro y un paso para lograrlo es que nuestro gestor de dependencias también lo sea.

## Gestores de dependencias
### [Cargo](https://doc.rust-lang.org/cargo/)
Cargo es el gestor de dependencias oficial de Rust y, por lo tanto, es el que establece el éstandar del lenguaje. Al estar escrito en Rust, es muy eficiente. Como ya hemos nombrado, al ser una herramienta oficial del lenguaje, nos aseguramos que reciba mantenimiento frecuentemente y a largo plazo. También es una herramienta segura ya que las dependencias las instala desde crate.io, el repositorio oficial de paquetes de Rust. Además hace uso de checksums para verificar la integridad de las dependencias instaladas.

## Elección
Por todo lo nombrado anteriormente, el gestor de dependencias que se ha decidido usar es Cargo, ya que cumple con todos los criterios y, como se verá en el fichero de [gestor de tareas](gestor-tareas.md), también nos permite gestionar tareas.