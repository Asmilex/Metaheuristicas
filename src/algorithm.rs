use rand::{Rng, SeedableRng, rngs::StdRng, seq::SliceRandom, distributions::Uniform};
use std::time::Instant;

use nalgebra::{DVector};
use colored::*;

use crate::cluster::*;
use crate::utils::*;
use crate::operator::*;


//
// ──────────────────────────────────────────────────────────── I ──────────
//   :::::: P R A C T I C A   1 : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────
//


#[allow(non_snake_case)]
/// # Greedy para clustering con restricciones
/// Pasos a seguir para implementar el algoritmo:
/// 1. Sacar centroides aleatorios. Todos los elementos del espacio se encuentran en [0, 1]x...x[0,1]
/// 2. Barajar los índices para recorrerlos de forma aleatoria sin repetición
/// 3. Mientras se produzcan cambios en el cluster:
///     1. Para cada índice barajado, mirar qué incremento supone en la infeasibility al asignarlo a un cluster. Tomar el menor de estos.
///     2. Actualizar los centroides
pub fn greedy_COPKM (cluster: &mut Clusters, seed: u64) -> &mut Clusters {

    println!("{} Ejecutando greedy_COPKM para el cálculo de los clusters", "▸".cyan());

    // ───────────────────────────────────────────────── 1. CENTROIDES ALEATORIOS ─────

    let mut generador = StdRng::seed_from_u64(seed);
    let rango_uniforme = Uniform::new_inclusive(0.0, 1.0);

    let mut centroides_aleatorios: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_clusters];

    for i in 0 .. centroides_aleatorios.len() {
        for j in 0 .. centroides_aleatorios[i].nrows() {
            centroides_aleatorios[i][(j)] = generador.sample(rango_uniforme);
        }
    }

    cluster.asignar_centroides(centroides_aleatorios);

    // ─────────────────────────────────────────────────────── 2. BARAJAR INDICES ─────

    let mut indices_barajados: Vec<usize> = (0..cluster.num_elementos).collect();
    indices_barajados.shuffle(&mut generador);

    // ─────────────────────────────────────────────────── 3. COMPUTO DEL CLUSTER ─────

    let mut cambios_en_cluster = true;
    let mut iters: usize = 0;

    while cambios_en_cluster {
        iters = iters + 1;

        cambios_en_cluster = false;

        // ─── 3.1 ─────────────────────────────────────────────────────────

        for index in indices_barajados.iter() {

            // Calcular el incremento en infeasibility que produce la asignación de xi a cada cluster cj

            let mut infeasibility_esperada: Vec<u32> = Vec::new();

            for c in 1 ..= cluster.num_clusters {
                infeasibility_esperada.push(cluster.infeasibility_esperada(*index as usize, c));
            }

            let minima_infeasibility = infeasibility_esperada.iter().min().unwrap();    // Al ser la infeasibily actual una constante, aquella que produzca la menor es la que tiene una delta menor con respecto al total.

            let mut distancia_min = f64::MAX;
            let mut best_cluster: usize = 0;

            // De entre las asignaciones que producen menos incremento en infeasiblity, seleccionar la asociada con el centroide mu_j más cercano a xi
            for c in 1 ..= cluster.num_clusters {
                if infeasibility_esperada[c-1] == *minima_infeasibility {
                    let distancia_temp = distancia(&cluster.centroide_cluster(c), &cluster.espacio[(*index as usize)]);
                    if distancia_temp < distancia_min {
                        distancia_min = distancia_temp;
                        best_cluster = c;
                    }
                }
            }

            if best_cluster != 0 && cluster.clusters()[*index as usize] != best_cluster {
                cluster.asignar_cluster_a_elemento(*index as usize, best_cluster);
                cambios_en_cluster = true;
            }
        }


        // ─── 3.2 ─────────────────────────────────────────────────────────

        cluster.calcular_centroides();
    }

    if cluster.solucion_valida() {
        println!("{} Cálculo del cluster finalizado en {} iteraciones {}\n", "▸".cyan() , iters, "✓".green());
        cluster
    }
    else {
        println!("{} Se ha encontrado una solución no válida. Ejecutando de nuevo el algoritmo\n", "✗".red());
        greedy_COPKM(cluster, seed+1)
    }
}


// ────────────────────────────────────────────────────────────────────────────────


