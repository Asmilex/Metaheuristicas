# Algoritmos genéticos y meméticos Problema de Agrupamiento con Restricciones

<img src="./img/P2/Portada.jpg" width="808" height="1264" style="zoom:75%;"/>

> **Autor**: Andrés Millán
> **DNI**:
> **Email**: amilmun@correo.ugr.es
> **Grupo de prácticas**: MH3

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
      - [Infeasibility](#infeasibility)
      - [Fitness](#fitness)
- [Conceptos básicos de los algoritmos genéticos y meméticos](#conceptos-básicos-de-los-algoritmos-genéticos-y-meméticos)
- [Operadores](#operadores)
  - [Operadores de selección](#operadores-de-selección)
  - [Operadores de cruce](#operadores-de-cruce)
    - [Cruce uniforme](#cruce-uniforme)
    - [Cruce de segmento fijo](#cruce-de-segmento-fijo)
  - [Operadores de mutación](#operadores-de-mutación)
  - [Reparación](#reparación)
  - [Generación aleatoria de la población inicial](#generación-aleatoria-de-la-población-inicial)
- [Algoritmos genéticos considerados](#algoritmos-genéticos-considerados)
  - [Esquemas de reemplazamiento](#esquemas-de-reemplazamiento)
  - [Implementación](#implementación)
- [Algoritmos meméticos](#algoritmos-meméticos)
  - [Descripción de los meméticos](#descripción-de-los-meméticos)
  - [Implementación](#implementación-1)
- [Análisis de resultados](#análisis-de-resultados)
  - [Descripción de los casos del problema empleados](#descripción-de-los-casos-del-problema-empleados)
  - [Benchmarking y resultados obtenidos](#benchmarking-y-resultados-obtenidos)
  - [Síntesis](#síntesis)
- [Referencias](#referencias)
  - [POR HACER](#por-hacer)


* * *


## Sobre esta memoria

En esta memoria se recoge toda la información necesaria para resolver el problema del **agrupamiento con restricciones**, así como la documentación y el desarrollo de la práctica 2 de la asignatura Metaheurísticas.

Todo el código está subido en el repositorio de Github https://github.com/Asmilex/Metaheuristicas. Para ejecutarlo, es necesario [tener instalado Rust](https://www.rust-lang.org/tools/install). El comprimido de la entrega contendrá los mismos archivos que se encuentran en el repositorio. El único cambio será la fecha de la versión y la localización del PDF generado a partir de este archivo.

Se puede compilar y correr el proyecto con `cargo run --release`. Sin embargo, es necesario especificar ciertos parámetros de entrada, que dependerán de lo que se quiera hacer. Estas son las posibilidades:

#### Benchmark de uno o más algoritmos

Escribir en la línea de comandos `cargo run --release benchmark [algoritmos]`. donde `[algoritmos]` son uno o más elementos de la siguiente lista:
- `greedy`.
- `bl`.
- Algoritmos genéticos
  - Se puede introducir `geneticos` para ejecutarlos todos
  - Alternativamente, se pueden especificar a mano: `agg_un`, `agg_sf`, `age_un` o `age_sf`.
- Algoritmos meméticos:
  - Usando `memeticos` se ejecutarán todos los de esta categoría descritos en esta documentación
  - Alternativamente, se pueden especificar a mano: `am_10_1`, `am_10_01`, `am_10_01_mejores`.

Si no se especifica ninguno, se usarán todos. Cada algoritmo se ejecuta 5 veces por dataset (por lo que cada uno se realiza 30 veces). La información resultante se exportará al archivo `./data/csv/[dataset]_[número de restricciones]/[nombre del algoritmo].csv`, el cual contendrá las medidas necesarias para el posterior análisis que realizaremos.

Por ejemplo, un archivo sería `/data/csv/bupa_10/age_sf.csv`.

#### Ejecutar uno o varios algoritmos para un dataset en particular

Para ejecutar un único algoritmo para un cierto dataset, se debe introducir en la línea de comandos `cargo run --release [dataset] {10, 20} [algoritmos]`, donde `[dataset]` puede valer `bupa`, `glass` o `zoo`, eligiendo qué conjunto de restricciones usar (`10` o `20`). La lista de algoritmos funciona de la misma manera que en el apartado anterior.


* * *


## Descripción del problema

A lo largo de estas prácticas se resolverá el problema del **agrupamiento con restricciones**. Este es una modificación del clásico problema del *clustering*, el cual se describe de la siguiente forma:

Se nos presenta una lista de elementos con un cierto número de atributos. Los representaremos como vectores en $[0, 1]^d$. Debemos agruparlos en un cierto número de categorías, llamados **clústers**, de forma que se minimice la distancia entre estos vectores.

Nuestro matiz consiste en que les pondremos restricciones a los elementos a analizar. De esta forma, forzaremos a que dos vectores deban localizarse en en mismo clúster, o lo contrario; que deban estar en clusters distintos. Por tanto, no solo debemos conseguir una denominada *distancia intraclúster* baja, sino que se debe violar el mínimo número de restricciones posibles.

Durante estas prácticas propondremos diferentes algoritmos para resolver este problema.

En la práctica 1, presentaremos soluciones sencillas basadas en algoritmos simples como **Greedy** o **Búsqueda Local**.

En la práctica 2, la cual es la que vamos a tratar, implementaremos algunas versiones de los algoritmos **genéticos** y **meméticos**.


* * *


## Procedimiento seguido para resolver la práctica

Todo el código está escrito en **Rust**. Es un lenguaje moderno, eficiente y fiable en cuestiones relativas a accesos a memoria. Su elección ha sido puro interés personal.


### Crates usadas

Para facilitar la implementación, se han utilizado una serie de *crates* (nombre que reciben las librerías por parte de los *rustáceos*). Estas son:
- [Naglebra](https://www.nalgebra.org/): una librería de álgebra lineal. Utilizada para operaciones con matrices y vectores.
- [Rand](https://docs.rs/rand/0.8.3/rand/): para los generadores de números aleatorios.
- [Multimap](https://docs.rs/multimap/0.8.3/multimap/index.html): para el almacenamiento eficiente de las listas de restricciones.
- [Csv](https://docs.rs/csv/1.1.6/csv/): para exportar los resultados a `.csv`.
- [Colored](https://docs.rs/colored/2.0.0/colored/): para hacer más bonitas y legibles las salidas a consola.


### Estructura del programa

Se han dividido las funcionalidades clave del programa en distintos ficheros. Estos son:
- `main.rs`: contiene el código relacionado con la realización de una ejecución simple y de un benchmark, así como el *parseo* de argumentos.
- `file_io.rs`: aquí se ubican las funciones relacionadas con entrada/salida de archivos. En particular, la lectura de los ficheros de restricciones y la salida a los archivos csv.
- `utils.rs`: se definen las diferentes estructuras relacionadas con el problema. Por ejemplo, `ParametrosDataset` agrupa los datos necesarios para operar un cierto dataset.
- `algorithm.rs`: todos los algoritmos implementados se encuentran aquí.
- `cluster.rs`: las principales estructuras necesarias para resolver el problema se localizan en este fichero. Específicamente, la clase `Clusters`. Estudiaremos a fondo sus elementos.
- `operator.rs`: en este fichero se definen e implementan los operadores utilizados a partir de la práctica 2.


### La clase `Clusters`

Esta estructura supondrá el grueso de nuestro programa. *Agrupará* toda la información pertinente a la resolución del problema. En las siguientes secciones, describiremos sus elementos. No obstante, omitiremos las funciones de poco interés didáctico


#### Sobre las dimensiones

Necesitaremos tres medidas para generar una solución:

- `num_clusters` representa el número de clústers fijado por el problema.
- `dim_vectores` es el número de atributos del dataset.
- `num_elementos` es el número de vectores o muestras del dataset.


#### Elementos del espacio

La clase conoce en todo momento el conjunto de elementos del dataset que estamos tratando, así como sus distancias respectivas. Los miembros que se encargan de guardar esta información son `espacio` y `distancias` respectivamente. El cálculo de la variable $\lambda$, de la cual hablaremos más tarde, se guarda cuál es el máximo de las distancias al calcular la matriz `distancias`.

Para almacenar los vectores, hemos utilizado un vector de `Nalgebra::DVector`, un tipo de array dinámico con funciones de álgebra lineal. Esto nos será de gran ayuda, pues simplificará las operaciones del espacio vectorial con el que tratamos.


#### Representación de las soluciones

Las soluciones se representan con una lista de enteros, `lista_clusters`, de forma que, para una cierta entrada $i$ de dicha lista:
- Si su valor es $0$, entonces, ese elemento no tiene clúster asignado
- En otro caso, su valor está en el conjunto $\{1, ..., num\_clusters\}$.

Una solución solo se considerará válida si todo clúster tiene al menos un elemento asignado. Si algún elemento ha modificado su clúster, la clase automáticamente lo registra en la estructura `recuento_clusters`, por lo que nos resultará sencillo comprobar cuántos elementos hay en cada uno.


#### Restricciones

Representaremos las restricciones de dos formas distintas:
1. La primera de ellas es mediante una matriz dinámica de *Nalgebra* (`restricciones`) con entradas que toman valores en ${0, 1, -1}$. Para una cierta entrada $[(i, j)]$, si su valor es $0$, no hay ninguna restricción aplicada del vector con posición $i$ y el vector $j$. Si es $1$, entonces es una restricción del tipo `Must-Link`; esto es, deben ir agrupadas en el mismo clúster. Si su entrada es $-1$, ocurre lo contrario al caso anterior: estos dos vectores tienen una restricción del tipo `Cannot-Link`, y deben ir en clústers distintos. Esta estructura de datos nos resultará útil cuando queramos calcular el infeasibility de todo el sistema.
2. La segunda es un *hashmap* para cada tipo de restricción. Dado un cierto índice $i$, los hashmaps `restricciones_ML` y `restricciones_CL` devuelven todos los índices con los que tienen restricciones. Aceleran muchísimo el cálculo del infeasibility generado por la asignación de un clúster a un cierto elemento.

El número de restricciones se guarda al crear la matriz de restricciones.


#### Estadísticos

El interés de este problema reside en ser capaces de crear clústers lo más verosímiles posibles entre sí. Por tanto, para determinar cómo de buena es una solución, necesitamos algún tipo de estadístico que nos informe de ello. Debido a la naturaleza del problema, vamos a considerar dos: El **infeasibility** y el **fitness**.
- **Infeasiblity** es una medida de cuántas restricciones han sido violadas en conjunto; es decir, cuántos elementos con restricción *Cannot-Link* han caído en el mismo clúster, y cuántos vectores con restricción del tipo *Must-Link* se encuentran en clústers separados. La función `infeasibility()` nos permite conocer esto.
Sin embargo, no siempre nos interesa saber cuál es el estado de todo el sistema, sino cómo de malo sería meter un elemento en un cierto clúster. Para esto sirve la función `infeasibility_esperada(indice, clúster)`. Es una forma mucho más rápida de comprobar incrementos y decrementos en el sistema.
- En este problema, tanto las restricciones incumplidas como la distancia entre los elementos son importantes. Por ello, el **fitness** considera ambas. Éste se define de la siguiente manera:
$$
\text{fitness} = \text{desviación general de la partición} + \lambda \cdot \text{infeasiblity}
$$
donde la desviación general de la partición es la media de la suma de las distancias medias intraclúster, y $\lambda$ se define como el cociente entre el máximo de las distancias en el sistema y el número de restricciones totales del sistema. Se puede conocer gracias a la función `fitness()`.

El pseudocódigo de las funciones que calculan estos valores es el siguiente:

##### Infeasibility

```
infeasibility():
    infeasibility = 0
    M = Matriz de restricciones
    n = tamaño de la matriz de restricciones

    Para i en 0 .. n
        Para j en 0 .. n
            Si M[i, j] == 1
                Si solucion[i] != solucion[j]
                    infeasibility++
            Si M[i, j] == -1
                Si solucion[i] == solucion[j]
                    infeasibility++

    infeasibility
```

##### Fitness

```
fitness():
    desviacion_general_particion + lambda() * infeasibility()
```


* * *


## Conceptos básicos de los algoritmos genéticos y meméticos

En esta segunda práctica, realizaremos una versión adaptada de los algoritmos **genéticos** y **meméticos**. Los meméticos tienen como base un algoritmo genético al que se le introduce una fase de mejora de las soluciones presentes en cada generación. Por tanto, presentaremos primero los conceptos básicos para ambos.

Los genéticos introducen un conjunto de soluciones llamadas **cromosomas**. Cada componente de un cromosoma se denomina **gen**. Al conjunto de todos los cromosomas se le conoce como **población**.

La idea de estos algoritmos es que los cromosomas evolucionan, de forma que cada **generación** produce unos descendientes atendiendo a una serie de pasos ordenados: **selección**, **cruce**, **mutación** y **reemplazamiento**.

La forma en la que se realizan estas fases se define mediante los **operadores**.


## Operadores

Los operadores son funciones que reciben una o dos soluciones y producen otra. Cada paso tiene su operador específico, y todos poseen algún componente de aleatoriedad.

En la implementación, únicamente los operadores de cruce se han separado. El resto se encuentran incrustados en el código del algoritmo genético principal.

### Operadores de selección

El operador de selección será un **torneo binario**. Enfrentaremos dos cromosomas para ver quién tiene mejor fitness, y nos quedaremos con esa.

El psedocódigo es el siguiente:

```
operador de selección(p1, p2):
    Si fitness(p1) < fitness(p2)
        p1
    En otro caso
        p2
```

### Operadores de cruce

Los operadores de cruce reciben dos cromosomas, y producen un nuevo hijo a partir de sus genes. En estas prácticas usamos dos tipos:

#### Cruce uniforme

El operador de cruce uniforme coge la mitad de los genes de un padre, la otra mitad de los genes del otro padre y los combina en un hijo.

La implementación es la siguiente:

```
cruce_uniforme(p1, p2):
    descendencia: Vector del mismo tamaño que p1 y p2
    genes_a_copiar: Vector vacío

    Para _ en 0 .. p1.len()/2
        loop
            pos_gen = generar aleatorio en [0, p1.len())
            Si genes_a_copiar no contiene a pos_gen
                genes_a_copiar.push(pos_gen)
                break

    Para i en genes_a_copiar
        descendencia[i] = p1[i]

    Para i en 0 .. descendencia.len()
        Si descendencia[i] == 0
            descendencia[i] = p2[i]

    descendencia
```

#### Cruce de segmento fijo

El operador de cruce de segemento fijo determina un segmento de tamaño aleatorio y un inicio, de forma que copia los genes de p1 desde el inicio hacia delante, empezando por el principio del cromosoma si fuera necesario.

Para el resto de cromosomas sin copiar, se procede de la misma manera que en el cruce uniforme.

Debemos destacar que este operador está claramente sesgado, pues siempre se copian más genes del primer cromosoma que del segundo.

```
cruce_segmento_fijo(p1, p2):
    descendencia: Vector del mismo tamaño que p1 y p2
    inicio_segmento = aleatorio en [0, p1.len())
    tamaño_segmento = aleatorio en [0, p1.len())

    i = inicio_segmento
    copias = 0

    Mientras que copias < tamaño_segmento
        descendencia[i] = p1[i]

        i = (i+1) % p1.len()
        copias = copias + 1

    inicio = min (
        (inicio_segmento + 1)%p1.len(),
        (inicio_segmento + tamano_segmento + 1)%p1.len()
    )
    fin = max (
        (inicio_segmento + 1)%p1.len(),
        (inicio_segmento + tamano_segmento + 1)%p1.len()
    )

    genes_a_copiar: Vector vacío

    Para _ en 0 .. fin - inicio + 1
        loop
            pos_gen = aleatorio en [inicio, fin]

            Si genes_a_copiar no contiene a pos_gen
                genes_a_copiar.push(pos_gen)
                break

    Para i en genes_a_copiar
        descendencia[i] = p1[i]

    Para i en 0 .. descendencia.len()
        Si descendencia[i] == 0
            descendencia[i] = p2[i]

    descendencia
```

### Operadores de mutación

Este operador elige un cromosoma al azar de la población, y muta un gen aleatorio manteniendo la validez de la solución. No obstante, esto es únicamente el operador. La elección de cuántas mutaciones se deben hacer en la población total dependen de una serie de parámetros.

La implementación se verá en el pseudocódigo del algoritmo completo.

### Reparación

A veces, los operadores de cruce no generan soluciones válidas, debido a que se pueden dejar clústers vacíos. Para ello, se le aplica una reparación, descrita por el siguiente pseudocódigo:

```
reparar(hijo, k):
    recuento: Vector de tamaño k inicializado a 0

    Para c en hijo
        recuento[c - 1] = recuento[c-1] + 1

    Para indice en 0 .. recuento.len()
        Si recuento[indice] == 0
            loop
                i = aleatorio en [0, hijo.len())

                Si recuento[hijo[i]-1] > 1
                    recuento[hijo[i]-1]--
                    hijo[i] = indice + 1
                    recuento[indice]++
                    break
```

Recordemos que los clústers toman valores en $[1, k]$.


### Generación aleatoria de la población inicial

La mayor parte de los algoritmos requieren generar una población aleatoria inicial. Para ello, hacemos lo siguiente:

```
Para _ en 0 .. tamano_poblacion
    solucion_inicial: Vector de tamaño numero_genes inicializado a 0

    Mientras que solucion_inicial no sea válida
        Para c en solucion_inicial
            c = aleatorio en [0, k]

    poblacion.push(solucion_inicial)
```

## Algoritmos genéticos considerados

Implementaremos 4 tipos de algoritmos genéticos en total, que surgirán de combinar operadores y modelos de reemplazamiento. Estos son:
- Genético generacional con operador de cruce uniforme (**agg_un**).
- Genético generacional con operador de cruce de segmento fijo (**agg_sf**).
- Genético estacionario con operador de cruce uniforme (**age_un**).
- Genético estacionario con operador de cruce de segmento fijo (**age_sf**).

Todos estos algoritmos dependen de unos determinados parámetros, los cuales son:
- **Tamaño de la población**: cuántos individuos existen al final de un ciclo. Por defecto, consideramos $50$.
- **Número de genes** que tiene un cromosoma. Depende del dataset.
- **Evaluaciones del fitnes máximas**. Por defecto $100000$.
- **Número de cromosomas que se desarrollan**. Lo llamaremos $m$. Depende del esquema de reemplazamiento.
- **Probabilidad de cruce**. Depende del esquema de reemplazamiento.
- **Número de cruces esperado** = `probabilidad del cruce * m / 2`. Consideramos un único cruce al del cromosoma $i$ con $i+1$, así como el del $i+1$ con $i$. Los motivos son de eficiencia, pues en la selección, ya se considera que es aleatoria.
- **Operador de cruce**.
- **Probabilidad de mutación** = `1/número de genes`.
- **Número de mutaciones** = `probabilidad de mutación * m * número de genes`. Mutaremos considerando la población como una matriz, y eligiendo una entrada al azar.

### Esquemas de reemplazamiento

El esquema de reemplazamiento refleja cómo se procesa la generación actual, y cuántos descendientes se generan. Como hemos citado antes, usamos el modelo **generacional** y **estacionario**

El **modelo generacional** considera para el desarrollo de una generación el mismo número de cromosomas que el de la población. En nuestro caso, esto significa que `m = tamaño de la población` y que la probabilidad de cruce es de $0.7$.

El **modelo estacionario** toma dos individuos aleatorios y los procesa, para hacerlos competir por ver si entran en la población.
Por tanto, `m = 2`, y la probabilidad de cruce es de $1$.

### Implementación

Los 4 tipos de genéticos resultarán de cambiar los parámetros de la llamada de la función principal. Por tanto, solo consideraremos el pseudocódigo de ésta.

En la llamada no se ha tenido en cuenta la semilla.

```
genetico(cluster, modelo, operador_cruce):

tamano_poblacion = 50
numero_genes = cluster.num_elementos
max_evaluaciones_fitness = 100_000
m = 2 si modelo == estacionario, tamano_poblacion si m == generacional

probabilidad_cruce = 0.7 si modelo == estacionario, 1 si modelo == generacional
numero_cruces = (probabilidad_cruce * m/2).floor()

probabilidad_mutacion = 1.0/numero_genes
numero_mutaciones = (probabilidad_mutacion * m * numero_genes).ceil()

rango_clusters = distribución uniforme en [1, clusters.num_clusters]
rango_poblacion = distribución uniforme en [0, tamano_poblacion)
rango_m = distribución uniforme en [0, m)
rango_genes = distribución uniforme en [0, numero_genes)

Poblacion: Vector de tamaño tamano_poblacion
fitness_poblacion: Vector de tamaño tamano_poblacion

Generar la población inicial y evaluar su fitness

t = 0
evaluaciones_fitness = 0

Mientras que evaluaciones_fitness < max_evaluaciones_fitness

// ───────────────────────────────────────────────── SELECCION ─────

    p_padres: Vector nuevo vacío
    combate: par de números entero

    Para _ en 0 .. m
        combate = (aleatorio en rango_poblacion, aleatorio en rango_poblacion)

        Si fitness(poblacion[combate.0]) < fitness(poblacion[comabte.1])
            p_padres.push(poblacion[combate.0])
        En otro caso
            p_padres.push(poblacion[combate.1])

// ───────────────────────────────────────────────────── CRUCE ─────
     p_intermedio: Vector nuevo vacío

     cruces_restantes = numero_cruces

     Para i en 0 .. m
        Si cruces_restantes > 0
            hijo: Vector nuevo vacío

            Si i%2 == 0 y i < m
                hijo = operador_cruce(p_padres[i], p_padres[i+1])
            En otro caso
                hijo = operador_cruce(p_padres[i], p_padres[i-1])
                cruces_restantes--

            Si hijo no es un cromosoma válido
                reparar(hijo)

            p_intermedia.push(hijo)

        En otro caso
            p_intermedia.push(p_padres[i])

// ────────────────────────────────────────────────── MUTACION ─────

    p_hijos = p_intermedia
    i = 0

    Para _ en 0 .. numero_mutaciones
        i = aleatorio en rango_m

        loop
            gen_a_mutar = aleatorio en rango_genes

            antiguo_cluster = p_hijos[i][gen_a_mutar]
            p_hijos[i][gen_a_mutar] = aleatorio en rango_clusters

            Si p_hijos[i] no es una solución válida
                p_hijos[i][gen_a_mutar] = antiguo_cluster
            En otro caso
                break

// ─────────────────────────────────────────── REEMPLAZAMIENTO ─────

    Si el modelo es el estacionario
        posicion_peor = 0
        peor_fitness = 0.0

        Para (i, valor) en fitness_poblacion.enumerate()
            Si valor > peor_fitness
                peor_fitness = valor
                posicion_peor = i

        fitness_0 = fitness(p_hijos[0])
        fitness_1 = fitness(p_hijos[1])
        evaluaciones_fitness = evaluaciones_fitness + m

        Si fitness_0 < fitness_1
            poblacion[posicion_peor] = p_hijos[0]
            fitness_poblacion[posicion_peor] = fitness_0
        En otro caso
            poblacion[posicion_peor] = p_hijos[1]
            fitness_poblacion[posicion_peor] = fitness_1

    Si el modelo es el generacional
        posicion_mejor = 0
        mejor_fitness = máximo f64 posible

        Para (i, valor) en fitness_poblacion.enumerate()
            Si valor < mejor_fitness
                mejor_fitness = valor
                posicion_mejor = i

        mejor_cromosoma_antiguo = poblacion[posicion_mejor]
        poblacion = p_hijos

        Para (i, cromosoma) en poblacion.enumerate()
            fitness_poblacion[i] = fitness(cromosoma)

        evaluaciones_fitness = evaluaciones_fitness + m

        posicion_peor = 0
        peor_fitness = 0.0

        Para (i, valor) en fitness_poblacion.enumerate()
            Si valor > peor_fitness
                peor_fitness = valor
                posicion_peor = i

        poblacion[posicion_peor] = mejor_cromosoma_antiguo
        fitness[posicion_peor] = mejor_fitness

    t = t+1

posicion_mejor = 0
mejor_fitness = máximo f64 posible

Para (i, valor) en fitness_poblacion.enumerate()
    Si valor < mejor_fitness
        mejor_fitness = valor
        posicion_mejor = i

cluster.asignar_clusters(poblacion[posicion_mejor])

cluster
```

## Algoritmos meméticos

### Descripción de los meméticos

### Implementación


* * *


## Análisis de resultados

En esta sección discutiremos los resultados obtenidos por ambos algoritmos. Presentaremos los parámetros de los datasets utilizados, las distancias óptimas conocidas de estas, y cuánto se acercan Greedy y Búsqueda Local a esta.

### Descripción de los casos del problema empleados

Los datasets usados reciben el nombre de `Zoo`, `Glass`, y `Bupa`. Los dos primeros presentan una dificultad similar, mientras que el último requiere de un mayor tiempo de cómputo.

|                      | **Zoo** | **Glass** |  **Bupa** |
|----------------------|--------:|----------:|----------:|
| **Atributos**        | `16`    | `9`       | `5`       |
| **Clusters**         | `7`     | `7`       | `16`      |
| **Instancias**       | `101`   | `214`     | `345`     |
| **Distancia óptima** | `0.9048`| `0.36429` | `0.229248`|

Sobre estos conjuntos se ha impuesto un número de restricciones: al 10% y al 20%. Naturalmente, cuantas más restricciones, más costoso resulta computar una solución aceptable, pues el sistema contempla una mayor cantidad de enlaces entre sus elementos.
Los ficheros ubicados en `./data/PAR` guardan la información sobre los datasets.

### Benchmarking y resultados obtenidos

Cada algoritmo se ha ejecutado 5 veces por dataset. Como tenemos 3 datasets y 2 restricciones para cada uno, nos encontramos un total de 30 ejecuciones por algoritmo. Estudiaremos las siguientes medidas:
- **Tasa de infeasibility**: número de restricciones que se incumplen en la solución.
- **Desviación media intraclúster**: mide cómo de cohesionados están nuestros clústers. Cuanto más bajo es este valor, mejor.
- **Agregado**: el fitness de la solución. Cuanto más bajo, mejor.
- **Tiempo de ejecución** (ms).

Se ha utlizado un ordenador con un i7 4790 @ 3.6GHz con turbo a 4Ghz, así como un i5 8250U @ 1.60 GHz con turbo 3.40 GHz.

* * *


### Síntesis


## Referencias

- [El libro oficial de Rust](https://doc.rust-lang.org/book/)
- [Imagen de la portada](https://unsplash.com/photos/eXoXJrOGqG4)- [Documentación de Nalgebra](https://www.nalgebra.org/)
- [Documentación de Multimap](https://docs.rs/multimap/0.8.3/multimap/)
- [StackOverflow](https://stackoverflow.com/)
- Material de teoría y de prácticas


### POR HACER

Cosas que he hecho que tengo que documentar:

- parámetros (agX_Y) + geneticos + (am_10_*) + memeticos
- Cambio en la estructura de la carpeta csv
