# Metodología y herramientas de testing
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

### [Proptest](https://lib.rs/crates/proptest)
Proptest es un framework de testing que nos permite probar que, para entradas arbitrarias en nuestros tests, se cumplan ciertas propiedades. Está basado en el framework QuickCheck de Haskell y en Hypothesis de Python. La última versión estable es de 2020, aunque se siguen haciendo actualizaciones en el repositorio.

### [Trybuild](https://lib.rs/crates/trybuild)
Trybuild hace uso del compilador de Rust (rustc) para comprobar que los mensajes de error que se muestran al compilar el código son los esperados. Se escriben casos de prueba que desencadenan errores y se comprueban que son los esperados. La última versión estable es de diciembre de 2023. 

### Elección
Por lo que se ha expuesto anteriormente, la herramienta de testing que se ha decidido usar es cargo-nextest ya que, además de cumplir con todos los criterios de selección como también lo hace Cargo, es mucho más eficiente que Cargo y, por tanto, nos permitirá ahorrar tiempo en la ejecución de los tests.