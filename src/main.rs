//
// ─────────────────────────────────────────────────── ESTRUCTURA DE ARCHIVOS ─────
//

    mod cluster;
    mod utils;
    mod file_io;
    mod algorithm;

//
// ────────────────────────────────────────────────────────────────── IMPORTS ─────
//

    use std::time::{Instant};
    use file_io::*;
    use std::path::Path;

fn main() {

    let directorio: &Path = Path::new("./data/PAR/");
    let archivos = file_io::leer_archivos_dir(directorio);
    println!("Archivos: {:?}", &archivos);

    let mut mi_cluster = leer_archivo_PAR(&archivos[6], &archivos[7]);

    let mut now = Instant::now();
    mi_cluster.calcular_matriz_distancias();
    println!("Distancias calculadas en {:?} ", now.elapsed());

    println!("\nEjecutando greedy");

    now = Instant::now();
    let mi_cluster = algorithm::greedy_COPKM(&mut mi_cluster);
    println!("Clusters: {:?}", &mi_cluster.clusters());

    println!("Centroides calculados en {:?} ", now.elapsed());

}