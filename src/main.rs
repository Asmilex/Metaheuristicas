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
    use algorithm::greedy_COPKM;
    use file_io::*;
    use utils::{Algoritmos, InfoEjecucion, PAR_nombres, PAR_parametros, PAR_restr, RutasCSV};
    use std::env;
    use colored::*;

// ────────────────────────────────────────────────────────────────────────────────

fn benchmark(algoritmo: Algoritmos, dataset: PAR_parametros, restriccion: PAR_restr) -> Vec<InfoEjecucion> {
    /*
        Primero, se rellena el cluster con la información del espacio.
        Después, se decide qué algoritmo utilizar.
        Finalmente, se ejecuta el algoritmo seleccionado 5 veces y se provee un vector con los resultados de la ejecución.
    */

    println!("\n{}: Se empieza a ejecutar el benchmark del algoritmo {:#?} para el dataset {:#?} con las restricciones {:#?}", "§ BENCHMARK §".red(), algoritmo, dataset.tipo, restriccion);
    let mut cluster = leer_archivo_PAR(dataset, restriccion);

    // TODO cambiar cuando haya implementado el resto de algoritmos
    let funcion = greedy_COPKM;

    let mut ejecuciones: Vec<InfoEjecucion> = Vec::new();

    for i in 1 ..= 5 {
        let mut info = InfoEjecucion::new();
        let now = Instant::now();

        let cluster = funcion(&mut cluster);

        info.tiempo     = now.elapsed();
        info.tasa_inf   = cluster.infeasibility();
        info.error_dist = cluster.desviacion_general_particion();
        info.agr        = info.error_dist + cluster.lambda() * info.tasa_inf as f64;

        ejecuciones.push(info);

        cluster.reset_clusters();
    }

    println!("Benchmark completado {}\n", "✓".green());
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
            let rutas = RutasCSV::new(Algoritmos::Greedy);

            // ──────────────────────────────────────────────────── ZOO 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                PAR_parametros::new(PAR_nombres::Zoo),
                PAR_restr::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.zoo_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.zoo_10, "✓".green()),
                Err(r) => println!("ERROR: {}", r)
            };

            // ──────────────────────────────────────────────────── ZOO 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                PAR_parametros::new(PAR_nombres::Zoo),
                PAR_restr::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.zoo_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.zoo_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ────────────────────────────────────────────────── GLASS 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                PAR_parametros::new(PAR_nombres::Glass),
                PAR_restr::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.glass_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.glass_10, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ────────────────────────────────────────────────── GLASS 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                PAR_parametros::new(PAR_nombres::Glass),
                PAR_restr::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.glass_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.glass_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ─────────────────────────────────────────────────── BUPA 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                PAR_parametros::new(PAR_nombres::Bupa),
                PAR_restr::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.bupa_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.bupa_10, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ─────────────────────────────────────────────────── BUPA 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                PAR_parametros::new(PAR_nombres::Bupa),
                PAR_restr::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.bupa_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.bupa_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }
        }

        if algoritmos.BL {

        }
    }
}