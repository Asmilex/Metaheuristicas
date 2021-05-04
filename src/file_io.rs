use std::{fs::*};
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead};
use std::error::Error;

use crate::cluster::*;
use crate::utils::*;

use nalgebra::{DVector, DMatrix};
use colored::*;


//
// ───────────────────────────────────────────────────────── LECTURA DE DATOS ─────
//

pub fn leer_archivos_dir (directorio: &Path) -> Vec<PathBuf> {
    let mut vector_path:Vec<PathBuf> = Vec::new();

    if directorio.is_dir() {
        for entrada in read_dir(directorio).unwrap() {
            vector_path.push(entrada.unwrap().path());
        }
    }
    vector_path
}


#[allow(non_snake_case)]
pub fn leer_archivo_PAR (parametros: &ParametrosDataset, restricciones_a_usar: &Restricciones) -> Clusters {
    /*
        Pasos a seguir:
        1. Cargar todos los puntos (se encuentran en los .dat)
        2. Cargar las restricciones (se encuentran en los .const)
    */

    println!("{} Comienza la lectura de los archivos", "🗘".yellow());

    let mut cluster = Clusters::new(parametros.clusters , parametros.atributos, parametros.instancias);

    let mut espacio: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_elementos];
    let mut sig_entrada: usize = 0; // Siguiente entrada a escribir del espacio


    // ────────────────────────────────────────────────────── LECTURA DEL ARCHIVO ─────

    println!("\t{} Se empieza a leer el archivo {:?}", "▸".yellow() , &parametros.archivo_datos);

    let f = File::open(&parametros.archivo_datos).unwrap();
    let reader = BufReader::new(f);

    for linea in reader.lines() {
        let mut punto: Punto = DVector::zeros(cluster.dim_vectores);
        let entradas_como_vector_str = linea.unwrap();

        if !entradas_como_vector_str.is_empty() {
            let entradas_como_vector_str: Vec<&str> = entradas_como_vector_str.split(',').collect();

            for i in 0 .. entradas_como_vector_str.len() {
                punto[(i)] = entradas_como_vector_str[i].parse().unwrap();
            }

            espacio[(sig_entrada)] = punto;
            sig_entrada = sig_entrada + 1;
        }
    }

    cluster.asignar_espacio(espacio);


    // ───────────────────────────────────────────── LECTURA DE LAS RESTRICCIONES ─────

    let ruta_archivo_restric = match restricciones_a_usar {
        Restricciones::Diez => parametros.archivo_restricciones_10.clone(),
        Restricciones::Veinte => parametros.archivo_restricciones_20.clone(),
    };

    println!("\t{} Se empiezan a leer las restricciones {:?}", "▸".yellow(), &ruta_archivo_restric);

    let mut restricciones: MatrizDinamica<i8> = DMatrix::from_element(parametros.instancias, parametros.instancias, 0);
    let mut fila: usize = 0;


    let f = File::open(ruta_archivo_restric).unwrap();
    let reader = BufReader::new(f);


    for linea in reader.lines() {
        let entradas_como_vector = linea.unwrap();

        if !entradas_como_vector.is_empty() {
            let entradas_como_vector: Vec<&str> = entradas_como_vector.split(',').collect();
            //let entradas_como_vector: Vec<i8> = linea.iter().flat_map(|x| x.parse()).collect();

            for columna in 0 .. entradas_como_vector.len() {
                restricciones[(fila, columna)] = entradas_como_vector[columna].parse().unwrap();
            }
            fila = fila + 1;
        }
    }

    cluster.asignar_matriz_restricciones(restricciones);

    println!("{} Finalizada la lectura del cluster {}\n", "🗘".yellow(), "✓".green());

    cluster
}

struct Datos {
    algoritmo: String,
    info: InfoEjecucion
}

fn analyze_dataset(path: &str) -> Vec<Datos> {
    let mut datos: Vec<Datos> = Vec::new();

    for p in leer_archivos_dir(Path::new(path)).iter() {
        datos.push( Datos {
            algoritmo: String::from(p.file_name().unwrap().to_str().unwrap()).replace(".csv", ""),
            info: hacer_media(p).unwrap()
        });
    }

    datos
}

