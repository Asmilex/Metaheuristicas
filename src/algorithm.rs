use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use nalgebra::{DVector};
use colored::*;

use crate::cluster::*;
use crate::utils::*;

#[allow(non_snake_case)]
pub fn greedy_COPKM (cluster: &mut Clusters) -> &mut Clusters {
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

    let mut rng = thread_rng();     // Criptográficamente seguro; distribución estándar https://rust-random.github.io/book/guide-rngs.html

    let mut centroides_aleatorios: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_clusters];

    for i in 0 .. centroides_aleatorios.len() {
        for j in 0 .. centroides_aleatorios[i].nrows() {
            centroides_aleatorios[i][(j)] = rng.gen();  // Distribución en 0 .. 1
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

    println!("{} Cálculo del cluster finalizado en {} iteraciones {}\n", "▸".cyan() , iters, "✓".green());
    cluster
}