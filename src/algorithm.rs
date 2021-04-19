use rand::{Rng, SeedableRng, rngs::StdRng, seq::SliceRandom};

use nalgebra::{DVector};
use colored::*;

use crate::cluster::*;
use crate::utils::*;
use crate::operator::*;

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

    let k = cluster.num_clusters;
    let solucion_valida = |s: &Vec<usize>| -> bool {
        for c in 1..=k {
            if !s.iter().any(|&valor| valor == c) {
                return false;
            }
        }
        return true;
    };

    while !solucion_valida(&solucion_inicial) {
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



pub fn genetico (cluster: &mut Clusters, modelo: ModeloGenetico, operador_cruce: Operadores, semilla: u64) -> &mut Clusters {
    /*
        Pasos:
            1. Inicializar variables:
                - Inicializar una población P(0)
                - Evaluar P(0)
            2. Bucle principal
                2.1 Seleccionar nueva población desde la anterior P(t-1). Sea P_padres ésta. El operador de selección es torneo binario. Otras consideraciones:
                    - En el modelo generacional, el tamaño de P_padres es el mismo que el de la población inicial => se hacen 50 torneos.
                    - En el modelo estacionario, el tamaño de P_padres es 2 => 2 torneos.
                2.2 Cruzar P_padres y guardarlo en P_intermedia.
                    Como la selección ya tiene una componente aleatoria, fijamos una probabilidad de cruce (P_c) y únicamente hacemos los siguientes cruces:
                        Número de cruces = P_c * Tamaño de la población / 2.
                    Los tomamos por orden: primero con el segundo, tercero con el cuarto...
                2.3 Mutar P_intermedia con probabilidad p_m y guardarlo en P_hijos
                2.4 Reemplazar la población P(t) a partir de P(t-1) y P_hijos.
                    - En el modelo generacional, se mantiene el mejor individuo de P(t-1).
                    - En el modelo estacionario, los dos hijos que se encuentran en P_hijos compiten para entrar en P(t).
                2.5 Evaluar P(t).
    */
    use std::time::{Instant};

    let tamano_poblacion = 50;
    let max_generaciones = 100;
    let probabilidad_cruce = 0.6;
    let m = match modelo {    // Sigo notación de las diapositivas
        ModeloGenetico::Estacionario => 2,
        ModeloGenetico::Generacional => tamano_poblacion
    };
    let numero_cruces = probabilidad_cruce * m as f64/2.0;
    let mut generador = StdRng::seed_from_u64(semilla);


    // ───────────────────────────────────────────── 1. GENERAR POBLACION INICIAL ─────

    // NOTE representaremos la población como un vector de soluciones.
    // De forma paralela, llevaremos un recuento del fitness que producen.
    let mut poblacion = Vec::new();
    let mut fitness_poblacion = Vec::new();

    let now = Instant::now();


    for _ in 0 .. tamano_poblacion {
        let mut solucion_inicial: Vec<usize> = vec![0; cluster.num_elementos];

        let k = cluster.num_clusters;
        let solucion_valida = |s: &Vec<usize>| -> bool {
            for c in 1..=k {
                if !s.iter().any(|&valor| valor == c) {
                    return false;
                }
            }
            return true;
        };

        while !solucion_valida(&solucion_inicial) {
            for c in solucion_inicial.iter_mut() {
                *c = generador.gen_range(1..=cluster.num_clusters);
            }
        }

        fitness_poblacion.push(cluster.genetico_fitness_sol(&solucion_inicial));
        poblacion.push(solucion_inicial);
    }

    println!("Población inicial generada en {}", now.elapsed().as_millis());


    // ─────────────────────────────────────────────────────── 2. BUCLE PRINCIPAL ─────

    for t in 1..max_generaciones {
        // ───────────────────────────────────────────────── SELECCION ─────

        let p_padres = Vec::new();
        let cruces = Vec::new();    // Los enfrentamientos se harán del `i` vs `i+1`. Se guardan como (combatiente 1, combatiente 2)

        // Crear cuadro de combatientes
        for i in 0 .. m {
            cruces.push(
                (generador.gen_range(0 .. tamano_poblacion - 1), generador.gen_range(0 .. tamano_poblacion - 1))
            );
        }

        // Enfrentar y seleccionar
        for i in 0 .. m {
            if fitness_poblacion[cruces[i].0] < fitness_poblacion[cruces[i].1] {
                p_padres.push(poblacion[cruces[i].0].clone());
            }
            else {
                p_padres.push(poblacion[cruces[i].1].clone());
            }
        }


        // ───────────────────────────────────────────────────── CRUCE ─────

        


    }

    cluster
}