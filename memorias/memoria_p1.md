# Búsqueda local y greedy para el PAR

> Autor: Andrés Millán
> DNI:
> Email: amilmun@correo.ugr.es
> Grupo de prácticas: MH3


  * * *


# Tabla de contenidos

- [Sobre esta memoria](#sobre-esta-memoria)
    - [Benchmark de uno o más algoritmos](#benchmark-de-uno-o-más-algoritmos)
    - [Ejecutar uno o varios algoritmos para un dataset en particular](#ejecutar-uno-o-varios-algoritmos-para-un-dataset-en-particular)
- [Descripción del problema](#descripción-del-problema)
- [Procedimiento seguido para resolver la práctica](#procedimiento-seguido-para-resolver-la-práctica)
  - [Crates usadas](#crates-usadas)
  - [Estructura del programa](#estructura-del-programa)
  - [La clase Clusters](#la-clase-clusters)
- [Algoritmos considerados](#algoritmos-considerados)
  - [Greedy](#greedy)
    - [Descripción del algoritmo](#descripción-del-algoritmo)
    - [Implementación](#implementación)
  - [Búsqueda local](#búsqueda-local)
    - [Descripción del algoritmo](#descripción-del-algoritmo-1)
    - [Implementación](#implementación-1)
- [Análisis de resultados](#análisis-de-resultados)
  - [Descripción de los casos del problema empleados](#descripción-de-los-casos-del-problema-empleados)
  - [Resultados obtenidos](#resultados-obtenidos)
  - [Síntesis](#síntesis)
- [Referencias](#referencias)


  * * *


## Sobre esta memoria

En esta memoria se recoge toda la información relevante al desarrollo de la práctica 1, así como los conceptos necesarios para resolver el problema que estamos tratando.

Todo el código está subido en el repositorio de Github https://github.com/Asmilex/Metaheuristicas. Para ejecutarlo, es necesario [tener instalado Rust](https://www.rust-lang.org/tools/install). El comprimido de la entrega contendrá los mismos archivos que se encuentran en el repositorio. El único cambio será la fecha de la versión y la localización del PDF generado a partir de este archivo.

Se puede compilar el proyecto con `cargo build --release`. Sin embargo, es necesario especificar qué se quiere hacer para usarlo. Estas son las posibilidades:

#### Benchmark de uno o más algoritmos

Escribir en la línea de comandos `cargo run --release benchmark [algoritmos]`. donde `[algoritmos]` son uno o más elementos de la siguiente lista:
- `greedy`.
- `bl`.

Si no se especifica ninguno, se usarán todos. Cada algoritmo se ejecuta 5 veces por dataset (por lo que cada uno se realiza 30 veces). La información resultante se exportará al archivo `./data/csv/[nombre del algoritmo]_[dataset]_[número de restricciones].csv`, el cual contendrá las medidas necesarias para el análisis de la práctica.

#### Ejecutar uno o varios algoritmos para un dataset en particular

Para ejecutar un único algoritmo para un cierto dataset, se debe introducir en la línea de comandos `cargo run --release [dataset] {10, 20} [algoritmos]`, donde `[dataset]` puede valer `bupa`, `glass` o `zoo`, eligiendo qué conjunto de restricciones usar (`10` o `20`). La lista de algoritmos funciona de la misma manera que en apartado anterior.


  * * *


## Descripción del problema

A lo largo de estas prácticas se resolverá el problema del **agrupamiento con restricciones**. Este es una modificación del clásico problema del *clustering*, el cual se describe de la siguiente forma:

Se nos presenta una lista de elementos con un cierto número de atributos. Los representaremos como vectores en $[0, 1]^d$. Debemos agruparlos en un cierto número de categorías, llamados **clusters**, de forma que se minimice la distancia entre estos vectores.

Nuestro matiz consiste en que les ponemos restricciones a los elementos a analizar. De esta forma, se describe cuándo un vector debe estar en el mismo cluster que otro, o cuándo deben estar en distintos. Por tanto, no solo debemos conseguir una distancia intracluster baja, sino que se debe violar el mínimo número de restricciones posibles.

A lo largo de estas prácticas propondremos diferentes algoritmos para resolver este problema. En la práctica 1, presentaremos soluciones sencillas basadas en algoritmos simples como **Greedy** o **Búsqueda Local**.


  * * *


## Procedimiento seguido para resolver la práctica

Todo el código está escrito en **Rust**. Es un lenguaje moderno, eficiente y muy seguro en temas de acceso a memoria. Su elección ha sido puro interés personal.

### Crates usadas

Para facilitar la implementación, se han utilizado una serie de *crates* (nombre que reciben las librerías por parte de los *rustáceos*). Estas son:
- [Naglebra](https://www.nalgebra.org/): una librería de álgebra lineal. Utilizada para operaciones con matrices y vectores.
- [Rand](https://docs.rs/rand/0.8.3/rand/): para los generadores de números aleatorios
- [Multimap](https://docs.rs/multimap/0.8.3/multimap/index.html): para el almacenamiento eficiente de las listas de restricciones.
- [Csv](https://docs.rs/csv/1.1.6/csv/): para exportar los resultados a `.csv`.
- [Colored](https://docs.rs/colored/2.0.0/colored/): para hacer más bonitas y legibles las salidas a consola.


### Estructura del programa

Se han dividido las funcionalidades clave del programa en distintos ficheros. Estos son:
- `main.rs`: contiene el código relacionado con la ejecución de una ejecución simple y de un benchmark
- `file_io.rs`: aquí se ubican las funciones relacionadas con entrada/salida de archivos. En particular, la lectura de los ficheros de restricciones y la salida a los archivos csv.
- `utils.rs`: se definen las diferentes estructuras relacionadas con el problema. Por ejemplo, `PAR_parametros` devuelve los datos necesarios para operar dado un dataset.
- `algorithm.rs`: todos los algoritmos implementados se encuentran aquí. Ahora mismo, estos son greedy y búsqueda local.
- `cluster.rs`: las principales estructuras necesarias para resolver el problema se localizan en este fichero. Específicamente, la clase `Clusters`. En la siguiente sección se detalla su implementación.

### La clase Clusters


  * * *


## Algoritmos considerados
TODO hay que incluir, en total:
- Consideraciones comunes a los algoritmos.
- Descripción de esquema de representación de solución.
- Descripción en pseudocódigo de la función objetivo y los operadores comunes.
- Pseudocódigo de la estructura del método de búsqueda y operaciones relevantes de cada algoritmo.
- Pseudocódigo de los algoritmos. Los implementados, no los de la teoría.
  - *Incluirá la descripción en pseudocódigo del método de exploración del entorno, el operador de generación de vecino y la generación de soluciones*
- Pseudocódigo de los algoritmos de comparación.

### Greedy
#### Descripción del algoritmo
#### Implementación

### Búsqueda local
#### Descripción del algoritmo
#### Implementación


  * * *


  * * *


## Análisis de resultados
### Descripción de los casos del problema empleados
### Resultados obtenidos
### Síntesis


  * * *


## Referencias

- [El libro oficial de Rust](https://doc.rust-lang.org/book/)
- [Nalgebra, una librería de álgebra lineal en Rust](https://www.nalgebra.org/)
- [Multimap para Rust](https://docs.rs/multimap/0.8.3/multimap/)
- [StackOverflow](https://stackoverflow.com/)
- Material de teoría y de prácticas