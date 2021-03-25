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
  - [La clase `Clusters`](#la-clase-clusters)
    - [Sobre las dimensiones](#sobre-las-dimensiones)
    - [Elementos del espacio](#elementos-del-espacio)
    - [Representación de las soluciones](#representación-de-las-soluciones)
    - [Restricciones](#restricciones)
    - [Estadísticos](#estadísticos)
- [Algoritmos considerados](#algoritmos-considerados)
  - [Greedy](#greedy)
    - [Descripción del algoritmo](#descripción-del-algoritmo)
  - [Búsqueda local](#búsqueda-local)
    - [Descripción del algoritmo](#descripción-del-algoritmo-1)
    - [Implementación](#implementación)
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
- `utils.rs`: se definen las diferentes estructuras relacionadas con el problema. Por ejemplo, `ParametrosDataset` agrupa los datos necesarios para operar un cierto dataset.
- `algorithm.rs`: todos los algoritmos implementados se encuentran aquí. Ahora mismo, estos son greedy y búsqueda local.
- `cluster.rs`: las principales estructuras necesarias para resolver el problema se localizan en este fichero. Específicamente, la clase `Clusters`. En la siguiente sección se detalla su implementación.


### La clase `Clusters`

Esta estructura supondrá el grueso de nuestro programa. *Agrupará* toda la información pertinente a la resolución del problema. En las siguientes secciones, describiremos sus elementos. No obstante, omitiremos las funciones de poco interés didáctico


#### Sobre las dimensiones

Necesitaremos tres medidas para generar una solución:

- `num_clusters` representa el número de clusters fijado por el problema.
- `dim_vectores` es el número de atributos del dataset.
- `num_elementos` es el número de vectores o muestras del dataset.


#### Elementos del espacio

La clase conoce en todo momento el conjunto de elementos del dataset que estamos tratando, así como sus distancias respectivas. Los miembros que se encargan de guardar esta información son `espacio` y `distancias` respectivamente. El cálculo de la variable $\lambda$, de la cual hablaremos más tarde, se guarda cuál es el máximo de las distancias al calcular la matriz `distancias`.

Para almacenar los vectores, hemos utilizado un vector de `Nalgebra::DVector`, un tipo de array dinámico con funciones de álgebra lineal. Esto nos será de gran ayuda, pues simplificará las operaciones del espacio vectorial con el que tratamos.


#### Representación de las soluciones

Las soluciones se representan con una lista de enteros, `lista_clusters`, de forma que, para una cierta entrada $i$ de dicha lista:
- Si su valor es $0$, entonces, ese elemento no tiene cluster asignado
- En otro caso, su valor está en el conjunto $\{1, ..., num\_clusters\}$.

Una solución solo se considerará válida si todo cluster tiene al menos un elemento asignado. Si algún elemento ha modificado su cluster, la clase automáticamente lo registra en la estructura `recuento_clusters`, por lo que nos resultará sencillo comprobar cuántos elementos hay en cada uno.


#### Restricciones

Representaremos las restricciones de dos formas distintas:
1. La primera de ellas es mediante una matriz (`restricciones`) con entradas que toman valores en ${0, 1, -1}$. Para una cierta entrada $[(i, j)]$, si su valor es $0$, no hay ninguna restricción aplicada del vector con posición $i$ y el vector $j$. Si es $1$, entonces es una restricción del tipo `Must-Link`; esto es, deben ir agrupadas en el mismo cluster. Si su entrada es $-1$, ocurre lo contrario al caso anterior: estos dos vectores tienen una restricción del tipo `Cannot-Link`, y deben ir en clusters distintos. Esta estructura de datos nos resultará útil cuando queramos calcular el infeasibility de todo el sistema.
2. La segunda es un *hashmap* para cada tipo de restricción. Dado un cierto índice $i$, los hashmaps `restricciones_ML` y `restricciones_CL` devuelven todos los índices con los que tienen restricciones. Aceleran muchísimo el cálculo del infeasibility generado por la asignación de un cluster a un cierto elemento.

El número de restricciones se guarda al crear la matriz de restricciones.

#### Estadísticos

El interés de este problema reside en ser capaces de crear clusters lo más verosímiles posibles entre sí. Por tanto, para determinar cómo de buena es una solución, necesitamos algún tipo de estadístico que nos informe de ello. Debido a la naturaleza del problema, vamos a considerar dos: El **infeasibility** y el **fitness**.
- **Infeasiblity** es una medida de cuántas restricciones han sido violadas en conjunto; es decir, cuántos elementos con restricción *Cannot-Link* han caído en el mismo cluster, y cuántos vectores con restricción del tipo *Must-Link* se encuentran en clusters separados. La función `infeasibility()` nos permite conocer esto.
Sin embargo, no siempre nos interesa saber cuál es el estado de todo el sistema, sino cómo de malo sería meter un elemento en un cierto cluster. Para esto sirve la función `infeasibility_esperada(indice, cluster)`. Es una forma mucho más rápida de comprobar incrementos y decrementos en el sistema.
- En este problema, tanto las restricciones incumplidas como la distancia entre los elementos son importantes. Por ello, el **fitness** considera ambas. Éste se define de la siguiente manera:
$$
\text{fitness} = \text{desviación general de la partición} + \lambda \cdot \text{infeasiblity}
$$
donde la desviación general de la partición es la media de la suma de las distancias medias intracluster, y $\lambda$ se define como el cociente entre el máximo de las distancias en el sistema y el número de restricciones totales del sistema. Se puede conocer gracias a la función `fitness()`.


* * *


## Algoritmos considerados

Los algoritmos implementados en la práctica 1 son capaces de generar una solución partiendo de un objeto de la clase `Clusters` sin asignaciones. Es el único punto en el que podrán encontrarse en este estado.

Dado que todos los métodos de resolución que programemos requieren aleatoriedad, fijaremos unas semillas para todos los generadores del programas. Se encuentran en la clase `utils.rs/Semillas`.


TODO hay que incluir, en total:
- Pseudocódigo de la estructura del método de búsqueda y operaciones relevantes de cada algoritmo.
- Pseudocódigo de los algoritmos. Los implementados, no los de la teoría.
  - *Incluirá la descripción en pseudocódigo del método de exploración del entorno, el operador de generación de vecino y la generación de soluciones*.
- Pseudocódigo de los algoritmos de comparación.

### Greedy

El algoritmo **Greedy K-medias aplicado a clustering con restricciones** es capaz de proporcionarnos una solución relativamente buena en muy pocos milisegundos. Su implementación es muy sencilla, así como la idea que hay tras éste.

#### Descripción del algoritmo

Partiendo de un cluster vacío, pero con todos los elementos cargados, consideramos una serie de centroides aleatorios. Tantos como número de clusters debamos generar. Recorremos los elementos del espacio de forma aleatoria, de forma que asignamos cada uno al cluster en el que menor número de restricciones se viola (esto es, de menor infeasibility). En caso de empate, se asigna al cluster con cenroide más cercano a nuestro punto, entendiendo por cercano por aquel centroide que minimiza la distancia euclidiana. Se actualizan los centroides, y se repite todo hasta que la solución se estabilice.

El pseudocódigo, por tanto, quedaría así:

```
Greedy_COPKM

1. Generar centroides aleatorios con distribución uniforme en R^d.
2. Barajar los índices de forma aleatoria y sin repetición.
3. Mientras se produzcan cambios en el cluster:
  3.1. Para cada índice, mirar qué incremento supone en la infeasibility al asignarlo a un cluster. Tomar el menor de estos.
  3.2. Actualizar los centroides
```

![Ejemplo de ejecución de greedy](img/Greedy_ejemplo.png)

En la sección [Análisis de resultados](#análisis-de-resultados) comprobaremos cómo de buena es la solución obtenida.

### Búsqueda local

**Búsqueda local** es la primera heurística propia que programaremos. Aunque es un algoritmo sencillo, supone una mejora en ciertos aspectos con respecto a Greedy. Se basa en la exploración de vecinos a la solución actual, tomando el mejor de entre los posibles. Por su naturaleza, suele generar óptimos locales.


#### Descripción del algoritmo

Como hemos citado, se exploran los vecinos de una cierta solución, mirando en cada iteración cuáles son las mejores soluciones. Se define un vecino como la asignación de un elemento $i$ a un cluster $c$ partiendo de una solución actual. Debemos verificar que la posible solución generada con este operador es válida, pues en otro caso, no tiene sentido seguir.

El concepto de *mejor solución* es el que proporciona el fitness. Es decir, en cada iteración, se comprobará si el vecino tiene un fitness menor que el actual. Si es así, la siguiente solución a explorar será esta.

#### Implementación

El pseudocódigo del algoritmo es el siguiente:

```
Generar una solución válida inicial.

Hasta que no se haya alcanzado un óptimo local
├  Guardar la información de la solución actual relevante: fitness, infeasibility,
├  Barajar los índices
│
├ Para i en {0, ..., num_elementos}
│  ├ Barajar los clusters
│  │
│  ├ Para c en {1, ..., num_clusters}
│  │  │ Si el cluster del i-ésimo elemento no es c
│  │  │  ├ Comprobar si el vecino nuevo es válido
│  │  │  ├ En ese caso, comprobar si tiene un fitness menor.
│  │  │  └ Si es así, reexplorar todos los índices de nuevo y actualizar la información de la solución nueva.
│  │  │
└  └  └─ Si se ha encontrado una nueva solución, ignorar el resto de índices.
```


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