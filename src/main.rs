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
    use utils::{Algoritmos, InfoExecution, PAR_nombres, PAR_parametros, PAR_restr};
    use std::env;

// ────────────────────────────────────────────────────────────────────────────────

fn benchmark(algoritmo: Algoritmos, dataset: PAR_parametros, restriccion: PAR_restr) -> Vec<InfoExecution> {
    /*
        Se ejecutarán los algoritmos seleccionados 5 veces cada uno. Los dataset a usar, y por orden de ejecución, son:
            1. Zoo, 10% de restricciones
            2. Zoo, 20% de restricciones
            3. Glass, 10% de restricciones
            4. Glass, 20% de restricciones
            5. Bupa, 10% de restricciones
            6. Bupa, 20% de restricciones
    */

    // FIXME REFACTORIZAR LA FUNCIÓN
    //

    let lambda: f64 = 1.0;
    let mut ejecuciones: Vec<InfoExecution> = Vec::new();

    for _i in 1..=5 {
        let mut info = InfoExecution::new();
        let mut mi_cluster = leer_archivo_PAR(PAR_parametros::new(PAR_nombres::Zoo), PAR_restr::Diez);
        let now = Instant::now();
        let mi_cluster = algorithm::greedy_COPKM(&mut mi_cluster);

        info.tiempo_zoo     = now.elapsed();
        info.tasa_inf_zoo   = mi_cluster.infeasibility();
        info.error_dist_zoo = mi_cluster.desviacion_general_particion();
        info.agr_zoo        = info.error_dist_zoo + lambda * info.tasa_inf_zoo as f64;


        ejecuciones.push(info);
    }

    ejecuciones

}


// ────────────────────────────────────────────────────────────────────────────────

fn main() {
    let (parametros, restricciones, algoritmos) = match parse_arguments(&env::args().collect()) {
        Ok(r) => r,
        Err(err) => panic!("No se han introducido correctamente los parámetros de entrada. Error: {}", err)
    };

    if !algoritmos.benchmark {
        let parametros = parametros.unwrap();
        let restricciones = restricciones.unwrap();
        let mut mi_cluster = leer_archivo_PAR(parametros, restricciones);

        if algoritmos.greedy {
            let now = Instant::now();
            let mi_cluster = algorithm::greedy_COPKM(&mut mi_cluster);
            println!("{}", &mi_cluster);
            println!("Greedy calculado en {:?}\n", now.elapsed());
        }
    }
    else {
        if algoritmos.greedy {

        }

        if algoritmos.BL {

        }
    }
}