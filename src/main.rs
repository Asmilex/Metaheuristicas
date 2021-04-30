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

    use algorithm::*;
    use utils::{Algoritmos, InfoEjecucion, Datasets, ParametrosDataset, Restricciones, RutasCSV};

// ────────────────────────────────────────────────────────────────────────────────

fn inicializador_benchmark(algoritmo: Algoritmos) {
    let rutas = RutasCSV::new(algoritmo);

    // ──────────────────────────────────────────────────── ZOO 10 ─────

    let datos_benchmark = benchmark(
        algoritmo,
        ParametrosDataset::new(Datasets::Zoo),
        Restricciones::Diez
    );

    match export_to_csv(&datos_benchmark, &rutas.zoo_10) {
        Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.zoo_10, "✓".green()),
        Err(r) => println!("Error al exportar el archivo a csv: {}", r)
    };

    // ──────────────────────────────────────────────────── ZOO 20 ─────

    let datos_benchmark = benchmark(
        algoritmo,
        ParametrosDataset::new(Datasets::Zoo),
        Restricciones::Veinte
    );

    match export_to_csv(&datos_benchmark,&rutas.zoo_20) {
        Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.zoo_20, "✓".green()),
        Err(r) => println!("Error al exportar el archivo a csv: {}", r)
    }

    // ────────────────────────────────────────────────── GLASS 10 ─────

    let datos_benchmark = benchmark(
        algoritmo,
        ParametrosDataset::new(Datasets::Glass),
        Restricciones::Diez
    );

    match export_to_csv(&datos_benchmark, &rutas.glass_10) {
        Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.glass_10, "✓".green()),
        Err(r) => println!("Error al exportar el archivo a csv: {}", r)
    }

    // ────────────────────────────────────────────────── GLASS 20 ─────

    let datos_benchmark = benchmark(
        algoritmo,
        ParametrosDataset::new(Datasets::Glass),
        Restricciones::Veinte
    );

    match export_to_csv(&datos_benchmark, &rutas.glass_20) {
        Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.glass_20, "✓".green()),
        Err(r) => println!("Error al exportar el archivo a csv: {}", r)
    }

    // ─────────────────────────────────────────────────── BUPA 10 ─────

    let datos_benchmark = benchmark(
        algoritmo,
        ParametrosDataset::new(Datasets::Bupa),
        Restricciones::Diez
    );

    match export_to_csv(&datos_benchmark, &rutas.bupa_10) {
        Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.bupa_10, "✓".green()),
        Err(r) => println!("Error al exportar el archivo a csv: {}", r)
    }

    // ─────────────────────────────────────────────────── BUPA 20 ─────

    let datos_benchmark = benchmark(
        algoritmo,
        ParametrosDataset::new(Datasets::Bupa),
        Restricciones::Veinte
    );

    match export_to_csv(&datos_benchmark, &rutas.bupa_20) {
        Ok(()) => println!("Exportado con éxito el archivo {} {}", &rutas.bupa_20, "✓".green()),
        Err(r) => println!("Error al exportar el archivo a csv: {}", r)
    }


    println!("{}", "Benchmark del algoritmo finalizado".green());
}


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
        Algoritmos::BL     => busqueda_local,
        Algoritmos::AGG_UN => agg_un,
        Algoritmos::AGG_SF => agg_sf,
        Algoritmos::AGE_UN => age_un,
        Algoritmos::AGE_SF => age_sf,
        Algoritmos::AM_10_1 => am_10_1,
        Algoritmos::AM_10_01 => am_10_01,
        Algoritmos::AM_10_01_mejores => am_10_01_mejores,
    };

    let mut ejecuciones: Vec<InfoEjecucion> = Vec::new();
    let semillas = utils::Semillas::new();

    let ejecucion_total = Instant::now();
    for i in 0 .. 5 {
        let mut info = InfoEjecucion::new();
        let now = Instant::now();

        let cluster = funcion(&mut cluster, semillas.semilla(i));

        info.tiempo             = now.elapsed();
        info.tasa_inf           = cluster.infeasibility();
        info.desviacion_general = cluster.desviacion_general_particion();
        info.agr                = cluster.fitness();

        ejecuciones.push(info);

        cluster.reset_clusters();
    }

    println!("Benchmark completado en {} segundos {}\n", ejecucion_total.elapsed().as_secs(), "✓".green().bold());
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
        else if algoritmos.agg_un {
            let now = Instant::now();
            let mi_cluster = algorithm::agg_un(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("{}", &mi_cluster);
            println!("Fitness: {}", &mi_cluster.fitness());
            println!("Algoritmo genético AGG_UN calculado en {:?} ms\n", now.elapsed().as_millis());
        }
        else if algoritmos.agg_sf {
            let now = Instant::now();
            let mi_cluster = algorithm::agg_sf(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("{}", &mi_cluster);
            println!("Algoritmo genético AGG_SF calculado en {:?}\n", now.elapsed().as_millis());
        }
        else if algoritmos.age_un {
            let now = Instant::now();
            let mi_cluster = algorithm::age_un(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("{}", &mi_cluster);
            println!("Algoritmo genético AGE_UN calculado en {:?}\n", now.elapsed().as_millis());
        }
        else if algoritmos.age_sf {
            let now = Instant::now();
            let mi_cluster = algorithm::age_sf(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("{}", &mi_cluster);
            println!("Algoritmo genético AGE_SF calculado en {:?}\n", now.elapsed().as_millis());
        }
        else if algoritmos.am_10_1 {
            let now = Instant::now();
            let mi_cluster = algorithm::am_10_1(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("{}", &mi_cluster);
            println!("Algoritmo memético AM_10_1 calculado en {:?}\n", now.elapsed().as_millis());
        }
        else if algoritmos.am_10_01 {
            let now = Instant::now();
            let mi_cluster = algorithm::am_10_01(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("{}", &mi_cluster);
            println!("Algoritmo memético AM_10_01 calculado en {:?}\n", now.elapsed().as_millis());
        }
        else if algoritmos.am_10_01_mejores {
            let now = Instant::now();
            let mi_cluster = algorithm::am_10_01_mejores(&mut mi_cluster, utils::Semillas::new().semilla(0));
            println!("{}", &mi_cluster);
            println!("Algoritmo memético AM_10_01_mejores calculado en {:?}\n", now.elapsed().as_millis());
        }
    }
    else {
        if algoritmos.greedy {
            inicializador_benchmark(Algoritmos::Greedy);
        }

        if algoritmos.BL {
            inicializador_benchmark(Algoritmos::BL);
        }

        if algoritmos.agg_un {
            inicializador_benchmark(Algoritmos::AGG_UN);
        }

        if algoritmos.agg_sf {
            inicializador_benchmark(Algoritmos::AGG_SF);
        }

        if algoritmos.age_un {
            inicializador_benchmark(Algoritmos::AGE_UN);
        }

        if algoritmos.age_sf {
            inicializador_benchmark(Algoritmos::AGE_SF);
        }

        if algoritmos.am_10_1 {
            inicializador_benchmark(Algoritmos::AM_10_1);
        }

        if algoritmos.am_10_01 {
            inicializador_benchmark(Algoritmos::AM_10_01);
        }

        if algoritmos.am_10_01_mejores {
            inicializador_benchmark(Algoritmos::AM_10_01_mejores);
        }

    }
}