/// # Búsqueda local
///  Pasos para implementar este algoritmo:
/// 1. Generar una solución válida inicial. Esto es, aquella en la que los clusters están entre 1 y num_cluster, y no tiene clusters vacíos
/// 2. Recorrer el vecindario hasta que encuentres una solución cuyo fitness se queda por debajo de tu solución actual.
/// El vecindario se debe recorrer de forma `(i, l)`, donde
/// - `i` = índice de la solución
/// - `l` es el cluster nuevo a asignar.
/// Cuando se alcancen el número máximo de iteraciones, o no se consiga minimizar la función objetivo, hemos acabado.
/// La solución óptima es aquella que cumple que no existe otra solución S' tal que f(S) < f(S') para toda otra S
pub fn busqueda_local (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    println!("{} Ejecutando búsqueda local para el cálculo de los clusters", "▸".cyan());

    let now = Instant::now();

    // Parámetros de la ejecución
    let max_iteraciones = 10_000;
    let mut generador = StdRng::seed_from_u64(semilla);

    // ──────────────────────────────────────────────────────────────────────── 1 ─────

    let solucion_inicial = cluster.generar_solucion_aleatoria(&mut generador);
    cluster.asignar_clusters(solucion_inicial.clone());

    // ──────────────────────────────────────────────────────────────────────── 2 ─────

    let mut sol_optima_encontrada: bool;
    let mut sol_nueva_encontrada: bool;
    let mut fitness_actual = cluster.fitness();
    let mut infeasibility_actual = cluster.infeasibility();
    let mut clusters_barajados: Vec<usize> = (1..=cluster.num_clusters).collect();

    for _ in 0..max_iteraciones {
        //let now = Instant::now();
        sol_nueva_encontrada = false;
        sol_optima_encontrada = true;

        let mut indices: Vec<usize> = (0..cluster.num_elementos).collect();
        indices.shuffle(&mut generador);

        for i in indices.iter() {
            clusters_barajados.shuffle(&mut generador);

            for c in clusters_barajados.iter() {
                if *c != cluster.cluster_de_indice(*i) {
                    let posible_fitness_nuevo = cluster.bl_fitness_posible_sol(*i, *c, infeasibility_actual);

                    match posible_fitness_nuevo {
                        Ok(fitness) => {
                            if fitness < fitness_actual {
                                fitness_actual = fitness;
                                infeasibility_actual = infeasibility_actual
                                    - cluster.infeasibility_esperada(*i, cluster.cluster_de_indice(*i))
                                    + cluster.infeasibility_esperada(*i, *c);
                                cluster.asignar_cluster_a_elemento(*i, *c);
                                sol_nueva_encontrada = true;
                                break;
                            }
                        },
                        Err(_r) => {}
                    };
                }
            }


            if sol_nueva_encontrada {
                sol_optima_encontrada = false;
                break;
            }
        }

        if sol_optima_encontrada{
            break;
        }
    }

    println!("{} Cálculo del cluster finalizado en {} ms {}\n", "▸".cyan(), now.elapsed().as_millis(),  "✓".green());

    cluster
}


//
// ──────────────────────────────────────────────────────────── II ──────────
//   :::::: P R A C T I C A   2 : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────
//

//
// ──────────────────────────────────────────────────────────────── GÉNETICOS ─────
//

/// Pasos:
/// 1. Inicializar variables:
///     - Inicializar una población P(0)
///     - Evaluar P(0)
/// 2. Bucle principal
///     2.1 Seleccionar nueva población desde la anterior P(t-1). Sea P_padres ésta. El operador de selección es torneo binario. Otras consideraciones:
///         - En el modelo generacional, el tamaño de P_padres es el mismo que el de la población inicial => se hacen 50 torneos.
///         - En el modelo estacionario, el tamaño de P_padres es 2 => 2 torneos.
///     2.2 Cruzar P_padres y guardarlo en P_intermedia.
///         Como la selección ya tiene una componente aleatoria, fijamos una probabilidad de cruce (P_c) y únicamente hacemos los siguientes cruces:
///             Número de cruces = P_c * Tamaño de la población / 2.
///         Los tomamos por orden: primero con el segundo, tercero con el cuarto...
///         Cuando agotemos el número de cruces, copiamos el resto de elementos tal cual a P_intermedia
///     2.3 Mutar P_intermedia con probabilidad p_m y guardarlo en P_hijos
///         La mutación es uniforme. Fijamos un número de mutaciones = prob_mutacion * m * número de genes.
///     2.4 Reemplazar la población P(t) a partir de P(t-1) y P_hijos.
///         - En el modelo generacional, se mantiene el mejor individuo de P(t-1).
///         - En el modelo estacionario, los dos hijos que se encuentran en P_hijos compiten para entrar en P(t). Quitamos el peor de la antigua
///     2.5 Evaluar P(t).