pub fn analyze() -> Result<(), Box<dyn Error>> {
    let cabeceras = [
        "algoritmo", "tasa_inf", "desviacion_general", "agr", "tiempo",
    ];

    let escribir = |datos: Vec<Datos>, wtr: &mut csv::Writer<File>| -> Result<(), Box<dyn Error>> {
        wtr.write_record(&cabeceras)?;

        let mut record: Vec<String>;
        for d in datos.iter() {
            record = Vec::from([
                d.algoritmo.clone(),
                d.info.tasa_inf.to_string(),
                d.info.desviacion_general.to_string(),
                d.info.agr.to_string(),
                d.info.tiempo.to_string()
            ]);

            wtr.write_record(&record)?;
        }

        wtr.flush()?;

        Ok(())
    };

    // ─────────────────────────────────────────────────────────────────── ZOO 10 ─────

    let datos = analyze_dataset("./data/csv/zoo_10");
    let mut wtr = csv::Writer::from_path("./data/csv/zoo_10/analisis.csv")?;
    escribir(datos, &mut wtr)?;

    // ─────────────────────────────────────────────────────────────────── ZOO 20 ─────

    let datos = analyze_dataset("./data/csv/zoo_20");
    let mut wtr = csv::Writer::from_path("./data/csv/zoo_20/analisis.csv")?;
    escribir(datos, &mut wtr)?;

    // ───────────────────────────────────────────────────────────────── GLASS 10 ─────

    let datos = analyze_dataset("./data/csv/glass_10");
    let mut wtr = csv::Writer::from_path("./data/csv/glass_10/analisis.csv")?;
    escribir(datos, &mut wtr)?;

    // ───────────────────────────────────────────────────────────────── GLASS 20 ─────

    let datos = analyze_dataset("./data/csv/glass_20");
    let mut wtr = csv::Writer::from_path("./data/csv/glass_20/analisis.csv")?;
    escribir(datos, &mut wtr)?;

    // ────────────────────────────────────────────────────────────────── BUPA 10 ─────

    let datos = analyze_dataset("./data/csv/bupa_10");
    let mut wtr = csv::Writer::from_path("./data/csv/bupa_10/analisis.csv")?;
    escribir(datos, &mut wtr)?;

    // ────────────────────────────────────────────────────────────────── BUPA 20 ─────

    let datos = analyze_dataset("./data/csv/bupa_20");
    let mut wtr = csv::Writer::from_path("./data/csv/bupa_20/analisis.csv")?;
    escribir(datos, &mut wtr)?;


    Ok(())
}


pub fn hacer_media(archivo: &PathBuf) -> Result<InfoEjecucion, Box<dyn Error>> {
    let mut resultados: Vec<InfoEjecucion> = Vec::new();
    let mut reader = csv::Reader::from_path(archivo).unwrap();

    for result in reader.deserialize() {
        let record = result?;
        resultados.push(record);
    }

    let resultado = InfoEjecucion {
        agr: resultados.iter().fold(0.0, |suma: f64, valor| suma + valor.agr) * 1.0/resultados.len() as f64,
        desviacion_general: resultados.iter().fold(0.0, |suma: f64, valor| suma + valor.desviacion_general) * 1.0/resultados.len() as f64,
        tasa_inf: (resultados.iter().fold(0, |suma: u32, valor| suma + valor.tasa_inf) as f64 * 1.0/resultados.len() as f64).ceil() as u32,
        tiempo: (resultados.iter().fold(0, |suma: u128, valor| suma + valor.tiempo) as f64 * 1.0/resultados.len() as f64).ceil() as u128
    };


    Ok(resultado)
}


