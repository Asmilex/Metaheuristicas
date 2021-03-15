use crate::utils::*;
use nalgebra::*;
use multimap::MultiMap;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Clusters {
    pub num_clusters: usize,        // NOTE: los clusters empiezan en 1. Por defecto se tiene el cluster 0
    lista_clusters: Vec<usize>,     // lista_clusters contiene índices a los vectores del espacio. .len() = num_elementos
    recuento_clusters: Vec<usize>,

    centroides: Vec<Punto>,

    pub dim_vectores: usize,        // Atributos presentes en el vector
    pub num_elementos: usize,       // Tamaño del espacio
    pub espacio: Vec<Punto>,
    pub distancias: MatrizDinamica<f64>,

    restricciones_ML: MultiMap<usize, usize>,
    restricciones_CL: MultiMap<usize, usize>,

    restricciones: MatrizDinamica<i8>
}

impl Clusters {
    pub fn new(num_clusters: usize, dim_vectores: usize, num_elementos: usize) -> Clusters {
        Clusters {
            num_clusters,
            lista_clusters: vec![0; num_elementos],           // Array con los índices a los vectores del espacio.
            recuento_clusters: vec![0; num_clusters],         // Cuántos elementos tiene cada cluster.

            centroides: vec![DVector::zeros(dim_vectores); num_clusters],     // Tantos como clusters haya

            dim_vectores,
            num_elementos,
            espacio: vec![DVector::zeros(dim_vectores); num_elementos],       // Vector de puntos aka matriz.
            distancias: DMatrix::from_diagonal_element(num_elementos, num_elementos, 0.0),  // Matriz de distancias entre puntos.

            restricciones_CL: MultiMap::new(),
            restricciones_ML: MultiMap::new(),

            restricciones: DMatrix::from_diagonal_element(num_elementos, num_elementos, 0)
        }
    }

//
// ─── ESPACIO ────────────────────────────────────────────────────────────────────
//


/*     pub fn resize_espacio(&mut self, nuevo_tam: usize) {
        self.num_elementos = nuevo_tam;

        // Cambiar el resto de componentes
        self.espacio = vec![DVector::zeros(self.dim_vectores); self.num_elementos];
        self.distancias = DMatrix::from_diagonal_element(self.num_elementos, self.num_elementos, 0.0);
    }
 */

    pub fn calcular_matriz_distancias(&mut self) {
        for i in 0 .. self.espacio.len() {
            for j in i+1 .. self.espacio.len() {
                self.distancias[(i, j)] = distancia(&self.espacio[(i)], &self.espacio[(j)]);
            }
        }
    }


    /*
        NOTE: consume el vector, dejándolo inutilizado para el resto del programa
    */
    pub fn asignar_vector(&mut self, vector: Punto, posicion: usize) {
        if posicion >= self.espacio.len() {
            panic!("La posición descrita no se encuentra en el rango del espacio");
        }

        self.espacio[(posicion)] = vector;
    }


    /*
        NOTE: consume el vector, dejándolo inutilizado para el resto del programa
    */
    pub fn asignar_espacio(&mut self, nuevo_espacio: Vec<Punto>) {
        if nuevo_espacio.len() != self.espacio.len() {
            println!("PROBLEMA: el nuevo espacio asignado no tiene el mismo tamaño que el inicializado");
        }

        self.espacio = nuevo_espacio;
    }

    //
    // ──────────────────────────────────────────────────────────── RESTRICCIONES ─────
    //

    /*
        NOTE: consume el vector, dejándolo inutilizado para el resto del programa
    */
    pub fn asignar_matriz_restricciones(&mut self, nuevas_restricciones: MatrizDinamica<i8>) {
        if    nuevas_restricciones.nrows() != self.restricciones.nrows()
           || nuevas_restricciones.ncols() != self.restricciones.ncols() {

            println!("PROBLEMA: la dimensión de las filas y las columnas asignadas difiere de la existente");
        }

        self.restricciones = nuevas_restricciones;

        for i in 0..self.restricciones.nrows() {
            for j in 0..self.restricciones.ncols() {
                match self.restricciones[(i, j)]
                {
                    1 => {
                        self.restricciones_ML.insert(i, j);
                    }
                    -1 => {
                        self.restricciones_CL.insert(i, j);
                    }
                    _ => ()
                }
            }
        }

        println!("Hashmap ML del primero vector: {:?}", self.restricciones_ML.get_vec(&0));
    }


//
// ─── CLUSTERS ───────────────────────────────────────────────────────────────────
//


    pub fn clusters(&self) -> &Vec<usize> {
        &self.lista_clusters
    }


    //
    // ──────────────────────────────────────────────────────────────── ELEMENTOS ─────
    //

    pub fn indices_cluster(&self, c: usize) -> Vec<usize> {
        /*
            Cada posición del vector lista_clusters corresponde con la misma de espacio, salvo que
            las entradas denotan en qué cluster están.
        */
        assert_ne!(0, c);

        self.lista_clusters.iter().filter(|&valor| *valor == c).copied().collect()
    }