fn genetico (cluster: &mut Clusters, modelo: ModeloGenetico, op_cruce_a_usar: Operadores, semilla: u64) -> &mut Clusters {
    println!("{} Ejecutando algoritmo genético {:?} con operador de cruce {:?} para el cálculo de los clusters", "▸".cyan(), modelo, op_cruce_a_usar);
    let now = Instant::now();

    // ─────────────────────────────────────────────────── DECISION DE PARAMETROS ─────

    let tamano_poblacion = 50;
    let numero_genes = cluster.num_elementos;
    let max_evaluaciones_fitness = 100_000;
    let m = match modelo {    // Sigo notación de las diapositivas
        ModeloGenetico::Estacionario => 2,
        ModeloGenetico::Generacional => tamano_poblacion
    };

    let probabilidad_cruce = match modelo {
        ModeloGenetico::Generacional => 0.7,
        ModeloGenetico::Estacionario => 1.0
    };

    // NOTE contamos el cruce de i, i+1 e i+1, i como uno solo
    let numero_cruces:i32 = (probabilidad_cruce * m as f64/2.0).floor() as i32;
    let operador_cruce = match op_cruce_a_usar {
        Operadores::Uniforme => cruce_uniforme,
        Operadores::SegmentoFijo => cruce_segmento_fijo
    };

    let probabilidad_mutacion = 0.1/numero_genes as f64;
    let numero_mutaciones = (probabilidad_mutacion * m as f64 * numero_genes as f64).ceil() as i64;

    let mut generador = StdRng::seed_from_u64(semilla);
    let rango_clusters = Uniform::new_inclusive(1, cluster.num_clusters);
    let rango_poblacion = Uniform::new(0, tamano_poblacion);
    let rango_m = Uniform::new(0, m);
    let rango_genes = Uniform::new(0, numero_genes);


    // ───────────────────────────────────────────── 1. GENERAR POBLACION INICIAL ─────


    // NOTE representaremos la población como un vector de soluciones.
    // De forma paralela, llevaremos un recuento del fitness que producen.
    let mut poblacion = Vec::new();
    let mut fitness_poblacion = Vec::new();

    for _ in 0 .. tamano_poblacion {
        let solucion_inicial= cluster.generar_solucion_aleatoria(&mut generador);
        fitness_poblacion.push(cluster.fitness_externa(&solucion_inicial));
        poblacion.push(solucion_inicial);
    }


    // ─────────────────────────────────────────────────────── 2. BUCLE PRINCIPAL ─────


    let mut t = 0;
    let mut evaluaciones_fitness = 0;

    while evaluaciones_fitness < max_evaluaciones_fitness {
        //println!("\t{} Comienza generación {}", "▸".cyan(), t);

        // ───────────────────────────────────────────────── SELECCION ─────

        let mut p_padres = Vec::new();
        let mut combate; // Los enfrentamientos se harán del `i` vs `i+1`. Se guardan como (combatiente 1, combatiente 2)

        // Crear cuadro de combatientes
        for _ in 0 .. m {
            combate = (generador.sample(rango_poblacion), generador.sample(rango_poblacion));

            if fitness_poblacion[combate.0] < fitness_poblacion[combate.1] {
                p_padres.push(poblacion[combate.0].clone());
            }
            else {
                p_padres.push(poblacion[combate.1].clone());
            }
        }

        // ───────────────────────────────────────────────────── CRUCE ─────

        // No tiramos random para ver si se mete o no. Lo que hacemos es calcular la esperanza,
        // y cruzar padre_i, padre_(i+1) así como padre_(i+1), padre_i hasta completar los que debemos.
        // Cuando hayamos agotado todos los cruces, copiamos el resto tal cual.

        let mut p_intermedia = Vec::new();

        // NOTE ver nota de más arriba sobre el número de cruces
        let mut cruces_restantes = numero_cruces;

        for i in 0 .. m {
            if cruces_restantes > 0 {
                let mut hijo;

                if i % 2 == 0 && i < m {     // Pares => cruzar i con i+1
                    hijo = operador_cruce(&p_padres[i], &p_padres[i+1], &mut generador);
                }
                else {                      // Impares => cruzar i con i-1
                    hijo = operador_cruce(&p_padres[i], &p_padres[i-1], &mut generador);
                    cruces_restantes = cruces_restantes - 1;

                    // Restamos solo aquí porque consideramos que hemos cruzado los dos necesarios para que cuente un cruce
                    // Suena confuso pero yo no lo he elegido. Las culpas al guión y al seminario.
                }

                if !cluster.solucion_valida_externa(&hijo) {
                    reparar(&mut hijo, cluster.num_clusters, &mut generador);
                }

                p_intermedia.push(
                    hijo
                );

            }
            else {
                p_intermedia.push(
                    p_padres[i].clone()
                );
            }
        }



        // ────────────────────────────────────────────────── MUTACION ─────

        // Elegimos un cromosoma aleatoriamente, y después, lo mutamos uniformemente

        let mut p_hijos = p_intermedia;

        let mut i: usize;

        for _ in 0 .. numero_mutaciones {
            i = generador.sample(rango_m);

            loop {
                let gen_a_mutar = generador.sample(rango_genes);

                let antiguo_cluster = p_hijos[i][gen_a_mutar];
                p_hijos[i][gen_a_mutar] = generador.sample(rango_clusters);

                if cluster.solucion_valida_externa(&p_hijos[i]) {
                    break;
                }
                else {
                    p_hijos[i][gen_a_mutar] = antiguo_cluster;
                }
            }
        }

        // ─────────────────────────────────────────── REEMPLAZAMIENTO ─────

        match modelo {
            ModeloGenetico::Estacionario => {
                // Hacemos que luchen para ver quién entra. Nos quedamos con el mejor de los dos
                // En la población, quitaremos de en medio al que peor rendía
                for i in 0 .. m {
                    fitness_poblacion.push(cluster.fitness_externa(&p_hijos[i]));
                    poblacion.push(p_hijos[i].clone());
                    evaluaciones_fitness = evaluaciones_fitness + 1;
                }

                // Ordenar atendiendo al fitness
                for i in 0..fitness_poblacion.len() {
                    for j in 0..fitness_poblacion.len() - i - 1 {
                        if fitness_poblacion[j + 1] < fitness_poblacion[j] {
                            fitness_poblacion.swap(j, j + 1);
                            poblacion.swap(j, j+1);
                        }
                    }
                }

                // Quitar los m peores elementos
                for _ in 0 .. m {
                    poblacion.pop();
                    fitness_poblacion.pop();
                }
            },
            ModeloGenetico::Generacional => {
                // Calculamos el fitness de la nueva población de hijos.
                // El peor nos lo quitamos de en medio, y mantenemos el mejor de lapoblación antigua.

                let mut posicion_mejor: usize = 0;
                let mut mejor_fitness = f64::MAX;

                for (i, valor) in fitness_poblacion.iter().enumerate() {
                    if *valor < mejor_fitness {
                        mejor_fitness = *valor;
                        posicion_mejor = i;
                    }
                }

                let mejor_cromosoma_antiguo = poblacion[posicion_mejor].clone();
                poblacion = p_hijos;

                for (i, cromosoma) in poblacion.iter().enumerate() {
                    fitness_poblacion[i] = cluster.fitness_externa(cromosoma);
                }
                evaluaciones_fitness = evaluaciones_fitness + m;

                // Miramos cuál es el peor de la población, y nos lo cargamos
                let mut posicion_peor: usize = 0;
                let mut peor_fitness = 0.0;

                for (i, valor) in fitness_poblacion.iter().enumerate() {
                    if *valor > peor_fitness {
                        peor_fitness = *valor;
                        posicion_peor = i;
                    }
                }

                poblacion[posicion_peor] = mejor_cromosoma_antiguo;
                fitness_poblacion[posicion_peor] = mejor_fitness;
            }
        }

        //println!("\t{} Generación {} finalizada en {}", "▸".cyan(), t, now.elapsed().as_millis());

        t = t+1;
        //println!("\tPeor fitness: {}; mejor fitness: {}", fitness_poblacion.iter().cloned().fold(0./0., f64::max), fitness_poblacion.iter().cloned().fold(0./0., f64::min));
    }

    // Seleccionamos el mejor cromosoma
    let mut posicion_mejor: usize = 0;
    let mut mejor_fitness = f64::MAX;

    for (i, valor) in fitness_poblacion.iter().enumerate() {
        if *valor < mejor_fitness {
            mejor_fitness = *valor;
            posicion_mejor = i;
        }
    }

    cluster.asignar_clusters(poblacion[posicion_mejor].clone());

    println!("{} Cálculo del cluster finalizado en {} ms {}\n", "▸".cyan(), now.elapsed().as_millis(),  "✓".green());

    cluster
}

