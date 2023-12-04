# Metodología y herramientas de testing
## Elección de metodología
En el desarrollo de software, es imprescindible realizar tests para comprobar que todo el código que se escriba sea correcto. Para ello, existen diferentes metodologías que nos indican cómo se tiene que seguir el proceso de desarrollo de tests. A continuación se van a exponer las diferentes metodologías de desarrollo de tests basadas en el principio F.I.R.S.T. (Fast, Independent, Repeatable, Self-validating y Timely).

### [TDD](https://es.wikipedia.org/wiki/Desarrollo_guiado_por_pruebas)
El TDD es una metodología de desarrollo de software que consiste en escribir primero los tests y, a continuación, escribir el código que haga que los tests pasen. Una característica de esta metodología es evitar escribir código inneceasario, lo que nos permite mantener el código más limpio. 

### [BDD](https://es.wikipedia.org/wiki/Desarrollo_guiado_por_comportamiento)
El BDD, al igual que TDD, es una metodología basada en escribir los tests primero. La diferencia que existe entre ambas metodologías es que, en el BDD, los tests se escriben en un lenguaje más cercano al lenguaje natural, permitiendo que todos los miembros de un equipo, tanto desarrolladores como no, puedan participar en el desarrollo del dominio.

## Elección
Para este proyecto, se va a hacer uso de BDD ya que, al describir los tests con lenguaje natural, resulta más sencillo cubrir todos los casos de uso y, por tanto, se consigue una mayor cobertura de tests.

## Elección de herramientas
## Criterios de selección
Necesitamos que la herramienta de testing que elijamos cumpla los siguientes criterios:
- **Estándar**: En primer lugar, debemos conocer cuál es el estándar del lenguaje y estudiar las herramientas de testing que siguen el mismo.
- **Eficiencia**: Necesitamos que la ejecución de los tests sea rápida ya que, para poder subir cualquier cambio a producción, es necesario que todos los tests pasen y este proceso debe ser lo más rápido posible.
- **Mantenimiento**: La herramienta de testing que elijamos tiene que recibir mantenimiento con la mayor frecuencia posible. Esto nos asegura que es un software que no va a dejar de recibir actualizaciones a corto plazo y que, por tanto, no va a quedar obsoleto y no va a aumentar nuestra deuda técnica.
- **Seguridad**: Es necesario que, al igual que el resto de herramientas que usemos y nuestro propio software, la herramienta de testing que elijamos sea segura.

### [Cargo](https://doc.rust-lang.org/cargo/)
Cargo dispone de su propio test runner. Como hemos visto antes, es la herramienta éstandar del lenguaje Rust para realizar tareas como la gestión de dependencias, la compilación del código, etc. Únicamente debemos colocar la etiqueta #[test] encima de la función que queramos establecer como un test y, cuando ejecutemos el comando `cargo test`, se ejecutarán todos los tests que hayamos escrito. Es una herramienta rápida y, al ser la herramienta éstandar del lenguaje, recibe mantenimiento frecuentemente, con lo que minimizaremos la deuda técnica. Además de todo lo nombrado, como habíamos comentado en la gestión de dependencias y de tareas, es una herramienta segura.

### [cargo-nextest](https://nexte.st/)
Cargo-nextest es un test runner que amplía las funcionalidades de Cargo para la ejecución, validación y visualización de los tests. Es 3 veces más rápido que Cargo porque tiene un modelo de ejecución diferente al de Cargo. Una de las diferencias es la ejecución en paralelo de los tests. Es un proyecto que recibe actualizaciones con frecuencia y dispone de mecanismos de seguridad.