    fn elementos_en_cluster(&self, cluster: usize) -> usize {
        self.recuento_clusters[cluster-1]
    }


    pub fn asignar_cluster_a_elemento (&mut self, i: usize, c: usize) {
        if i > self.lista_clusters.len() {
            panic!("El índice pasado se sale del espacio");
        }
        if c > self.num_clusters {
            panic!("El cluster pasado se sale del espacio");
        }

        if self.lista_clusters[i] > 0 {
            self.recuento_clusters[self.lista_clusters[i] - 1] = self.recuento_clusters[self.lista_clusters[i] - 1] - 1;
        }

        self.lista_clusters[i] = c;
        self.recuento_clusters[c-1] = self.recuento_clusters[c-1] + 1;
    }


    //
    // ─────────────────────────────────────────────────────────────── CENTROIDES ─────
    //


    pub fn centroide_cluster(&mut self, c: usize) -> Punto {
        assert_ne!(c, 0);
        self.centroides[c - 1].clone()
    }


    pub fn vector_centroides(&mut self) -> &Vec<Punto> {
        &self.centroides
    }


    pub fn asignar_centroides(&mut self, nuevos_centroides: Vec<Punto>) {
        if nuevos_centroides.len() != self.centroides.len() {
            println!("PROBLEMA: los nuevos centroides asignados tienen distinto tamaño al esperado");
        }

        self.centroides = nuevos_centroides;
    }


    pub fn calcular_centroides(&mut self) {
        if self.lista_clusters.iter().any(|&x| x == 0) {
            println!("Existen elementos que no tienen cluster asignado. No se ejecuta nada - calcular_centroides");
            return
        }

        for centroide in self.centroides.iter_mut() {
            for i in 0..centroide.nrows() {
                centroide[(i)] = 0.0;
            }
        }

        for i in 0..self.lista_clusters.len() {
            // Clusters 1, .., num_clusters => i - 1 va desde 0 hasta num_clusters - 1. Memoria reservada previamente.
            // NOTE: condición de que no sea 0 asegurada arriba.
            self.centroides[(self.lista_clusters[i] - 1) as usize] += &self.espacio[i];
        }

        for i in 0..self.num_clusters {
            self.centroides[(i)] = &self.centroides[(i)] * (1.0/(self.recuento_clusters[i]) as f64);
        }
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

    pub fn infeasibility(&self) -> u32 {
        assert_eq!(self.espacio.len(), self.restricciones.nrows());
        assert_eq!(self.restricciones.nrows(), self.restricciones.ncols());
        assert_eq!(self.espacio.len(), self.lista_clusters.len());

        /*
            Calcular el número de restricciones violadas; esto es, dado un elemento de restricciones
                1  => deben estar en el mismo cluster.
                -1 => deben estar en distintos clusters.

                Si alguna de ellas es violada => infeasiblity++
        */

        let mut infeasibility: u32 = 0;

        // Matriz simétrica => tomamos solo triangular superior
        for i in 0 .. self.restricciones.nrows() {
            for j in i+1 .. self.restricciones.ncols() {
                match self.restricciones[(i,j)]
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

    pub fn infeasibility_delta_esperada (&mut self, indice: usize, c: usize) -> u32 {
        if c > self.num_clusters {
            panic!("Cluster mayor del que se esperaba");
        }
        if indice > self.num_elementos {
            panic!("Índice mayor del que se esperaba");
        }

        // NOTE
        // El incremento que se produce en la infeasibility es independiente del estado del resto del sistema (¿Creo?)
        // Por tanto, es suficiente comprobar cuáles se violan al colocar el índice en un cierto cluster.

        let expected_infeasibility: u32 =
               self.restricciones_ML.get_vec(&indice).unwrap()      // Lista de restricciones correspondientes al índice
                .iter()
                .filter(|&restriccion| c != self.lista_clusters[*restriccion])
                .count() as u32                                               // Contamos aquellas que no se cumplen si ponemos que su cluster es el c
            +  self.restricciones_CL.get_vec(&indice).unwrap()     // Sumamos tanto las CL como las ML
                .iter()
                .filter(|&restriccion| c == self.lista_clusters[*restriccion])
                .count() as u32;

        expected_infeasibility
    }
}

//
// ──────────────────────────────────────────────────────────────── FORMATTEO ─────
//


impl fmt::Display for Clusters {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! (
            f,
            "Información del cluster:
                \n\t▸ Número de clusters: {:?}
                \n\t▸ Lista con los clusters: {:?}
                \n\t▸ Elementos en cada cluster: {:?}
                \n\t▸ Centroides: {:#?}
                \n\t▸ Elementos en el espacio: {:?}",
            self.num_clusters,
            self.lista_clusters,
            self.recuento_clusters,
            self.centroides,
            self.num_elementos
        )
    }
}