// ────────────────────────────────────────────────────────────────────────────────

pub fn agg_un (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    genetico(cluster, ModeloGenetico::Generacional, Operadores::Uniforme, semilla)
}

pub fn agg_sf (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    genetico(cluster, ModeloGenetico::Generacional, Operadores::SegmentoFijo, semilla)
}

pub fn age_un (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    genetico(cluster, ModeloGenetico::Estacionario, Operadores::Uniforme, semilla)
}

pub fn age_sf (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    genetico(cluster, ModeloGenetico::Estacionario, Operadores::SegmentoFijo, semilla)
}


//
// ──────────────────────────────────────────────────────────────── MÉMETICOS ─────
//


fn busqueda_local_suave(solucion: &mut Vec<usize>, cluster: &mut Clusters, fallos_permitidos: usize, generador: &mut StdRng) -> usize {
    let mut indices_barajados: Vec<usize> = (0..solucion.len()).collect();
    indices_barajados.shuffle(generador);

    let mut fallos = 0;
    let mut mejora = true;
    let mut i = 0;
    let mut evaluaciones_fitness: usize = 0;

    while (mejora || fallos < fallos_permitidos) && i < solucion.len() {
        mejora = false;

        // Asignar el mejor valor posible a solucion[indice de indices_barajados]
        // Es decir, asignar la instancia indices_barajados[i] al cluster que minimice el fitness

        let mut mejor_fitness = cluster.fitness_externa(solucion);
        let mut mejor_cluster = solucion[indices_barajados[i]];

        for c in 1 ..= cluster.num_clusters {
            if c != mejor_cluster {     // Evitar comprobar el cluster que ya venía
                solucion[indices_barajados[i]] = c;

                if cluster.solucion_valida_externa(solucion) {
                    let fitness_actual = cluster.fitness_externa(solucion);
                    evaluaciones_fitness = evaluaciones_fitness + 1;

                    if fitness_actual < mejor_fitness {
                        mejor_cluster = c;
                        mejor_fitness = fitness_actual;
                        mejora = true;
                    }
                    else {
                        solucion[indices_barajados[i]] = mejor_cluster;
                    }
                }
                else {
                    solucion[indices_barajados[i]] = mejor_cluster;
                }
            }
        }

        if !mejora {
            fallos = fallos + 1;
        }

        i = i + 1;
    }

    evaluaciones_fitness
}

