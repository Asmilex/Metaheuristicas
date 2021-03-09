use crate::utils::*;
use nalgebra::*;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Clusters {
    num_clusters: usize,
    lista_clusters: Vec<u8>,

    dim_vectores: usize,
    num_elementos: usize,
    espacio: Vec<Punto>,
    distancias: MatrizDinamica<f64>,

    restricciones_ML: Vec<Restriccion>,
    restricciones_CL: Vec<Restriccion>,
}

impl Clusters {
    pub fn new(n_clusters: usize, dim_vectores: usize, num_elementos: usize) -> Clusters {
        Clusters {
            num_clusters: n_clusters,
            lista_clusters: vec![0; num_elementos],

            dim_vectores: dim_vectores,
            num_elementos: num_elementos,
            espacio: vec![vec![0.0; dim_vectores]; num_elementos],
            distancias: DMatrix::from_diagonal_element(num_elementos, num_elementos, 0.0),

            restricciones_CL: Vec::new(),
            restricciones_ML: Vec::new()
        }
    }

    pub fn resize_espacio(&mut self, nuevo_tam: usize) {
        self.num_elementos = nuevo_tam;

        // Cambiar el resto de componentes
        self.espacio = vec![vec![0.0; self.dim_vectores]; self.num_elementos];
        self.distancias = DMatrix::from_diagonal_element(self.num_elementos, self.num_elementos, 0.0);
    }

    pub fn calcular_distancias(&mut self) {
        
    }
}