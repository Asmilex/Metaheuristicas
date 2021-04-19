//
// ─────────────────────────────────────────────────── ESTRUCTURA DE ARCHIVOS ─────
//

    mod cluster;
    mod utils;
    mod file_io;
    mod algorithm;
    mod operator;

//
// ────────────────────────────────────────────────────────────────── IMPORTS ─────
//

    use std::time::Instant;
    use std::env;
    use file_io::*;
    use colored::*;

    use algorithm::{busqueda_local, greedy_COPKM};
    use utils::{Algoritmos, InfoEjecucion, Datasets, ParametrosDataset, Restricciones, RutasCSV};

// ────────────────────────────────────────────────────────────────────────────────

fn benchmark(algoritmo: Algoritmos, dataset: ParametrosDataset, restriccion: Restricciones) -> Vec<InfoEjecucion> {
    /*
        Primero, se rellena el cluster con la información del espacio.
        Después, se decide qué algoritmo utilizar.
        Finalmente, se ejecuta el algoritmo seleccionado 5 veces y se provee un vector con los resultados de la ejecución.
    */

    println!("\n{}: Se empieza a ejecutar el benchmark del algoritmo {:#?} para el dataset {:#?} con las restricciones {:#?}", "§ BENCHMARK §".red().bold(), algoritmo, dataset.tipo, restriccion);
    let mut cluster = leer_archivo_PAR(&dataset, &restriccion);

    // TODO cambiar cuando haya implementado el resto de algoritmos
    let funcion = match algoritmo {
        Algoritmos::Greedy => greedy_COPKM,
        Algoritmos::BL     => busqueda_local
    };

    let mut ejecuciones: Vec<InfoEjecucion> = Vec::new();
    let semillas = utils::Semillas::new();

    for i in 0 .. 5 {
        let mut info = InfoEjecucion::new();
        let now = Instant::now();

        let cluster = funcion(&mut cluster, semillas.semilla(i));

        info.tiempo     = now.elapsed();
        info.tasa_inf   = cluster.infeasibility();
        info.error_dist = f64::abs(cluster.desviacion_general_particion() - dataset.distancia_optima);
        info.agr        = cluster.fitness();

        ejecuciones.push(info);

        cluster.reset_clusters();
    }

    println!("Benchmark completado {}\n", "✓".green().bold());
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
        let mut mi_cluster = leer_archivo_PAR(&parametros, &restricciones);

        if algoritmos.greedy {
            let now = Instant::now();
            let mi_cluster = algorithm::greedy_COPKM(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("Greedy calculado en {:?} {}\n", now.elapsed(), "✓".green().bold());
            println!("{}", &mi_cluster);
        }
        else if algoritmos.BL {
            let now = Instant::now();
            let mi_cluster = algorithm::busqueda_local(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("{}", &mi_cluster);
            println!("Búsqueda local calculado en {:?}\n", now.elapsed());
        }
    }
    else {
        if algoritmos.greedy {
            let rutas = RutasCSV::new(Algoritmos::Greedy);

            // ──────────────────────────────────────────────────── ZOO 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                ParametrosDataset::new(Datasets::Zoo),
                Restricciones::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.zoo_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.zoo_10, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            };

            // ──────────────────────────────────────────────────── ZOO 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                ParametrosDataset::new(Datasets::Zoo),
                Restricciones::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.zoo_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.zoo_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ────────────────────────────────────────────────── GLASS 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                ParametrosDataset::new(Datasets::Glass),
                Restricciones::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.glass_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.glass_10, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ────────────────────────────────────────────────── GLASS 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                ParametrosDataset::new(Datasets::Glass),
                Restricciones::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.glass_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.glass_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ─────────────────────────────────────────────────── BUPA 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                ParametrosDataset::new(Datasets::Bupa),
                Restricciones::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.bupa_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.bupa_10, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ─────────────────────────────────────────────────── BUPA 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::Greedy,
                ParametrosDataset::new(Datasets::Bupa),
                Restricciones::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.bupa_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.bupa_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            println!("{}", "Finalizado el benchmark de greedy".green());
        }

        if algoritmos.BL {
            let rutas = RutasCSV::new(Algoritmos::BL);

            // ──────────────────────────────────────────────────── ZOO 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::BL,
                ParametrosDataset::new(Datasets::Zoo),
                Restricciones::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.zoo_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.zoo_10, "✓".green()),
                Err(r) => println!("ERROR: {}", r)
            };

            // ──────────────────────────────────────────────────── ZOO 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::BL,
                ParametrosDataset::new(Datasets::Zoo),
                Restricciones::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.zoo_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.zoo_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ────────────────────────────────────────────────── GLASS 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::BL,
                ParametrosDataset::new(Datasets::Glass),
                Restricciones::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.glass_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.glass_10, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ────────────────────────────────────────────────── GLASS 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::BL,
                ParametrosDataset::new(Datasets::Glass),
                Restricciones::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.glass_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.glass_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ─────────────────────────────────────────────────── BUPA 10 ─────

            let datos_benchmark = benchmark(
                Algoritmos::BL,
                ParametrosDataset::new(Datasets::Bupa),
                Restricciones::Diez
            );

            match export_to_csv(&datos_benchmark, &rutas.bupa_10) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.bupa_10, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            // ─────────────────────────────────────────────────── BUPA 20 ─────

            let datos_benchmark = benchmark(
                Algoritmos::BL,
                ParametrosDataset::new(Datasets::Bupa),
                Restricciones::Veinte
            );

            match export_to_csv(&datos_benchmark, &rutas.bupa_20) {
                Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.bupa_20, "✓".green()),
                Err(r) => println!("ERROR al exportar el archivo csv: {}", r)
            }

            println!("{}", "Finalizado el benchmark de Búsqueda Local".green());
        }
    }
}