/// # Algoritmo memético
/// Parámetros:
/// - **Periodo generacional**: cada cuántas generaciones se aplica la búsqueda local
/// - **Probabilidad**: En `[0, 1]`. Indica cuál es la probabilidad de aplicar la búsqueda local a un cromosoma
/// - **Solo_a_mejores**: si está activado, la búsqueda local solo se aplica a los `probabilidad * tamaño de la población` mejores cromosomas.
fn memetico (cluster: &mut Clusters, periodo_generacional: usize, probabilidad: f64, solo_a_mejores: bool, semilla: u64) -> &mut Clusters {
    /*
        TODO
        Realmente, la implementación debería usar la función agg_un, pero por motivos de comodidad/tiempo/pereza,
        voy a hacer copy - paste del genético usando únicamente el modelo generacional con operador de cruce uniforme,
        que es el que mejores resultados produce.

        Si el Andrés del futuro va menos agobiado con el tiempo, y tiene ganas, le dejo propuesto como ejercicio
        solucionar este problema y refactorizar la función (?)
    */


    // ─────────────────────────────────────────────────── DECISION DE PARÁMETROS ─────

    let tamano_poblacion = 50;
    let numero_genes = cluster.num_elementos;
    let max_evaluaciones_fitness = 100_000;
    let m = tamano_poblacion;

    let probabilidad_cruce = 0.7;
    let numero_cruces:i32 = (probabilidad_cruce * m as f64/2.0).floor() as i32;
    let operador_cruce = cruce_uniforme;

    let probabilidad_mutacion = 0.1/numero_genes as f64;
    let numero_mutaciones = (probabilidad_mutacion * m as f64 * numero_genes as f64).ceil() as i64;

    let fallos_maximos = (0.1 * numero_genes as f64).floor() as usize;

    if probabilidad != 0.1 && solo_a_mejores {
        println!("{}: este algoritmo no está pensado para ejecutarse con estos parámetros de entrada", "WARNING".red());
    }

    let mut generador = StdRng::seed_from_u64(semilla);
    let rango_clusters = Uniform::new_inclusive(1, cluster.num_clusters);
    let rango_0_1 = Uniform::new_inclusive(0.0, 1.0);
    let rango_poblacion = Uniform::new(0, tamano_poblacion);
    let rango_m = Uniform::new(0, m);
    let rango_genes = Uniform::new(0, numero_genes);


    // ───────────────────────────────────────────── 1. GENERAR POBLACION INICIAL ─────

    println!("{} Ejecutando algoritmo memético de base agg_un para el cálculo de los clusters", "▸".cyan());

    let mut poblacion = Vec::new();
    let mut fitness_poblacion = Vec::new();

    let now = Instant::now();


    // TODO usar la implementación de la solución aleatoria que se encuentra en la clase cluster.
    for _ in 0 .. tamano_poblacion {
        let solucion_inicial = cluster.generar_solucion_aleatoria(&mut generador);
        fitness_poblacion.push(cluster.fitness_externa(&solucion_inicial));
        poblacion.push(solucion_inicial);
    }


    // ─────────────────────────────────────────────────────── 2. BUCLE PRINCIPAL ─────


    let mut t = 0;  // Generaciones
    let mut evaluaciones_fitness = 0;

    while evaluaciones_fitness < max_evaluaciones_fitness {
        //println!("\t{} Comienza generación {}", "▸".cyan(), t);

        // ───────────────────────────────────────── EXPLORACION LOCAL ─────

        if t % periodo_generacional == 0 && t > 0 {             // Evitamos la primera generación. Creo que merece más la pena explorar más tarde
            if solo_a_mejores {
                let busquedas_totales = (probabilidad * poblacion.len() as f64).floor() as usize;

                // Necesitamos ordenar de menor a mayor para ver quiénes son los mejores.
                // Los mejores se encuentran al principio del vector
                for i in 0..fitness_poblacion.len() {
                    for j in 0..fitness_poblacion.len() - i - 1 {
                        if fitness_poblacion[j + 1] < fitness_poblacion[j] {
                            fitness_poblacion.swap(j, j + 1);
                            poblacion.swap(j, j+1);
                        }
                    }
                }

                for i in 0 .. busquedas_totales {
                    let evaluaciones = busqueda_local_suave(&mut poblacion[i], cluster, fallos_maximos, &mut generador);
                    fitness_poblacion[i] = cluster.fitness_externa(&poblacion[i]);
                    evaluaciones_fitness = evaluaciones_fitness + evaluaciones;
                }
            }
            else {
                for i in 0 .. tamano_poblacion {
                    if generador.sample(rango_0_1) <= probabilidad {
                        let evaluaciones = busqueda_local_suave(&mut poblacion[i], cluster, fallos_maximos, &mut generador);
                        fitness_poblacion[i] = cluster.fitness_externa(&poblacion[i]);
                        evaluaciones_fitness = evaluaciones_fitness + evaluaciones;
                    }
                }
            }
        }

        // ───────────────────────────────────────────────── SELECCION ─────

        let mut p_padres = Vec::new();
        let mut combate; // Los enfrentamientos se harán del `i` vs `i+1`. Se guardan como (combatiente 1, combatiente 2)

        // Crear cuadro de combatientes
        for _ in 0 .. m {
            combate = (generador.sample(rango_poblacion), generador.sample(rango_poblacion));

            if fitness_poblacion[combate.0] < fitness_poblacion[combate.1] {
                p_padres.push(poblacion[combate.0].clone());
            }
            else {
                p_padres.push(poblacion[combate.1].clone());
            }
        }

        // ───────────────────────────────────────────────────── CRUCE ─────

        // No tiramos random para ver si se mete o no. Lo que hacemos es calcular la esperanza,
        // y cruzar padre_i, padre_(i+1) así como padre_(i+1), padre_i hasta completar los que debemos.
        // Cuando hayamos agotado todos los cruces, copiamos el resto tal cual.

        let mut p_intermedia = Vec::new();

        let mut cruces_restantes = numero_cruces;

        for i in 0 .. m {
            if cruces_restantes > 0 {
                let mut hijo;

                if i % 2 == 0 && i < m {     // Pares => cruzar i con i+1
                    hijo = operador_cruce(&p_padres[i], &p_padres[i+1], &mut generador);
                }
                else {                      // Impares => cruzar i con i-1
                    hijo = operador_cruce(&p_padres[i], &p_padres[i-1], &mut generador);
                }

                if !cluster.solucion_valida_externa(&hijo) {
                    reparar(&mut hijo, cluster.num_clusters, &mut generador);
                }

                p_intermedia.push(
                    hijo
                );

                cruces_restantes = cruces_restantes - 1;
            }
            else {
                p_intermedia.push(
                    p_padres[i].clone()
                );
            }
        }

        // ────────────────────────────────────────────────── MUTACION ─────

        // Elegimos un cromosoma aleatoriamente, y después, lo mutamos uniformemente

        let mut p_hijos = p_intermedia;

        let mut i: usize;

        for _ in 0 .. numero_mutaciones {
            i = generador.sample(rango_m);

            loop {
                let gen_a_mutar = generador.sample(rango_genes);

                let antiguo_cluster = p_hijos[i][gen_a_mutar];
                p_hijos[i][gen_a_mutar] = generador.sample(rango_clusters);

                if cluster.solucion_valida_externa(&p_hijos[i]) {
                    break;
                }
                else {
                    p_hijos[i][gen_a_mutar] = antiguo_cluster;
                }
            }
        }

        // ─────────────────────────────────────────── REEMPLAZAMIENTO ─────

        // Calculamos el fitness de la nueva población de hijos.
        // El peor nos lo quitamos de en medio, y mantenemos el mejor de lapoblación antigua.

        let mut posicion_mejor: usize = 0;
        let mut mejor_fitness = f64::MAX;

        for (i, valor) in fitness_poblacion.iter().enumerate() {
            if *valor < mejor_fitness {
                mejor_fitness = *valor;
                posicion_mejor = i;
            }
        }

        let mejor_cromosoma_antiguo = poblacion[posicion_mejor].clone();
        poblacion = p_hijos;

        for (i, cromosoma) in poblacion.iter().enumerate() {
            fitness_poblacion[i] = cluster.fitness_externa(cromosoma);
        }
        evaluaciones_fitness = evaluaciones_fitness + m;

        // Miramos cuál es el peor de la población, y nos lo cargamos
        let mut posicion_peor: usize = 0;
        let mut peor_fitness = 0.0;

        for (i, valor) in fitness_poblacion.iter().enumerate() {
            if *valor > peor_fitness {
                peor_fitness = *valor;
                posicion_peor = i;
            }
        }

        poblacion[posicion_peor] = mejor_cromosoma_antiguo;
        fitness_poblacion[posicion_peor] = mejor_fitness;


        //println!("\t{} Generación {} finalizada en {}", "▸".cyan(), t, now.elapsed().as_millis());

        t = t+1;
        //println!("\tPeor fitness: {}; mejor fitness: {}", fitness_poblacion.iter().cloned().fold(0./0., f64::max), fitness_poblacion.iter().cloned().fold(0./0., f64::min));
    }

    // Seleccionamos el mejor cromosoma
    let mut posicion_mejor: usize = 0;
    let mut mejor_fitness = f64::MAX;

    for (i, valor) in fitness_poblacion.iter().enumerate() {
        if *valor < mejor_fitness {
            mejor_fitness = *valor;
            posicion_mejor = i;
        }
    }

    cluster.asignar_clusters(poblacion[posicion_mejor].clone());

    println!("{} Cálculo del cluster finalizado en {} ms {}\n", "▸".cyan(), now.elapsed().as_millis(),  "✓".green());

    cluster
}

