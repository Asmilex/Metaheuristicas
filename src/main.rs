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
    use utils::{Algoritmos, PAR_nombres, PAR_parametros};
    use std::env;

// ────────────────────────────────────────────────────────────────────────────────

fn parse_arguments(args: Vec<String>) -> Result<(PAR_parametros, Algoritmos, usize), &'static str> {

    // ─────────────────────────────────────────────────────────────── PARAMETROS ─────

    let parametros: PAR_parametros;

    if args.contains(&String::from("zoo")) {
        parametros = PAR_parametros::new(PAR_nombres::Zoo);
    }
    else if args.contains(&String::from("glass")) {
        parametros = PAR_parametros::new(PAR_nombres::Glass);
    }
    else if args.contains(&String::from("bupa")) {
        parametros = PAR_parametros::new(PAR_nombres::Bupa);
    }
    else {
        return Err("No se ha especificado el conjunto de datos a resolver");
    }

    // ─────────────────────────────────────────────────────────────── ALGORITMOS ─────

    let mut algoritmos =  Algoritmos::new();

    if args.contains(&String::from("greedy")) {
        algoritmos.greedy = true
    }
    if args.contains(&String::from("BL")) {
        algoritmos.BL = true
    }

    // ──────────────────────────────────────────────────────────── RESTRICCIONES ─────

    let restricciones: usize;

    if args.contains(&String::from("10")) {
        restricciones = 10;
    }
    else if args.contains(&String::from("20")) {
        restricciones = 20;
    }
    else {
        return Err("No se ha proporcionado el archivo de restricciones que usar. Posibilidades: {10, 20}");
    }


    Ok((parametros, algoritmos, restricciones))
}

fn main() {
    let (parametros, algoritmo, restricciones) = match parse_arguments(env::args().collect()) {
        Ok(r) => r,
        Err(err) => panic!("No se han introducido correctamente los parámetros de entrada. Error: {}", err)
    };

    let mut mi_cluster = leer_archivo_PAR(parametros, restricciones);

    if algoritmo.greedy {
        let now = Instant::now();
        let mi_cluster = algorithm::greedy_COPKM(&mut mi_cluster);
        println!("{}", &mi_cluster);
        println!("Greedy calculado en {:?}\n", now.elapsed());
    }


}