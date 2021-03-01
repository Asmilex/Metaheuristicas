/*  NOTE código de ejemplo para usar random. Se eliminará más tarde.
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    for _i in 1..4 {
        let x: u32 = rng.gen();
        println!("{}", x);
        println!("{:?}", rng.gen::<(f64, bool)>());
    }
 */

mod file_io;
use file_io::*;
use std::path::Path;

fn main() {
    let directorio: &Path = Path::new("./data/MDG/");
    let archivos = leer_archivos_dir(directorio);

    for elemento in archivos {
        println!("{:?}", elemento);
    }
}