// ────────────────────────────────────────────────────────────────────────────────

// NOTE está feo tener funciones fijadas para los parámetros, pero bueno. Los requisitos de la práctica

pub fn am_10_1 (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    memetico(cluster, 10, 1.0, false, semilla)
}

pub fn am_10_01 (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    memetico(cluster, 10, 0.1, false, semilla)
}

pub fn am_10_01_mejores (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    memetico(cluster, 10, 0.1, true, semilla)
}


//
// ──────────────────────────────────────────────────────────── III ──────────
//   :::::: P R A C T I C A   3 : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────────────────
//

/// Intensificador basado en la técnica del enfriamiento simulado
#[allow(non_snake_case)]
pub fn enfriamiento_simulado_aplicado (solucion: &Vec<usize>, cluster: &mut Clusters, max_evaluaciones: usize, generador: &mut StdRng) -> Vec<usize> {
    //
    // ─────────────────────────────────────────────────────────────── PARAMETROS ─────
    //

    let uniforme_0_1 = Uniform::new(0.0, 1.0);

    let mut solucion_actual = solucion.clone();
    let mut fitness_actual = cluster.fitness_externa(&solucion_actual);

    let mut mejor_solucion = solucion_actual.clone();
    let mut mejor_fitness = fitness_actual;

    let max_vecinos = 10 * cluster.num_elementos;
    let max_exitos = (0.1 * max_vecinos as f64).ceil() as usize;

    let mu: f64 = 0.3;
    let phi: f64 = 0.3;

    let T0 = mu * mejor_fitness/-phi.ln();
    let Tf = 0.001;
    let k = 0.1;

    let M = max_evaluaciones/max_vecinos;

    //
    // ────────────────────────────────────────────────────────── BUCLE PRINCIPAL ─────
    //

    let mut T = T0;
    let mut evaluaciones_fitness = 0;
    let mut num_exitos = 1; // Para que entre. Este valor ahora mismo no sirve para nada más.

    // Condiciones:
    // 1. Temperatura mayor que la final. Poco a poco se va enfriando, por lo que debemos parar al llegar al final
    // 2. No nos hemos pasado de las máximas evaluaciones fijadas
    // 3. Se ha conseguido generar al menos una nueva solución
    while T > Tf && evaluaciones_fitness < max_evaluaciones && num_exitos > 0 {
        let mut vecinos_generados = 0;
        num_exitos = 0;

        while vecinos_generados < max_vecinos && num_exitos < max_exitos {
            let vecino = generar_vecino(&solucion_actual, cluster, generador);
            vecinos_generados = vecinos_generados + 1;

            let fitness_vecino = cluster.fitness_externa(&vecino);
            let delta = fitness_vecino - fitness_actual;
            evaluaciones_fitness = evaluaciones_fitness + 1;

            if delta < 0.0 || generador.sample(uniforme_0_1) <= (-delta/(k * T)).exp() {
                solucion_actual = vecino;
                fitness_actual = fitness_vecino;

                num_exitos = num_exitos + 1;

                if fitness_actual < mejor_fitness {
                    mejor_solucion = solucion_actual.clone();
                    mejor_fitness = fitness_actual;
                }
            }
        }

        T = enfriar(T, M, T0, Tf);
    }

    mejor_solucion
}


