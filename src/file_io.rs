use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

pub fn leer_archivos_dir (directorio: &Path) -> Vec<PathBuf> {
    let mut vector_path:Vec<PathBuf> = Vec::new();

    if directorio.is_dir() {
        for entrada in fs::read_dir(directorio).unwrap() {
            vector_path.push(entrada.unwrap().path());
        }
    }
    vector_path
}

pub fn leer_MDG (archivo: &File) {
    
}