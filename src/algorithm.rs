use rand::{Rng, SeedableRng, rngs::StdRng, seq::SliceRandom};
use rand::distributions::{Distribution, Uniform};

use nalgebra::{DVector};
use colored::*;


use crate::cluster::*;
use crate::utils::*;

#[allow(non_snake_case)]
pub fn greedy_COPKM (cluster: &mut Clusters, seed: u64) -> &mut Clusters {
    /*
        Pasos:
            1. Sacar centroides aleatorios. Todos los elementos del espacio se encuentran en [0, 1]x...x[0,1]
            2. Barajar los índices para recorrerlos de forma aleatoria sin repetición
            3. Mientras se produzcan cambios en el cluster:
                3.1. Para cada índice barajado, mirar qué incremento supone en la infeasibility al asignarlo a un cluster. Tomar el menor de estos.
                3.2. Actualizar los centroides
    */

    // ───────────────────────────────────────────────── 1. CENTROIDES ALEATORIOS ─────

    println!("{} Ejecutando greedy_COPKM para el cálculo de los clusters", "▸".cyan());

    let mut rng = StdRng::seed_from_u64(seed);

    let mut centroides_aleatorios: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_clusters];

    for i in 0 .. centroides_aleatorios.len() {
        for j in 0 .. centroides_aleatorios[i].nrows() {
            centroides_aleatorios[i][(j)] = rng.gen();
        }
    }

    cluster.asignar_centroides(centroides_aleatorios);

    // ─────────────────────────────────────────────────────── 2. BARAJAR INDICES ─────

    let mut indices_barajados: Vec<i32> = vec![-1; cluster.num_elementos];
    let mut recuento_indices: usize = 0;

    let gen_indices = Uniform::from(0 .. cluster.num_elementos);

    loop {
        let indice_generado: i32 = gen_indices.sample(&mut rng) as i32;

        if !indices_barajados.contains(&indice_generado) {
            indices_barajados[recuento_indices] = indice_generado;
            recuento_indices = recuento_indices + 1;
        }

        if recuento_indices == indices_barajados.len() {
            break;
        }
    }

    // ─────────────────────────────────────────────────── 3. COMPUTO DEL CLUSTER ─────

    let mut cambios_en_cluster = true;
    let mut iters: usize = 0;

    while cambios_en_cluster {
        iters = iters + 1;

        cambios_en_cluster = false;

        // ─── 3.1 ─────────────────────────────────────────────────────────

        for index in indices_barajados.iter() {

            // Calcular el incremento en infeasibility que produce la asignación de xi a cada cluster cj

            let mut expected_infeasibility: Vec<u32> = Vec::new();

            for c in 1 ..= cluster.num_clusters {
                expected_infeasibility.push(cluster.infeasibility_delta_esperada(*index as usize, c));
            }

            let minima_infeasibility = expected_infeasibility.iter().min().unwrap();    // Al ser la infeasibily actual una constante, aquella que produzca la menor es la que tiene una delta menor con respecto al total.

            let mut distancia_min = f64::MAX;
            let mut best_cluster: usize = 0;

            // De entre las asignaciones que producen menos incremento en infeasiblity, seleccionar la asociada con el centroide mu_j más cercano a xi
            for c in 1 ..= cluster.num_clusters {
                if expected_infeasibility[c-1] == *minima_infeasibility {
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



pub fn busqueda_local (cluster: &mut Clusters, semilla: u64) -> &mut Clusters {
    /*
        Pasos:
            1. Generar una solución válida inicial. Esto es, aquella en la que los clusters están entre 1 y num_cluster, y
            no tiene clusters vacíos
            2. Recorrer el vecindario hasta que encuentres una solución cuyo fitness se queda por debajo de tu solución actual.
            El vecindario se debe recorrer de forma (i, l), donde
                -> i = índice de la solución
                -> l es el cluster nuevo a asignar.
            3. Cuando se alcancen el número máximo de iteraciones, o no se consiga minimizar la función objetivo, hemos acabado.
    */
    use std::time::{Instant};
    println!("{} Ejecutando búsqueda local para el cálculo de los clusters", "▸".cyan());
    let now = Instant::now();

    let max_iteraciones = 10_000;

    let k = cluster.num_clusters;
    let mut generador = StdRng::seed_from_u64(semilla);
    let solucion_valida = |s: &Vec<usize>| -> bool {
        for c in 1..=k {
            if !s.iter().any(|&valor| valor == c) {
                return false;
            }
        }
        return true;
    };

    // ──────────────────────────────────────────────────────────────────────── 1 ─────

    let mut solucion_inicial: Vec<usize> = vec![0; cluster.num_elementos];

    while !solucion_valida(&solucion_inicial) {
        for c in solucion_inicial.iter_mut() {
            *c = generador.gen_range(1..=cluster.num_clusters);
        }
    }

    cluster.asignar_clusters(solucion_inicial.clone());
    let mut sol_optima: bool;      // Aquella que cumple que no existe otra solución S' tal que f(S) < f(S') para toda otra S
    let mut fitness_actual = cluster.fitness();
    let mut infeasibility_actual = cluster.infeasibility();
    let mut clusters_barajados: Vec<usize> = (1..=cluster.num_clusters).collect();

    for _ in 0..max_iteraciones {
        //let now = Instant::now();
        let mut nueva_sol_encontrada = false;
        sol_optima = true;

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
                                    - cluster.infeasibility_delta_esperada(*i, cluster.cluster_de_indice(*i))
                                    + cluster.infeasibility_delta_esperada(*i, *c);
                                cluster.asignar_cluster_a_elemento(*i, *c);
                                nueva_sol_encontrada = true;
                                break;
                            }
                        },
                        Err(_r) => {}
                    };
                }
            }


            if nueva_sol_encontrada {
                sol_optima = false;
                break;
            }
        }

        if sol_optima{
            break;
        }
    }

    println!("{} Cálculo del cluster finalizado en {} ms {}\n", "▸".cyan(), now.elapsed().as_millis(),  "✓".green());

    cluster
}