pub fn enfriamiento_simulado(cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    println!("{} Ejecutando enfriamiento simulado para el cálculo de los clusters", "▸".cyan());
    let now = Instant::now();

    let mut generador = StdRng::seed_from_u64(semilla);

    let max_evaluaciones = 100_000;

    let mejor_solucion = enfriamiento_simulado_aplicado (
        &cluster.generar_solucion_aleatoria(&mut generador),
        cluster,
        max_evaluaciones,
        &mut generador
    );

    println!("{} Cálculo del cluster finalizado en {} ms {}\n", "▸".cyan(), now.elapsed().as_millis(),  "✓".green());

    cluster.asignar_clusters(mejor_solucion);
    cluster
}

/// # Esquema de enfriamiento
/// Actualmente, se utiliza el de Cauchy.
#[allow(non_snake_case)]
fn enfriar(T: f64, M: usize, T0: f64, Tf: f64) -> f64 {
    let beta = (T0 - Tf)/(M as f64 * T0 * Tf);

    T/(1.0 + beta * T)
}


fn generar_vecino(s: &Vec<usize>, cluster: &Clusters, generador: &mut StdRng) -> Vec<usize> {
    let mut vecino = s.clone();
    let rango_clusters = Uniform::new_inclusive(1, cluster.num_clusters);
    let rango_indices = Uniform::new(0, vecino.len());

    let mut i: usize;
    let mut c: usize;
    let mut antiguo_cluster: usize;

    loop {
        i = generador.sample(rango_indices);
        c = generador.sample(rango_clusters);

        antiguo_cluster = vecino[i];
        vecino[i] = c;

        match cluster.solucion_valida_externa(&vecino) {
            true => break vecino,
            false => vecino[i] = antiguo_cluster
        }
    }
}


