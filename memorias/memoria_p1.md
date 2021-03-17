# Búsqueda local y greedy para el PAR

> Autor: Andrés Millán
> DNI:
> Email: amilmun@correo.ugr.es
> Grupo de prácticas: MH3

  * * *

## Tabla de contenidos

- [Tabla de contenidos](#tabla-de-contenidos)
- [Sobre esta memoria](#sobre-esta-memoria)
    - [Benchmark de uno o más algoritmos](#benchmark-de-uno-o-más-algoritmos)
    - [Ejecutar uno o varios algoritmos para un data set en particular](#ejecutar-uno-o-varios-algoritmos-para-un-data-set-en-particular)
- [Descripción del problema](#descripción-del-problema)
- [Algoritmos considerados](#algoritmos-considerados)
  - [Greedy](#greedy)
    - [Descripción del algoritmo](#descripción-del-algoritmo)
    - [Implementación](#implementación)
  - [Búsqueda local](#búsqueda-local)
    - [Descripción del algoritmo](#descripción-del-algoritmo-1)
    - [Implementación](#implementación-1)
- [Procedimiento considerado para resolver la práctica](#procedimiento-considerado-para-resolver-la-práctica)
- [Análisis de resultados](#análisis-de-resultados)
  - [Descripción de los casos del problema empleados](#descripción-de-los-casos-del-problema-empleados)
  - [Resultados obtenidos](#resultados-obtenidos)
  - [Síntesis](#síntesis)
- [Referencias](#referencias)

  * * *

## Sobre esta memoria

En esta memoria se recoge toda la información relevante al desarrollo de la práctica 1, así como los conceptos necesarios para resolver el problema que estamos tratando.

Todo el código está subido en el repositorio de Github https://github.com/Asmilex/Metaheuristicas. Para ejecutarlo, es necesario [tener instalado Rust](https://www.rust-lang.org/tools/install). El comprimido de la entrega contendrá los mismos archivos que se encuentran en el repositorio, pero con la estructura cambiada para que la memoria se encuentre en la raíz.

Se puede compilar el proyecto con `cargo build --release`. Sin embargo, es necesario especificar qué se quiere hacer para usarlo. Estas son las posibilidades:

#### Benchmark de uno o más algoritmos

Escribir en la línea de comandos `cargo run --release benchmark [algoritmos]`. donde `[algoritmos]` son uno o más elementos de la siguiente lista:
- `greedy`.
- `bl`.

Si no se especifican, se usarán todos. Cada algoritmo se ejecuta 5 veces por data set (por lo que cada uno se realiza 30 veces). La información resultante se exportará al archivo `TODO.csv`, el cual contendrá las medidas necesarias para el análisis de la práctica.

#### Ejecutar uno o varios algoritmos para un data set en particular

En la línea de comandos, `cargo run --release [dataset] {10, 20} [algoritmos]`, donde `[dataset]` puede valer `bupa`, `glass` o `zoo`, eligiendo qué conjunto de restricciones usar (`10` o `20`). La lista de algoritmos funciona de la misma manera que en apartado anterior.


  * * *

## Descripción del problema

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

## Procedimiento considerado para resolver la práctica

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
- Saltos de página: https://support.typora.io/Page-Breaks/ BORRAR ESTO DE LA ENTREGA !!! FIXME