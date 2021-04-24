use std::fs::*;
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


pub fn parse_arguments(args: &Vec<String>) -> Result<(Option<ParametrosDataset>, Option<Restricciones>, AlgoritmosAEjecutar), &'static str> {

    // ─────────────────────────────────────────────────────────────── ALGORITMOS ─────

    let mut algoritmos =  AlgoritmosAEjecutar::new();

    if args.contains(&String::from("benchmark")) {
        algoritmos.benchmark = true;
    }

    // Si no se especifican algoritmos, ejecutarlos todos
    if      !args.contains(&String::from("greedy")) && !args.contains(&String::from("bl"))
        &&  !args.contains(&String::from("agg_un")) && !args.contains(&String::from("agg_sf")) && !args.contains(&String::from("age_un")) && !args.contains(&String::from("age_sf"))
        &&  !args.contains(&String::from("geneticos"))  {

        algoritmos.greedy = true;
        algoritmos.BL     = true;

        algoritmos.age_sf = true;
        algoritmos.age_un = true;
        algoritmos.agg_sf = true;
        algoritmos.agg_un = true;
    }
    else if args.contains(&String::from("geneticos")) {
        algoritmos.age_sf = true;
        algoritmos.age_un = true;
        algoritmos.agg_sf = true;
        algoritmos.agg_un = true;
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
            algoritmos.age_un = true;
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
        "Tasa infeasibility", "Desviación general", "Agregado", "Tiempo de ejecución (ms)",
    ])?;

    let mut record: Vec<String>;
    for bench in info.iter() {
        record = Vec::from([
            bench.tasa_inf.to_string(),
            bench.desviacion_general.to_string(),
            bench.agr.to_string(),
            bench.tiempo.as_millis().to_string()
        ]);

        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}
