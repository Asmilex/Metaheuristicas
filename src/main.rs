/*  NOTE código de ejemplo para usar random. Se eliminará más tarde.
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    for _i in 1..4 {
        let x: u32 = rng.gen();
        println!("{}", x);
        println!("{:?}", rng.gen::<(f64, bool)>());
    }
 */


//
// ─────────────────────────────────────────────────── ESTRUCTURA DE ARCHIVOS ─────
//

    mod cluster;
    mod utils;
    mod file_io;

//
// ────────────────────────────────────────────────────────────────── IMPORTS ─────
//

    use cluster::*;
    use file_io::*;
    use std::path::Path;


fn main() {

    let directorio: &Path = Path::new("./data/PAR/");
    let archivos = file_io::leer_archivos_dir(directorio);

    let mi_cluster = leer_archivo_PAR(&archivos[0], &archivos[1]);

    //println!("{:?}", mi_cluster);

}