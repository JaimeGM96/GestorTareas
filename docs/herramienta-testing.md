# Metodología y herramientas de testing
## Elección de metodología


## Elección de herramientas
## Criterios de selección
Necesitamos que la herramienta de testing que elijamos cumpla los siguientes criterios:
- **Estándar**: En primer lugar, debemos conocer cuál es el estándar del lenguaje y estudiar las herramientas de testing que siguen el mismo.
- **Eficiencia**: Necesitamos que la ejecución de los tests sea rápida ya que, para poder subir cualquier cambio a producción, es necesario que todos los tests pasen y este proceso debe ser lo más rápido posible.
- **Mantenimiento**: La herramienta de testing que elijamos tiene que recibir mantenimiento con la mayor frecuencia posible. Esto nos asegura que es un software que no va a dejar de recibir actualizaciones a corto plazo y que, por tanto, no va a quedar obsoleto y no va a aumentar nuestra deuda técnica.
- **Seguridad**: Es necesario que, al igual que el resto de herramientas que usemos y nuestro propio software, la herramienta de testing que elijamos sea segura.

### [Cargo](https://doc.rust-lang.org/cargo/)
Cargo dispone de su propio test runner. Como hemos visto antes, es la herramienta éstandar del lenguaje Rust para realizar tareas como la gestión de dependencias, la compilación del código, etc. Únicamente debemos colocar la etiqueta #[test] encima de la función que queramos establecer como un test y, cuando ejecutemos el comando `cargo test`, se ejecutarán todos los tests que hayamos escrito. Es una herramienta rápida y, al ser la herramienta éstandar del lenguaje, recibe mantenimiento frecuentemente, con lo que minimizaremos la deuda técnica. Además de todo lo nombrado, como habíamos comentado en la gestión de dependencias y de tareas, es una herramienta segura.
