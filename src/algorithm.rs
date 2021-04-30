use rand::{Rng, SeedableRng, rngs::StdRng, seq::SliceRandom};

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

    let mut rng = StdRng::seed_from_u64(seed);

    let mut centroides_aleatorios: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_clusters];

    for i in 0 .. centroides_aleatorios.len() {
        for j in 0 .. centroides_aleatorios[i].nrows() {
            centroides_aleatorios[i][(j)] = rng.gen();
        }
    }

    cluster.asignar_centroides(centroides_aleatorios);

    // ─────────────────────────────────────────────────────── 2. BARAJAR INDICES ─────

    let mut indices_barajados: Vec<usize> = (0..cluster.num_elementos).collect();
    indices_barajados.shuffle(&mut rng);

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
    use std::time::{Instant};

    println!("{} Ejecutando búsqueda local para el cálculo de los clusters", "▸".cyan());

    let now = Instant::now();

    // Parámetros de la ejecución
    let max_iteraciones = 10_000;
    let mut generador = StdRng::seed_from_u64(semilla);

    // ──────────────────────────────────────────────────────────────────────── 1 ─────

    let mut solucion_inicial: Vec<usize> = vec![0; cluster.num_elementos];

    while !cluster.solucion_valida_externa(&solucion_inicial) {
        for c in solucion_inicial.iter_mut() {
            *c = generador.gen_range(1..=cluster.num_clusters);
        }
    }

    cluster.asignar_clusters(solucion_inicial.clone());

    // ──────────────────────────────────────────────────────────────────────── 2 ─────

    let mut sol_optima_encontrada: bool;
    let mut sol_nueva_encontrada:bool;
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
    use std::time::Instant;

    // ─────────────────────────────────────────────────── DECISION DE PARAMETROS ─────

    let tamano_poblacion = 50;
    let numero_genes = cluster.num_elementos;
    //let max_generaciones = 100;
    let max_evaluaciones_fitness = 100_000;
    let m = match modelo {    // Sigo notación de las diapositivas
        ModeloGenetico::Estacionario => 2,
        ModeloGenetico::Generacional => tamano_poblacion
    };

    let probabilidad_cruce = 0.7;
    let numero_cruces:i32 = (probabilidad_cruce * m as f64/2.0).floor() as i32;
    let operador_cruce = match op_cruce_a_usar {
        Operadores::Uniforme => cruce_uniforme,
        Operadores::SegmentoFijo => cruce_segmento_fijo
    };

    let probabilidad_mutacion = 0.001;
    let numero_mutaciones = (probabilidad_mutacion * m as f64 * numero_genes as f64).floor() as i64;

    let mut generador = StdRng::seed_from_u64(semilla);


    // ───────────────────────────────────────────── 1. GENERAR POBLACION INICIAL ─────

    println!("{} Ejecutando algoritmo genético {:?} con operador de cruce {:?} para el cálculo de los clusters", "▸".cyan(), modelo, op_cruce_a_usar);

    // NOTE representaremos la población como un vector de soluciones.
    // De forma paralela, llevaremos un recuento del fitness que producen.
    let mut poblacion = Vec::new();
    let mut fitness_poblacion = Vec::new();

    let now = Instant::now();

    for _ in 0 .. tamano_poblacion {
        let mut solucion_inicial: Vec<usize> = vec![0; numero_genes];

        while !cluster.solucion_valida_externa(&solucion_inicial) {
            for c in solucion_inicial.iter_mut() {
                *c = generador.gen_range(1..=cluster.num_clusters);
            }
        }

        fitness_poblacion.push(cluster.genetico_fitness_sol(&solucion_inicial));
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
            combate = (generador.gen_range(0 .. tamano_poblacion), generador.gen_range(0 .. tamano_poblacion));

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

        match modelo {
            ModeloGenetico::Estacionario => {
                // Crucar ambos con probabilidad uno
                let mut hijo_1= operador_cruce(&p_padres[0], &p_padres[1], &mut generador);

                if !cluster.solucion_valida_externa(&hijo_1) {
                    reparar(&mut hijo_1, cluster.num_clusters, &mut generador);
                }

                p_intermedia.push(
                    hijo_1
                );

                let mut hijo_2 = operador_cruce(&p_padres[1], &p_padres[0], &mut generador);

                if !cluster.solucion_valida_externa(&hijo_2) {
                    reparar(&mut hijo_2, cluster.num_clusters, &mut generador);
                }

                p_intermedia.push(
                    hijo_2
                );
            },
            ModeloGenetico::Generacional => {
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
            }
        }

        // ────────────────────────────────────────────────── MUTACION ─────

        // Elegimos un cromosoma aleatoriamente, y después, lo mutamos uniformemente

        let mut p_hijos = p_intermedia;

        let mut i: usize;

        for _ in 0 .. numero_mutaciones {
            i = generador.gen_range(0 .. m);

            loop {
                let gen_a_mutar = generador.gen_range(0 .. numero_genes);

                let antiguo_cluster = p_hijos[i][gen_a_mutar];
                p_hijos[i][gen_a_mutar] = generador.gen_range(1 ..= cluster.num_clusters);

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
                let mut posicion_peor: usize = 0;
                let mut peor_fitness = 0.0;

                for (i, valor) in fitness_poblacion.iter().enumerate() {
                    if *valor > peor_fitness {
                        peor_fitness = *valor;
                        posicion_peor = i;
                    }
                }

                let fitness_0 = cluster.genetico_fitness_sol(&p_hijos[0]);
                let fitness_1 = cluster.genetico_fitness_sol(&p_hijos[1]);
                evaluaciones_fitness = evaluaciones_fitness + m;


                if fitness_0 < fitness_1 {
                    // Fitness más baja => mejor solución => nos quedamos con el 0
                    poblacion[posicion_peor] = p_hijos[0].clone();
                    fitness_poblacion[posicion_peor] = fitness_0;
                }
                else {
                    poblacion[posicion_peor] = p_hijos[1].clone();
                    fitness_poblacion[posicion_peor] = fitness_1;
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
                    fitness_poblacion[i] = cluster.genetico_fitness_sol(cromosoma);
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


// ────────────────────────────────────────────────────────────────────────────────


fn busqueda_local_suave(solucion: &mut Vec<usize>, cluster: Clusters, fallos_permitidos: usize, generador: &mut StdRng) {
    let mut indices_barajados: Vec<usize> = (0..solucion.len()).collect();
    indices_barajados.shuffle(generador);

    let mut fallos = 0;
    let mut mejora = true;
    let mut i = 0;

    while (mejora || fallos < fallos_permitidos) && i < solucion.len() {
        mejora = false;

        // Asignar el mejor valor posible a solucion[indice de indices_barajados]
        // Es decir, asignar la instancia indices_barajados[i] al cluster que minimice el fitness

        let mut mejor_fitness = cluster.genetico_fitness_sol(solucion);
        let mut mejor_cluster = solucion[indices_barajados[i]];

        for c in 1 ..= cluster.num_clusters {
            if c != mejor_cluster {     // Evitar comprobar el cluster que ya venía
                solucion[indices_barajados[i]] = c;

                let fitness_actual = cluster.genetico_fitness_sol(solucion);

                if fitness_actual < mejor_fitness {
                    mejor_cluster = c;
                    mejor_fitness = fitness_actual;
                    mejora = true;
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
}

/// # Algoritmo memético
/// Parámetros:
/// - **Periodo generacional**: cada cuántas generaciones se aplica la búsqueda local
/// - **Probabilidad**: En `[0, 1]`. Indica cuál es la probabilidad de aplicar la búsqueda local a un cromosoma
/// - **Solo_a_mejores**: si está activado, la búsqueda local solo se aplica a los `probabilidad * tamaño de la población` mejores cromosomas.
fn memetico (cluster: &mut Clusters, periodo_generacional: usize, probabilidad: f64, solo_a_mejores: bool, semilla: u64) -> &mut Clusters {
    /*
        NOTE
        Realmente, la implementación debería usar la función agg_un, pero por motivos de comodidad/tiempo/pereza,
        voy a hacer copy - paste del genético usando únicamente el modelo generacional con operador de cruce uniforme,
        que es el que mejores resultados produce.

        Si el Andrés del futuro va menos agobiado con el tiempo, y tiene ganas, le dejo propuesto como ejercicio
        solucionar este problema y refactorizar la función (?)
    */

    use std::time::Instant;

    // ─────────────────────────────────────────────────── DECISION DE PARÁMETROS ─────

    let tamano_poblacion = 10;
    let numero_genes = cluster.num_elementos;
    //let max_generaciones = 100;
    let max_evaluaciones_fitness = 100_000;
    let m = tamano_poblacion;

    let probabilidad_cruce = 0.7;
    let numero_cruces:i32 = (probabilidad_cruce * m as f64/2.0).floor() as i32;
    let operador_cruce = cruce_uniforme;

    let probabilidad_mutacion = 0.001;
    let numero_mutaciones = (probabilidad_mutacion * m as f64 * numero_genes as f64).floor() as i64;

    let fallos_maximos = (0.1 * tamano_poblacion as f64).floor() as usize;      // FIXME n == tamaño de la población?

    if probabilidad != 0.1 && solo_a_mejores {
        println!("{}: este algoritmo no está pensado para ejecutarse con estos parámetros de entrada", "WARNING".red());
    }

    let mut generador = StdRng::seed_from_u64(semilla);


    // ───────────────────────────────────────────── 1. GENERAR POBLACION INICIAL ─────

    println!("{} Ejecutando algoritmo memético de base agg_un para el cálculo de los clusters", "▸".cyan());

    // NOTE representaremos la población como un vector de soluciones.
    // De forma paralela, llevaremos un recuento del fitness que producen.
    let mut poblacion = Vec::new();
    let mut fitness_poblacion = Vec::new();

    let now = Instant::now();

    for _ in 0 .. tamano_poblacion {
        let mut solucion_inicial: Vec<usize> = vec![0; numero_genes];

        while !cluster.solucion_valida_externa(&solucion_inicial) {
            for c in solucion_inicial.iter_mut() {
                *c = generador.gen_range(1..=cluster.num_clusters);
            }
        }

        fitness_poblacion.push(cluster.genetico_fitness_sol(&solucion_inicial));
        poblacion.push(solucion_inicial);
    }


    // ─────────────────────────────────────────────────────── 2. BUCLE PRINCIPAL ─────


    let mut t = 0;  // Generaciones
    let mut evaluaciones_fitness = 0;

    while evaluaciones_fitness < max_evaluaciones_fitness {
        //println!("\t{} Comienza generación {}", "▸".cyan(), t);

        // ───────────────────────────────────────── EXPLORACION LOCAL ─────

        if t % periodo_generacional == 0 {
            if solo_a_mejores {
                let busquedas_totales = (probabilidad * poblacion.len() as f64).floor() as usize;

                // Necesitamos ordenar de menor a mayor para ver quiénes son los mejores.
                // Los mejores se encuentran al principio del vector
                fitness_poblacion.sort_by(|a, b| a.partial_cmp(b).unwrap());
                poblacion.sort_by(|a, b|
                    cluster.genetico_fitness_sol(a).partial_cmp(&cluster.genetico_fitness_sol(b)).unwrap()
                );

                for i in 0 .. busquedas_totales {
                    busqueda_local_suave(&mut poblacion[i], cluster, fallos_maximos, &mut generador);
                    fitness_poblacion[i] = cluster.genetico_fitness_sol(&poblacion[i]);
                }
            }
            else {
                for i in 0 .. tamano_poblacion {
                    if generador.gen_range(0.0..=1.0) <= probabilidad {
                        busqueda_local_suave(&mut poblacion[i], cluster, fallos_maximos, &mut generador);
                        fitness_poblacion[i] = cluster.genetico_fitness_sol(&poblacion[i]);
                    }
                }
            }
        }

        // ───────────────────────────────────────────────── SELECCION ─────

        let mut p_padres = Vec::new();
        let mut combate; // Los enfrentamientos se harán del `i` vs `i+1`. Se guardan como (combatiente 1, combatiente 2)

        // Crear cuadro de combatientes
        for _ in 0 .. m {
            combate = (generador.gen_range(0 .. tamano_poblacion), generador.gen_range(0 .. tamano_poblacion));

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
            i = generador.gen_range(0 .. m);

            loop {
                let gen_a_mutar = generador.gen_range(0 .. numero_genes);

                let antiguo_cluster = p_hijos[i][gen_a_mutar];
                p_hijos[i][gen_a_mutar] = generador.gen_range(1 ..= cluster.num_clusters);

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
            fitness_poblacion[i] = cluster.genetico_fitness_sol(cromosoma);
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