pub fn parse_arguments(args: &Vec<String>) -> Result<(Option<ParametrosDataset>, Option<Restricciones>, AlgoritmosAEjecutar), &'static str> {
    if args.contains(&String::from("analyze")) {
        match analyze() {
            Ok(_) => {
                println!("{}", "Se han analizado correctamente todos los benchmarks. Comprueba las carpetas que se encuentran en ./csv".green());
                std::process::exit(0)
            },
            Err(r) => {
                println!("{}: {}", "Error al procesar los archivos".red(), r);
                std::process::exit(1)
            }
        }
    }

    // ─────────────────────────────────────────────────────────────── ALGORITMOS ─────

    let mut algoritmos =  AlgoritmosAEjecutar::new();

    if args.contains(&String::from("benchmark")) {
        algoritmos.benchmark = true;
    }

    // Si no se especifican algoritmos, ejecutarlos todos
    if      !args.contains(&String::from("greedy")) && !args.contains(&String::from("bl"))
        &&  !args.contains(&String::from("agg_un")) && !args.contains(&String::from("agg_sf")) && !args.contains(&String::from("age_un")) && !args.contains(&String::from("age_sf"))
        &&  !args.contains(&String::from("geneticos"))
        &&  !args.contains(&String::from("memeticos"))
        &&  !args.contains(&String::from("am_10_1")) && !args.contains(&String::from("am_10_01")) && !args.contains(&String::from("am_10_01_mejores")) {

        algoritmos.greedy = true;
        algoritmos.BL     = true;

        algoritmos.age_sf = true;
        algoritmos.age_un = true;
        algoritmos.agg_sf = true;
        algoritmos.agg_un = true;

        algoritmos.am_10_1          = true;
        algoritmos.am_10_01         = true;
        algoritmos.am_10_01_mejores = true;
    }
    else if args.contains(&String::from("geneticos")) {
        algoritmos.age_sf = true;
        algoritmos.age_un = true;
        algoritmos.agg_sf = true;
        algoritmos.agg_un = true;
    }
    else if args.contains(&String::from("memeticos")) {
        algoritmos.am_10_1          = true;
        algoritmos.am_10_01         = true;
        algoritmos.am_10_01_mejores = true;
    }
    else {
        // En caso contrario, seleccionar aquellos que sí que se usarán
        if args.contains(&String::from("greedy")) {
            algoritmos.greedy = true
        }
        if args.contains(&String::from("bl")) {
            algoritmos.BL = true
        }

        if args.contains(&String::from("agg_un")) {
            algoritmos.agg_un = true;
        }

        if args.contains(&String::from("agg_sf")) {
            algoritmos.agg_sf = true;
        }

        if args.contains(&String::from("age_un")) {
            algoritmos.age_un = true;
        }

        if args.contains(&String::from("age_sf")) {
            algoritmos.age_sf = true;
        }

        if args.contains(&String::from("am_10_1")) {
            algoritmos.am_10_1 = true;
        }

        if args.contains(&String::from("am_10_01")) {
            algoritmos.am_10_01 = true;
        }

        if args.contains(&String::from("am_10_01_mejores")) {
            algoritmos.am_10_01_mejores = true;
        }
    }

    // ─────────────────────────────────────────────────────────────── PARAMETROS ─────

    if !algoritmos.benchmark {
        // Al no haber escogido benchmark, hay que especificar el tipo de archivo y las restricciones
        let parametros: ParametrosDataset;

        if args.contains(&String::from("zoo")) {
            parametros = ParametrosDataset::new(Datasets::Zoo);
        }
        else if args.contains(&String::from("glass")) {
            parametros = ParametrosDataset::new(Datasets::Glass);
        }
        else if args.contains(&String::from("bupa")) {
            parametros = ParametrosDataset::new(Datasets::Bupa);
        }
        else {
            return Err("No se ha especificado el conjunto de datos a resolver ni especificado un benchmark");
        }

        // ──────────────────────────────────────────────────────────── RESTRICCIONES ─────

        let restricciones: Restricciones;

        if args.contains(&String::from("10")) {
            restricciones = Restricciones::Diez;
        }
        else if args.contains(&String::from("20")) {
            restricciones = Restricciones::Veinte;
        }
        else {
            return Err("No se ha proporcionado el archivo de restricciones que usar. Posibilidades: {10, 20}");
        }

        Ok((Some(parametros), Some(restricciones), algoritmos))
    }
    else {
        Ok((None, None, algoritmos))
    }
}


//
// ─────────────────────────────────────────────────────────────────── SALIDA ─────
//

pub fn export_to_csv (info: &Vec<InfoEjecucion>, path: &str) ->  Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;

    // When writing records without Serde, the header record is written just
    // like any other record.
    wtr.write_record(&[
        "tasa_inf", "desviacion_general", "agr", "tiempo",
    ])?;

    let mut record: Vec<String>;
    for bench in info.iter() {
        record = Vec::from([
            bench.tasa_inf.to_string(),
            bench.desviacion_general.to_string(),
            bench.agr.to_string(),
            bench.tiempo.to_string()
        ]);

        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}
