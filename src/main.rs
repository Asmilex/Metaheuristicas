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
    println!("Archivos: {:#?}", &archivos);

    let mut mi_cluster = leer_archivo_PAR(&archivos[0], &archivos[2]);

    let now = Instant::now();
    let mi_cluster = algorithm::greedy_COPKM(&mut mi_cluster);

    println!("{}", &mi_cluster);
    println!("Greedy calculado en {:?}\n", now.elapsed());

}