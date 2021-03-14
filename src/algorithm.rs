use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use nalgebra::{DVector};

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

    println!("▸ Calculando centroides aleatorios iniciales");

    let mut rng = thread_rng();     // Criptográficamente seguro; distribución estándar https://rust-random.github.io/book/guide-rngs.html

    let mut centroides_aleatorios: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_clusters];

    for i in 0 .. centroides_aleatorios.len() {
        for j in 0 .. centroides_aleatorios[i].nrows() {
            centroides_aleatorios[i][(j)] = rng.gen();  // Distribución en 0 .. 1
        }
    }

    cluster.asignar_centroides(centroides_aleatorios);

    // ─────────────────────────────────────────────────────── 2. BARAJAR INDICES ─────

    println!("▸ Barajando índices");

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

    println!("▸ Computando cluster");

    let mut cambios_en_cluster = true;

    while cambios_en_cluster {
        cambios_en_cluster = false;

        // ─── 3.1 ─────────────────────────────────────────────────────────

        for index in indices_barajados.iter() {
            //println!("\t▸ Calculando índice {}", index);
            // FIXME hay que comprobar el incremento, no la esperada!
            // FIXME Andrés de mañana, corrígelo fiera.

            let mut min_infeasibility: u32 = u32::MAX;
            let mut best_clusters: Vec<usize> = Vec::new();

            for cluster_temp in 1 ..= cluster.num_clusters {
                let expected_infeasibility = cluster.infeasibility_esperada(*index as usize, cluster_temp);

                if expected_infeasibility < min_infeasibility {
                    // Si es menor, limpiar todo y seguir. Si son iguales, tenemos un nuevo candidato

                    if expected_infeasibility != min_infeasibility {
                        min_infeasibility = expected_infeasibility;
                        best_clusters.clear();
                    }

                    best_clusters.push(cluster_temp);
                    cambios_en_cluster = true;
                }
            }

            // Una vez tenemos los mejores clusters para un cierto índice, seleccionar aquel cuyo centroide sea el menor
            let mut distancia_min = f64::MAX;
            let mut best_cluster: usize = 0;

            for c in best_clusters.iter() {
                let distancia_temp = distancia(&cluster.centroide_cluster(*c), &cluster.espacio[(*index as usize)]);
                if distancia_temp < distancia_min {
                    distancia_min = distancia_temp;
                    best_cluster = *c;
                }
            }

            // Asignar el nuevo centroide
            cluster.asignar_cluster_a_elemento(*index as usize, best_cluster);
        }


        // ─── 3.2 ─────────────────────────────────────────────────────────

        cluster.calcular_centroides();
    }

    cluster
}