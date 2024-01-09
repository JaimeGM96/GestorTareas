# Metodología y herramientas de testing
## Elección de herramientas
## Criterios de selección
Necesitamos que la herramienta de testing que elijamos cumpla los siguientes criterios:
- **Estándar**: En primer lugar, debemos conocer cuál es el estándar del lenguaje y estudiar las herramientas de testing que siguen el mismo.
- **Eficiencia**: Necesitamos que la ejecución de los tests sea rápida ya que, para poder subir cualquier cambio a producción, es necesario que todos los tests pasen y este proceso debe ser lo más rápido posible.
- **Mantenimiento**: La herramienta de testing que elijamos tiene que recibir mantenimiento con la mayor frecuencia posible. Esto nos asegura que es un software que no va a dejar de recibir actualizaciones a corto plazo y que, por tanto, no va a quedar obsoleto y no va a aumentar nuestra deuda técnica.
- **Seguridad**: Es necesario que, al igual que el resto de herramientas que usemos y nuestro propio software, la herramienta de testing que elijamos sea segura.

## Tests runners
### [Cargo](https://doc.rust-lang.org/cargo/)
Cargo dispone de su propio test runner. Como hemos visto antes, es la herramienta éstandar del lenguaje Rust para realizar tareas como la gestión de dependencias, la compilación del código, etc. Únicamente debemos colocar la etiqueta #[test] encima de la función que queramos establecer como un test y, cuando ejecutemos el comando `cargo test`, se ejecutarán todos los tests que hayamos escrito. Es una herramienta rápida y, al ser la herramienta éstandar del lenguaje, recibe mantenimiento frecuentemente, con lo que minimizaremos la deuda técnica. Además de todo lo nombrado, como habíamos comentado en la gestión de dependencias y de tareas, es una herramienta segura.

### [cargo-nextest](https://nexte.st/)
Cargo-nextest es un test runner que amplía las funcionalidades de Cargo para la ejecución, validación y visualización de los tests. Es 3 veces más rápido que Cargo porque tiene un modelo de ejecución diferente al de Cargo. Una de las diferencias es la ejecución en paralelo de los tests. Es un proyecto que recibe actualizaciones con frecuencia y dispone de mecanismos de seguridad.

## Frameworks de testing
### [Rust](https://doc.rust-lang.org/rust-by-example/testing.html)
El propio lenguaje de programación, nos proporciona la posibilidad de realizar tests. Estos tests pueden ser tanto unitarios como de integración. Para ello se declara un modulo llamado tests que debe contener en su encabezado la etiqueta #[cfg(test)] y, dentro de este, se declaran las funciones que queramos que sean tests con la etiqueta #[test].

### [Proptest](https://lib.rs/crates/proptest)
Proptest es un framework de testing que nos permite probar que, para entradas arbitrarias en nuestros tests, se cumplan ciertas propiedades. Para ello, englobamos nuestros tests dentro de la sección proptest!. La última versión estable es de 2020, aunque se siguen haciendo actualizaciones en el repositorio.


## Bibliotecas de aserciones
### [Aserciones de Rust](https://doc.rust-lang.org/std/macro.assert.html)
Al igual que muchas otras funciones, Rust dispone de su propia batería de aserciones. Estas aserciones son básicas pero nos permiten cubrir muchos casos de uso que queramos testear en nuestro código.

### [more_asserts](https://docs.rs/more-asserts/latest/more_asserts/)
Es una biblioteca de aserciones que incorpora funcionalidades que bien se pueden llegar a realizar con las aserciones de Rust, son más intuitivas para los casos concretos que cubren.

### [assert2](https://lib.rs/crates/assert2)
La principal característica de esta biblioteca de aserciones es que ofrece, con la asercion check!, mensajes bastante descriptivos además de poder realizar diversas aserciones en el test antes de la ejecución de un panic!, lo que llevaría a terminar el test. Esto nos permite encontrar a la vez todos los errores que se produzcan en nuestro código sin tener que realizar muchas ejecuciones del test, lo que conlleva un ahorro de tiempo.

### Elección
Por lo que se ha expuesto anteriormente, el test runner que se ha decidido usar es cargo-nextest ya que, además de cumplir con todos los criterios de selección como también lo hace Cargo, es mucho más eficiente que Cargo y, por tanto, nos permitirá ahorrar tiempo en la ejecución de los tests. En cuanto a framework de testing y biblioteca de aserciones, se ha decidido usar las funciones por defecto de Rust. Las características adicionales que nos proporcionan las otras herramientas no nos solucionan problemas que no podamos solventar con lo que Rust nos proporciona y nos haría aumentar la deuda técnica.