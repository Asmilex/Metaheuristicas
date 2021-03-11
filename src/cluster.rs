use crate::utils::*;
use nalgebra::*;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Clusters {
    num_clusters: usize,        // NOTE: los clusters empiezan en 1. Por defecto se tiene el cluster 0
    lista_clusters: Vec<usize>,    // lista_clusters contiene índices a los vectores del espacio. .len() = num_elementos
    recuento_clusters: Vec<usize>,

    centroides: Vec<Punto>,
    recalcular_centroides: bool,

    dim_vectores: usize,
    num_elementos: usize,
    espacio: Vec<Punto>,
    distancias: MatrizDinamica<f64>,

    restricciones_ML: Vec<Restriccion>,     // FIXME Actualmente en desuso. Se prefiere implementación en matriz debido a la presentación de los datos
    restricciones_CL: Vec<Restriccion>,     // FIXME ^

    restricciones: MatrizDinamica<i8>
}

impl Clusters {
    pub fn new(num_clusters: usize, dim_vectores: usize, num_elementos: usize) -> Clusters {
        Clusters {
            num_clusters,
            lista_clusters: vec![0; num_elementos],         // Array con los índices a los vectores del espacio.
            recuento_clusters: vec![0; num_clusters],         // Cuántos elementos tiene cada cluster.

            centroides: vec![DVector::zeros(dim_vectores); num_clusters],     // Tantos como clusters haya
            recalcular_centroides: true,

            dim_vectores,
            num_elementos,
            espacio: vec![DVector::zeros(dim_vectores); num_elementos],       // Vector de puntos aka matriz.
            distancias: DMatrix::from_diagonal_element(num_elementos, num_elementos, 0.0),  // Matriz de distancias entre puntos.

            restricciones_CL: Vec::new(),
            restricciones_ML: Vec::new(),

            restricciones: DMatrix::from_diagonal_element(num_elementos, num_elementos, 0)
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

    pub fn indices_cluster(&self, c: usize) -> Vec<usize> {
        /*
            Cada posición del vector lista_clusters corresponde con la misma de espacio, salvo que
            las entradas denotan en qué cluster están.
        */
        assert_ne!(0, c);

        let mut indices: Vec<usize> = Vec::new();

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
    // ───────────────────────────────────────────── DISTANCIA MEDIA INTRACLUSTER ─────
    //


    pub fn vector_distancias_medias_intracluster(&self) -> Vec<f64> {
        let mut dm_ic = vec![0.0; self.num_clusters];

        for i in 1 ..= self.num_clusters {
            let indices_cluster = self.indices_cluster(i);
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

    pub fn distancia_media_intracluster(&self, c: usize) -> f64 {
        self.vector_distancias_medias_intracluster()[c - 1]
    }

    pub fn desviacion_general_particion(&self) -> f64 {
        self.vector_distancias_medias_intracluster().iter().sum::<f64>() * 1.0/(self.num_clusters as f64)
    }

    //
    // ──────────────────────────────────────────────────────── MEDIDAS GENERALES ─────
    //

    pub fn infeasibility(&self) -> u8 {
        assert_eq!(self.espacio.len(), self.restricciones.len());
        assert_eq!(self.espacio.len(), self.lista_clusters.len());

        /*
            Calcular el número de restricciones violadas; esto es, dado un elemento de restricciones
                1  => deben estar en el mismo cluster.
                -1 => deben estar en distintos clusters.

                Si alguna de ellas es violada => infeasiblity++


                Clusters: []
        */

        let mut infeasibility: u8 = 0;

        // Matriz simétrica => tomamos solo triangular superior
        for i in 0 .. self.restricciones.len() {
            for j in i+1 .. self.restricciones.len() {
                match self.restricciones[(i,j)]                     // NOTE: echarle un ojo a a la eficiencia del match. Proponer if.
                {
                    1 => {
                        // Comprobar que ambos sí están en el mismo.
                        if self.lista_clusters[i] != self.lista_clusters[j] {       // Índices corresponden a los mismos que se encuetran en los clusters.
                            infeasibility = infeasibility + 1;
                        }
                    }
                    -1 => {
                        // Comprobar que no están presentes en el mismo.
                        if self.lista_clusters[i] == self.lista_clusters[j] {
                            infeasibility = infeasibility + 1;
                        }
                    }
                    _ => (),        // Otros valores; i.e. 0
                }
            }
        }

        infeasibility
    }

}