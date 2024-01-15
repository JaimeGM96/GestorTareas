# Imagen del contenedor
En este proyecto, se va a investigar cuales son las diferentes imágenes que existen para crear un contenedor con Rust que nos permita ejecutar nuestro código. Para ello, se expondrán una serie de criterios de selección que nos indicarán los requisitos que deben cumplir las imágenes para ver cual se adapta mejor a nuestras necesidades.

## Criterios de selección
Los criterios de selección que se van a tener en cuenta son los siguientes:
- **Estándar**: En primer lugar, debemos conocer cuál es el estándar del lenguaje y estudiar las imágenes que siguen el mismo.
- **Eficiencia**: Necesitamos que la imagen que elijamos sea eficiente ya que, para poder subir cualquier cambio a producción, es necesario que la imagen se construya lo más rápido posible.
- **Mantenimiento**: La imagen que elijamos tiene que recibir mantenimiento con la mayor frecuencia posible. Esto nos asegura que es un software que no va a dejar de recibir actualizaciones a corto plazo y que, por tanto, no va a quedar obsoleto y no va a aumentar nuestra deuda técnica.
- **Seguridad**: Es necesario que, al igual que el resto de herramientas que usemos y nuestro propio software, la imagen que elijamos sea segura.
- **Tamaño**: El tamaño de la imagen es importante ya que, cuanto más grande sea, más tiempo tardará en descargarse y más espacio ocupará.

## Imágenes
Existen diferentes imágenes que nos permiten crear un contenedor con Rust, tanto oficiales y no oficiales. A continuación, se van a exponer las imágenes que se han estudiado para ver cual se adapta mejor a los criterios de selección.

### [Rust-alpine](https://hub.docker.com/_/rust)
Es la imágen oficial de Rust sobre Alpine Linux. Es una imagen ligera, 262MB comprimida, y una vez montada con las dependencias pesa 824MB. En cuanto a la eficiencia, el tiempo de contruir la imagen es de 18.7 segundos y el tiempo de ejecución es de 6 segundos. Como podemos ver en docker hub, es la más segura de todas ya que no se han encontrado vulnerabilidades. Al se una imagen oficial, ésta recibe mantenimiento con bastante frecuencia.

### [Rust-slim-bullseye](https://hub.docker.com/_/rust)
Es la imágen oficial de Rust sobre Debian Bullseye. Esta imagen es más ligera que la anterior, 256MB comprimida, y una vez montada con las dependencias pesa 761MB. En cuanto a la eficiencia, es parecida a la imagen anterior, rebajando a los 5 segundos el tiempo de ejecución. Tiene vulnerabilidades aunque son de baja prioridad, aunque su número es elevado. Al se una imagen oficial, ésta recibe mantenimiento con bastante frecuencia.

### [Rust-slim-buster](https://hub.docker.com/_/rust)
Es la imágen oficial de Rust sobre Debian Buster. De las imágenes oficiales, es la más ligera con 238MB comprimida, y una vez montada con las dependencias pesa 712MB. En cuanto a la eficiencia, sigue mejorando con respecto a la anterior, rebajando el tiempo de ejecución a los 4 segundos. Es la que mayor número de vulnerabilidades tiene, aunque todas son de baja prioridad. También recibe mantenimiento frecuente al ser una imagen oficial.

### [cimg/rust](https://hub.docker.com/r/cimg/rust)
Es una imagen no oficial de Rust. El tamaño sin comprimir es considerablemente más grande incluso es mayor que las imagenes anteriores ya montadas. También tarda más tiempo en construirse y en ejecutarse. No tiene vulnerabilidades aunque si que recibe mantenimiento con bastante frecuencia.

## Elección
La imagen por la que se va a optar es la de Debian slim buster. A pesar de sus vulnerabilidades, no son tan importantes y lo compensa el tiempo de ejecución y el tamaño que ocupa. Y como ya hemos visto, cumple con el resto de criterios de selección.