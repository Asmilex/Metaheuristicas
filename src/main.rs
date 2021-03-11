/*  NOTE código de ejemplo para usar random. Se eliminará más tarde.
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();

    for _i in 1..4 {
        let x: u32 = rng.gen();
        println!("{}", x);
        println!("{:?}", rng.gen::<(f64, bool)>());
    }
 */


// Declaración de la estructura de archivos
mod cluster;
mod utils;

// Imports
use cluster::*;

fn main() {
    let mi_cluster = Clusters::new(2, 3, 10);

    print!("{:#?}", mi_cluster);
}