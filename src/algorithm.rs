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
            3.
    */

    //
    // ───────────────────────────────────────────────── 1. CENTROIDES ALEATORIOS ─────
    //

    let mut rng = thread_rng();     // Criptográficamente seguro https://rust-random.github.io/book/guide-rngs.html

    let mut centroides_aleatorios: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_clusters];

    for i in 0 .. centroides_aleatorios.len() {
        for j in 0 .. centroides_aleatorios[i].ncols() {
            centroides_aleatorios[i][(j)] = rng.gen();  // Distribución en 0 .. 1
        }
    }

    //
    // ─────────────────────────────────────────────────────── 2. BARAJAR INDICES ─────
    //


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


    cluster
}