use crate::utils::*;

use nalgebra::*;
use multimap::MultiMap;

use std::{fmt};

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
    maximo_distancias: f64,

    restricciones: MatrizDinamica<i8>,

    restricciones_ML: MultiMap<usize, usize>,
    restricciones_CL: MultiMap<usize, usize>,

    pub num_restricciones: usize
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
            maximo_distancias: 0.0,

            restricciones: DMatrix::from_diagonal_element(num_elementos, num_elementos, 0),

            restricciones_ML: MultiMap::new(),
            restricciones_CL: MultiMap::new(),

            num_restricciones: 0
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

        self.maximo_distancias = self.distancias.max();
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
        self.calcular_matriz_distancias();
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
            for j in i+1..self.restricciones.ncols() {
                match self.restricciones[(i, j)]
                {
                    1 => {
                        self.restricciones_ML.insert(i, j);
                        self.restricciones_ML.insert(j, i);
                        self.num_restricciones = self.num_restricciones + 1;
                    }
                    -1 => {
                        self.restricciones_CL.insert(i, j);
                        self.restricciones_CL.insert(j, i);
                        self.num_restricciones = self.num_restricciones + 1;
                    }
                    _ => ()
                }
            }
        }
    }


//
// ─── CLUSTERS ───────────────────────────────────────────────────────────────────
//


    pub fn clusters(&self) -> &Vec<usize> {
        &self.lista_clusters
    }

    pub fn asignar_clusters(&mut self, clusters: Vec<usize>) {
        if clusters.len() != self.lista_clusters.len() {
            panic!("La longitud de la lista pasada no es la indicada (debería ser {}, es {})", self.lista_clusters.len(), clusters.len());
        }

        self.lista_clusters = clusters;

        // Recontar los elementos
        self.recuento_clusters.fill(0);
        for c in self.lista_clusters.iter() {
            self.recuento_clusters[c-1] = self.recuento_clusters[c-1] + 1;
        }
    }


    pub fn reset_clusters(&mut self) {
        self.lista_clusters = vec![0; self.num_elementos];
        self.centroides =  vec![DVector::zeros(self.dim_vectores); self.num_clusters];
    }


    pub fn solucion_valida(&self) -> bool {
        !self.recuento_clusters.iter().any(|&valor| valor == 0)
    }


    pub fn indices_cluster(&self, c: usize) -> Vec<usize> {
        /*
            Cada posición del vector lista_clusters corresponde con la misma de espacio, salvo que
            las entradas denotan en qué cluster están.
        */
        assert_ne!(0, c);

        self.lista_clusters
            .iter()
            .enumerate()                               // Pares (índice, valor)
            .filter(|&(_indice, valor)| *valor == c)  // Filtrar por aquellos que están en el cluster
            .map(|(indice, _)| indice)                   // Quedarnos con los índices
            .collect()                                                                 // Recogerlos y devolver el valor
    }


    pub fn cluster_de_indice (&self, i: usize) -> usize {
        self.lista_clusters[i]
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


    pub fn centroide_cluster(&self, c: usize) -> Punto {
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
        if !self.solucion_valida() {
            println!("Existen elementos que no tienen cluster asignado. No se ejecuta nada - calcular_centroides");
            return
        }


        for centroide in self.centroides.iter_mut() {
            centroide.fill(0.0);
        }

        for i_centroide in 0 .. self.num_clusters {

            // Si hay alguien en ese cluster, sumar los elementos del espacio y dividirlos por la cantidad de elementos que hay
            for indice_elemento in self.indices_cluster(i_centroide+1).iter() {
                self.centroides[i_centroide] = &self.centroides[i_centroide] + (&self.espacio[*indice_elemento]);
            }

            self.centroides[i_centroide] = self.centroides[i_centroide].scale(1.0/(self.recuento_clusters[i_centroide] as f64));
        }
    }

    //
    // ───────────────────────────────────────────── DISTANCIA MEDIA INTRACLUSTER ─────
    //


    pub fn vector_distancias_medias_intracluster(&mut self) -> Vec<f64> {
        self.calcular_centroides();
        let mut dm_ic = vec![0.0; self.num_clusters];

        for i in 0 .. self.num_clusters {
            let centroide = self.centroide_cluster(i+1);
            for indice in self.indices_cluster(i+1).iter() {
                dm_ic[i] = dm_ic[i] + distancia(&self.espacio[*indice], &centroide);
            }

            dm_ic[i] = dm_ic[i] * 1.0/self.elementos_en_cluster(i+1) as f64;
        }

        dm_ic
    }


    pub fn distancia_media_intracluster(&mut self, c: usize) -> f64 {
        self.vector_distancias_medias_intracluster()[c - 1]
    }


    pub fn desviacion_general_particion(&mut self) -> f64 {
        self.vector_distancias_medias_intracluster().iter().sum::<f64>() * 1.0/(self.num_clusters as f64)
    }

//
// ─── MEDIDAS GENERALES ──────────────────────────────────────────────────────────
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

        let mut expected_infeasibility: u32 = 0;

        // Calcular cuántas se violan, y sumarlas.
        match self.restricciones_ML.get_vec(&indice) {
            Some(restricciones) =>
                expected_infeasibility =
                    restricciones.iter()
                    .filter(|&restriccion| c != self.lista_clusters[*restriccion])
                    .count() as u32,
            None => ()
        }

        match self.restricciones_CL.get_vec(&indice) {
            Some(restricciones) =>
                expected_infeasibility = expected_infeasibility +
                    restricciones.iter()
                    .filter(|&restriccion| c == self.lista_clusters[*restriccion])
                    .count() as u32,
            None => ()
        }

        expected_infeasibility
    }


    pub fn lambda(&self) -> f64 {
        self.maximo_distancias/self.num_restricciones as f64
    }


    pub fn fitness(&mut self) -> f64 {
        self.desviacion_general_particion() + self.lambda() * self.infeasibility() as f64
    }

    //
    // ─── ESPECIFICOS ────────────────────────────────────────────────────────────────
    //

    pub fn bl_fitness_posible_sol(&mut self, i: usize, c: usize, antiguo_infeas: u32) -> Result<f64, &'static str> {
        /*
            Computa si la solución que ocurre de asginar el cluster c al vector en la posición c es válido. En ese caso, devuelve su fitness
        */
        //use std::time::{Instant};
        //let now = Instant::now();

        let antiguo_c = self.lista_clusters[i];

        self.asignar_cluster_a_elemento(i, c);

        if !self.solucion_valida() {
            self.asignar_cluster_a_elemento(i, antiguo_c);
            return Err("La solución no es válida");
        }

        let nuevo_infeas = antiguo_infeas - self.infeasibility_delta_esperada(i, antiguo_c) + self.infeasibility_delta_esperada(i, c);
        let fitness = self.desviacion_general_particion() + self.lambda()*nuevo_infeas as f64;

        self.asignar_cluster_a_elemento(i, antiguo_c);

        //println!("Tiempo en bl_fitness_posible_sol: {}", now.elapsed().as_millis());
        Ok(fitness)
    }
}

//
// ──────────────────────────────────────────────────────────────── FORMATTEO ─────
//


impl fmt::Display for Clusters {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln! (
            f,
            "Información del cluster:
            ▸ Número de clusters: {}
            ▸ Lista con los clusters: {:?}
            ▸ Elementos en cada cluster: {:?}
            ▸ Número de restricciones: {}
            ▸ Elementos en el espacio: {:?}
            ▸ Centroides:",
            self.num_clusters,
            self.lista_clusters,
            self.recuento_clusters,
            self.num_restricciones,
            self.num_elementos,
        )?;

        for centroide in self.centroides.iter() {
            writeln!(f, "\t{}", centroide)?;
        }
        writeln!(f, " ▸ Infeasibility: {}", self.infeasibility())?;
        Ok(())
    }
}