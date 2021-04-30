use nalgebra::*;
use std::path::{PathBuf};
use std::time;

//
// ───────────────────────────────────────────────────── TIPOS PERSONALIZADOS ─────
//


    pub type Punto = DVector<f64>;
    pub type MatrizDinamica<Tipo> = MatrixMN<Tipo, Dynamic, Dynamic>;


//
// ────────────────────────────────────────────────────── ENUMS Y ESTRUCTURAS ─────
//

    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub enum Datasets {
        Bupa,
        Glass,
        Zoo
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum Restricciones {
        Diez,
        Veinte
    }


    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub struct ParametrosDataset {
        pub tipo: Datasets,

        pub archivo_datos: PathBuf,
        pub archivo_restricciones_10: PathBuf,
        pub archivo_restricciones_20: PathBuf,

        pub atributos: usize,
        pub clusters: usize,
        pub instancias: usize,
    }


    impl ParametrosDataset {
        pub fn new (tipo: Datasets) -> ParametrosDataset {
            match tipo {
                Datasets::Bupa  => ParametrosDataset {
                    tipo,
                    archivo_datos:            PathBuf::from("./data/PAR/bupa_set.dat"),
                    archivo_restricciones_10: PathBuf::from("./data/PAR/bupa_set_const_10.const"),
                    archivo_restricciones_20: PathBuf::from("./data/PAR/bupa_set_const_20.const"),
                    atributos: 5,
                    clusters: 16,
                    instancias: 345,
                },

                Datasets::Glass => ParametrosDataset {
                    tipo,
                    archivo_datos:            PathBuf::from("./data/PAR/glass_set.dat"),
                    archivo_restricciones_10: PathBuf::from("./data/PAR/glass_set_const_10.const"),
                    archivo_restricciones_20: PathBuf::from("./data/PAR/glass_set_const_20.const"),
                    atributos: 9,
                    clusters: 7,
                    instancias: 214,
                },

                Datasets::Zoo   => ParametrosDataset {
                    tipo,
                    archivo_datos:            PathBuf::from("./data/PAR/zoo_set.dat"),
                    archivo_restricciones_10: PathBuf::from("./data/PAR/zoo_set_const_10.const"),
                    archivo_restricciones_20: PathBuf::from("./data/PAR/zoo_set_const_20.const"),
                    atributos: 16,
                    clusters: 7,
                    instancias: 101,
                }
            }
        }
    }

    // ────────────────────────────────────────────────────────────────────────────────

    #[derive(Debug, Copy, Clone)]
    #[allow(non_camel_case_types)]
    pub enum Algoritmos {
        Greedy,
        BL,
        AGG_UN,
        AGG_SF,
        AGE_UN,
        AGE_SF
    }

    #[allow(non_camel_case_types)]
    pub struct AlgoritmosAEjecutar {
        pub greedy: bool,
        pub BL: bool,

        pub agg_un: bool,
        pub agg_sf: bool,
        pub age_un: bool,
        pub age_sf: bool,

        pub benchmark: bool,
    }

    impl AlgoritmosAEjecutar {
        pub fn new() -> AlgoritmosAEjecutar {
            AlgoritmosAEjecutar {
                greedy: false,
                BL    : false,
                agg_un: false,
                agg_sf: false,
                age_un: false,
                age_sf: false,

                benchmark: false
            }
        }
    }

    // ────────────────────────────────────────────────────────────────────────────────

    #[allow(non_camel_case_types)]
    pub struct InfoEjecucion {
        pub tasa_inf: u32,
        pub desviacion_general: f64,
        pub agr: f64,
        pub tiempo: std::time::Duration,
    }

    impl InfoEjecucion {
        pub fn new() -> InfoEjecucion {
            InfoEjecucion {
                tasa_inf: 0,
                desviacion_general: 0.0,
                agr: 0.0,
                tiempo: time::Duration::new(0, 0),
            }
        }
    }

    // ────────────────────────────────────────────────────────────────────────────────

    pub struct RutasCSV {
        pub zoo_10: String,
        pub zoo_20: String,
        pub glass_10: String,
        pub glass_20: String,
        pub bupa_10: String,
        pub bupa_20: String
    }

    impl RutasCSV {
        pub fn new(alg: Algoritmos) -> RutasCSV {
            match alg {
                Algoritmos::Greedy => RutasCSV {
                    zoo_10  : String:: from("./data/csv/zoo_10/greedy.csv"),
                    zoo_20  : String:: from("./data/csv/zoo_20/greedy.csv"),
                    glass_10: String:: from("./data/csv/glass_10/greedy.csv"),
                    glass_20: String:: from("./data/csv/glass_20/greedy.csv"),
                    bupa_10 : String:: from("./data/csv/bupa_10/greedy.csv"),
                    bupa_20 : String:: from("./data/csv/bupa_20/greedy.csv"),
                },
                Algoritmos::BL => RutasCSV {
                    zoo_10  : String:: from("./data/csv/zoo_10/bl.csv"),
                    zoo_20  : String:: from("./data/csv/zoo_20/bl.csv"),
                    glass_10: String:: from("./data/csv/glass_10/bl.csv"),
                    glass_20: String:: from("./data/csv/glass_20/bl.csv"),
                    bupa_10 : String:: from("./data/csv/bupa_10/bl.csv"),
                    bupa_20 : String:: from("./data/csv/bupa_20/bl.csv"),
                },
                Algoritmos::AGG_UN => RutasCSV {
                    zoo_10  : String:: from("./data/csv/zoo_10/agg_un.csv"),
                    zoo_20  : String:: from("./data/csv/zoo_20/agg_un.csv"),
                    glass_10: String:: from("./data/csv/glass_10/agg_un.csv"),
                    glass_20: String:: from("./data/csv/glass_20/agg_un.csv"),
                    bupa_10 : String:: from("./data/csv/bupa_10/agg_un.csv"),
                    bupa_20 : String:: from("./data/csv/bupa_20/agg_un.csv"),
                },
                Algoritmos::AGG_SF => RutasCSV {
                    zoo_10  : String:: from("./data/csv/zoo_10/agg_sf.csv"),
                    zoo_20  : String:: from("./data/csv/zoo_20/agg_sf.csv"),
                    glass_10: String:: from("./data/csv/glass_10/agg_sf.csv"),
                    glass_20: String:: from("./data/csv/glass_20/agg_sf.csv"),
                    bupa_10 : String:: from("./data/csv/bupa_10/agg_sf.csv"),
                    bupa_20 : String:: from("./data/csv/bupa_20/agg_sf.csv"),
                },
                Algoritmos::AGE_UN => RutasCSV {
                    zoo_10  : String:: from("./data/csv/zoo_10/age_un.csv"),
                    zoo_20  : String:: from("./data/csv/zoo_20/age_un.csv"),
                    glass_10: String:: from("./data/csv/glass_10/age_un.csv"),
                    glass_20: String:: from("./data/csv/glass_20/age_un.csv"),
                    bupa_10 : String:: from("./data/csv/bupa_10/age_un.csv"),
                    bupa_20 : String:: from("./data/csv/bupa_20/age_un.csv"),
                },
                Algoritmos::AGE_SF => RutasCSV {
                    zoo_10  : String:: from("./data/csv/zoo_10/age_sf.csv"),
                    zoo_20  : String:: from("./data/csv/zoo_20/age_sf.csv"),
                    glass_10: String:: from("./data/csv/glass_10/age_sf.csv"),
                    glass_20: String:: from("./data/csv/glass_20/age_sf.csv"),
                    bupa_10 : String:: from("./data/csv/bupa_10/age_sf.csv"),
                    bupa_20 : String:: from("./data/csv/bupa_20/age_sf.csv"),
                },
            }
        }
    }

    // ────────────────────────────────────────────────────────────────────────────────


    pub struct Semillas {
        semillas: Vec<u64>
    }

    impl Semillas {
        pub fn new() -> Semillas {
            Semillas {
                semillas: vec![
                    328471273,
                    1821789317287,
                    128931083781,
                    1802783721873,
                    9584985309
                ]
            }
        }

        pub fn semilla(&self, i: usize) -> u64 {
            if i > self.semillas.len() {
                panic!("Se ha introducido un índice mayor que el número de semillas presente");
            }

            self.semillas[i]
        }
    }


    // ────────────────────────────────────────────────────────────────────────────────

    #[derive(Debug)]
    pub enum ModeloGenetico {
        Estacionario,
        Generacional
    }

//
// ──────────────────────────────────────────────────────────────── FUNCIONES ─────
//

    pub fn distancia(p1: &Punto, p2: &Punto) -> f64 {
        assert_eq!(p1.len(), p2.len());

        p1.metric_distance(p2)
    }