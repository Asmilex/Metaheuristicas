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

    now = Instant::now();
    mi_cluster.calcular_centroides();
    println!("Centroides: {:?}", &mi_cluster.vector_centroides());
    println!("Centroides calculados en {:?} ", now.elapsed());

}