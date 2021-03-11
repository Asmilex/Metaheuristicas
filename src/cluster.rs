use crate::utils::*;
use nalgebra::*;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Clusters {
    num_clusters: usize,        // NOTE: los clusters empiezan en 1. Por defecto se tiene el cluster 0
    lista_clusters: Vec<u8>,    // lista_clusters contiene índices a los vectores del espacio. .len() = num_elementos
    recuento_clusters: Vec<usize>,

    centroides: Vec<Punto>,
    recalcular_centroides: bool,

    dim_vectores: usize,
    num_elementos: usize,
    espacio: Vec<Punto>,
    distancias: MatrizDinamica<f64>,

    restricciones_ML: Vec<Restriccion>,
    restricciones_CL: Vec<Restriccion>,
}

impl Clusters {
    pub fn new(n_clusters: usize, dim_vectores: usize, num_elementos: usize, num_clusters: usize) -> Clusters {
        Clusters {
            num_clusters: n_clusters,
            lista_clusters: vec![0; num_elementos],
            recuento_clusters: vec![0; n_clusters],

            centroides: vec![DVector::zeros(dim_vectores); num_clusters],
            recalcular_centroides: true,

            dim_vectores: dim_vectores,
            num_elementos: num_elementos,
            espacio: vec![DVector::zeros(dim_vectores); num_elementos],
            distancias: DMatrix::from_diagonal_element(num_elementos, num_elementos, 0.0),

            restricciones_CL: Vec::new(),
            restricciones_ML: Vec::new()
        }
    }

//
// ─── ESPACIO ────────────────────────────────────────────────────────────────────
//


    pub fn resize_espacio(&mut self, nuevo_tam: usize) {
        self.num_elementos = nuevo_tam;

        // Cambiar el resto de componentes
        self.espacio = vec![DVector::zeros(self.dim_vectores); self.num_elementos];
        self.distancias = DMatrix::from_diagonal_element(self.num_elementos, self.num_elementos, 0.0);
    }

    pub fn calcular_matriz_distancias(&mut self) {
        // TODO
    }

//
// ─── CLUSTERS ───────────────────────────────────────────────────────────────────
//

    //
    // ──────────────────────────────────────────────────────────────── ELEMENTOS ─────
    //

    pub fn indices_cluster(&self, c: u8) -> Vec<u8> {
        /*
            Cada posición del vector lista_clusters corresponde con la misma de espacio, salvo que
            las entradas denotan en qué cluster están.
        */
        assert_ne!(0, c);

        let mut indices = Vec::new();
        for i in 0..self.lista_clusters.len() {
            if self.lista_clusters[i] == c {
                indices.push(self.lista_clusters[i])
            }
        }

        indices
    }


    fn elementos_en_cluster(&self, cluster: usize) -> usize {
        self.recuento_clusters[cluster-1]
    }


    //
    // ─────────────────────────────────────────────────────────────── CENTROIDES ─────
    //


    pub fn centroide_cluster(&mut self, c: u8) -> &Punto {
        assert_ne!(c, 0);
        &self.centroides[c as usize -1]
    }

    pub fn vector_centroides(&mut self) -> &Vec<Punto> {
        &self.centroides
    }

    fn calcular_centroides(&mut self) {
        for i in 0..self.lista_clusters.len() {
            if self.lista_clusters[i] != 0 {
                // Clusters 1, .., num_clusters => i - 1 va desde 0 hasta num_clusters - 1. Memoria reservada previamente.
                self.centroides[(self.lista_clusters[i] - 1) as usize] += &self.espacio[i];
                self.recuento_clusters[(self.lista_clusters[i] -1 ) as usize] = self.recuento_clusters[(self.lista_clusters[i] - 1) as usize] + 1;
            }
        }

        for i in 0..self.num_clusters {
            self.centroides[(i)] = &self.centroides[(i)] * (1.0/(self.recuento_clusters[i]) as f64);
        }

        self.recalcular_centroides = false;
        dbg!("Centroides recalculados");
    }

    //
    // ──────────────────────────────────────────────────────────── OTRAS MEDIDAS ─────
    //


    pub fn distancia_media_intracluster(&self) -> Vec<f64> {
        let mut dm_ic = vec![0.0; self.num_clusters];

        for i in 1..=self.num_clusters {
            let indices_cluster = self.indices_cluster(i as u8);
            let cent = &self.centroides[i - 1];

         /*    let mut suma_distancias = 0.0;
            for i in 0..indices_cluster.len() {
                suma_distancias = suma_distancias + distancia(&self.espacio[(indices_cluster[i] as usize)].clone(), &cent);
            }
            dm_ic[i - 1] = suma_distancias * 1.0/self.elementos_en_cluster(i) as f64;
            */

            dm_ic[i-1] = indices_cluster.iter()
                .map(
                    |&indice|
                    distancia(&self.espacio[(indice as usize)], &cent)
                ).sum();
        }

        dm_ic
    }

    pub fn infeasibility(&self) -> u8 {
        assert_eq!(self.espacio.len(), self.restricciones.len());

        // Sacar todos los índices de los clusters
        let mut indices_clusters: Vec<Vec<usize>> = Vec::with_capacity(self.num_clusters);

        for i in 0 .. indices_clusters.len() {
            indices_clusters[i] = self.indices_cluster(i + 1) as Vec<usize>;
        }

        /*
            Calcular el número de restricciones violadas; esto es, dado un elemento de restricciones
                1  => deben estar en el mismo cluster.
                -1 => deben estar en distintos clusters.

                Si alguna de ellas es violada => infeasiblity++


                Clusters: []
        */

        let mut infeasibility: u8 = 0;

        for i in 0 .. self.restricciones.len() {
            for j in 0 .. self.restricciones.len() {
                match self.restricciones[(i,j)]                     // NOTE: echarle un ojo a a la eficiencia del match. Proponer if.
                {
                    1 => {
                        // Comprobar que ambos sí están en el mismo.
                        for k in 0 .. indices_clusters.len() {
                            if indices_clusters[k].contains(&i) && !indices_clusters[k].contains(&j) {
                                infeasibility = infeasibility + 1;
                                break;
                            }
                        }
                    }
                    -1 => {
                        // Comprobar que no están presentes en el mismo.
                        for k in 0 .. indices_clusters.len() {
                            if indices_clusters[k].contains(&i) && indices_clusters[k].contains(&j) {
                                infeasibility = infeasibility + 1;
                                break;
                            }
                        }
                    }
                    _ => (),        // Otros valores; i.e. 0
                }
            }
        }

        infeasibility
    }

}