// ────────────────────────────────────────────────────────────────────────────────


pub fn busqueda_multiarranque_basica(cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    println!("{} Ejecutando búsqueda multiarranque básica para el cálculo de los clusters", "▸".cyan());
    let now = Instant::now();

    let mut generador = StdRng::seed_from_u64(semilla);

    cluster.asignar_clusters(cluster.generar_solucion_aleatoria(&mut generador));   // Problema técnico: BL requiere de una solución de partida. La inicializo aleatoriamente.
    let mut mejor_solucion = vec![0; cluster.num_elementos];    // No se usa para nada. Si no se asigna ninguno, fallará
    let mut mejor_fitness = f64::MAX;

    let soluciones_a_generar = 10;
    let iteraciones_maximas = 10_000;

    for _ in 0 .. soluciones_a_generar {
        let solucion = cluster.generar_solucion_aleatoria(&mut generador);
        let solucion = busqueda_local_bmb(&solucion, cluster, iteraciones_maximas, &mut generador);

        let fitness = cluster.fitness_externa(&solucion);

        if fitness < mejor_fitness {
            mejor_solucion = solucion;
            mejor_fitness = fitness;
        }
    }

    println!("{} Cálculo del cluster finalizado en {} ms {}\n", "▸".cyan(), now.elapsed().as_millis(),  "✓".green());

    cluster.asignar_clusters(mejor_solucion);
    cluster
}


fn busqueda_local_bmb(solucion: &Vec<usize>, cluster: &mut Clusters, iteraciones_maximas: usize, generador: &mut StdRng) -> Vec<usize> {
    // TODO cambiar cómo se gestiona la búsqueda local. Funcionamiento interno. Shit happens.
    // Por la implementación actual, BL trabaja muy cercano a la sol. interna del clúster.
    // Como no es mi intención modificarlo aquí, voy a darle la nueva, para restaurar después la antigua.

    let antigua_sol = cluster.clusters().clone();
    cluster.asignar_clusters(solucion.clone());

    let mut fitness_actual = cluster.fitness();
    let mut infeasibility_actual = cluster.infeasibility();

    let mut clusters_barajados: Vec<usize> = (1..=cluster.num_clusters).collect();

    let mut sol_optima_encontrada: bool;
    let mut sol_nueva_encontrada: bool;

    for _ in 0..iteraciones_maximas {
        sol_nueva_encontrada = false;
        sol_optima_encontrada = true;

        let mut indices: Vec<usize> = (0..cluster.num_elementos).collect();
        indices.shuffle(generador);

        for i in indices.iter() {
            clusters_barajados.shuffle(generador);

            for c in clusters_barajados.iter() {
                if *c != cluster.cluster_de_indice(*i) {
                    let posible_fitness_nuevo = cluster.bl_fitness_posible_sol(*i, *c, infeasibility_actual);

                    match posible_fitness_nuevo {
                        Ok(fitness) => {
                            if fitness < fitness_actual {
                                fitness_actual = fitness;
                                infeasibility_actual = infeasibility_actual
                                    - cluster.infeasibility_esperada(*i, cluster.cluster_de_indice(*i))
                                    + cluster.infeasibility_esperada(*i, *c);
                                cluster.asignar_cluster_a_elemento(*i, *c);
                                sol_nueva_encontrada = true;
                                break;
                            }
                        },
                        Err(_r) => {}
                    };
                }
            }


            if sol_nueva_encontrada {
                sol_optima_encontrada = false;
                break;
            }
        }

        if sol_optima_encontrada{
            break;
        }
    }

    let s = cluster.clusters().clone();
    cluster.asignar_clusters(antigua_sol);

    s
}


// ────────────────────────────────────────────────────────────────────────────────


pub fn busqueda_local_reiterada(cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    println!("{} Ejecutando búsqueda multiarranque reiterada para el cálculo de los clusters", "▸".cyan());
    let now = Instant::now();

    println!("{} Cálculo del cluster finalizado en {} ms {}\n", "▸".cyan(), now.elapsed().as_millis(),  "✓".green());

    cluster
}


// ────────────────────────────────────────────────────────────────────────────────


pub fn hibrido_ils_es(cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    println!("{} Ejecutando el híbrido ILS-ES para el cálculo de los clusters", "▸".cyan());
    let now = Instant::now();

    println!("{} Cálculo del cluster finalizado en {} ms {}\n", "▸".cyan(), now.elapsed().as_millis(),  "✓".green());

    cluster
}