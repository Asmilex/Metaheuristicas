use std::fs::*;
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead};

use crate::cluster::*;
use crate::utils::*;

use nalgebra::{DVector, DMatrix};


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
pub fn leer_archivo_PAR (ruta_archivo_vectores: &PathBuf, ruta_archivo_restric: &PathBuf) -> Clusters {
    /*
    Pasos a seguir:
    1. Determinar si el archivo es de tipo Bupa, Glass o Zoo.
    2. Cargar todos los puntos (se encuentran en los .dat)
    3. Cargar las restricciones (se encuentran en los .const)
    */

    println!("Comienza la lectura de los archivos");
    println!("\t▸ Se deciden los parámetros del cluster");

    // ──────────────────────────────────────────────────── DETERMINAR PARAMETROS ─────


    let parametros: PAR_parametros = if ruta_archivo_vectores.as_os_str().to_str().unwrap().contains("bupa") {
        PAR_parametros::new(PAR_nombres::Bupa)
    } else if ruta_archivo_vectores.as_os_str().to_str().unwrap().contains("glass"){
        PAR_parametros::new(PAR_nombres::Glass)
    } else {
        PAR_parametros::new(PAR_nombres::Zoo)
    };

    let mut cluster = Clusters::new(parametros.clusters , parametros.atributos, parametros.instancias);

    let mut espacio: Vec<Punto> = vec![DVector::zeros(cluster.dim_vectores); cluster.num_elementos];
    let mut sig_entrada: usize = 0; // Siguiente entrada a escribir del espacio


    // ────────────────────────────────────────────────────── LECTURA DEL ARCHIVO ─────


    let f = File::open(ruta_archivo_vectores).unwrap();
    let reader = BufReader::new(f);

    println!("\t▸ Se empieza a leer el archivo {:?}", &ruta_archivo_vectores);

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


    let mut restricciones: MatrizDinamica<i8> = DMatrix::from_element(parametros.instancias, parametros.instancias, 0);
    let mut fila: usize = 0;

    let f = File::open(ruta_archivo_restric).unwrap();
    let reader = BufReader::new(f);

    println!("\t▸ Se empiezan a leer las restricciones {:?}", &ruta_archivo_restric);

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

    println!("Finalizada la lectura del cluster ✓\n");

    cluster
}