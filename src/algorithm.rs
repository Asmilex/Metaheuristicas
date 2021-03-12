use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use nalgebra::{DVector};

use crate::cluster::*;
use crate::utils::*;

#[allow(non_snake_case)]
pub fn greedy_COPKM (cluster: &mut Clusters) -> &mut Clusters {
    /*
        Pasos:
            1. Sacar centroides aleatorios. Cojo ciertos índices y los asigno como centroides
            2. Barajar los índices para recorrerlos de forma aleatoria sin repetición
            3.
    */

    //
    // ───────────────────────────────────────────────── 1. CENTROIDES ALEATORIOS ─────
    //

    let mut rng = thread_rng();
    let gen_indices = Uniform::from(0 .. cluster.num_elementos);

    let mut centroides_aleatorios: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_clusters];
    let mut vector_indices: Vec<i32> = vec![-1; cluster.num_clusters];
    let mut recuento_indices: usize = 0;

    loop {
        let indice_generado: i32 = gen_indices.sample(&mut rng) as i32;

        if !vector_indices.contains(&indice_generado) {
            vector_indices[recuento_indices] = indice_generado;
            recuento_indices = recuento_indices + 1;
        }

        if recuento_indices == vector_indices.len() {
            break;
        }
    }

    for i in 0 .. vector_indices.len() {
        centroides_aleatorios[i] = cluster.espacio[(vector_indices[i] as usize)].clone();
    }

    //
    // ─────────────────────────────────────────────────────── 2. BARAJAR INDICES ─────
    //

    let mut indices_barajados: Vec<i32> = vec![-1; cluster.num_elementos];
    recuento_indices = 0;

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


    cluster
}