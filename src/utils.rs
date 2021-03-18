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
    pub enum PAR_nombres {
        Bupa,
        Glass,
        Zoo
    }

    #[allow(non_camel_case_types)]
    pub enum PAR_restr {
        Diez,
        Veinte
    }


    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub struct PAR_parametros {
        pub tipo: PAR_nombres,

        pub archivo_datos: PathBuf,
        pub archivo_restricciones_10: PathBuf,
        pub archivo_restricciones_20: PathBuf,

        pub atributos: usize,
        pub clusters: usize,
        pub instancias: usize
    }


    impl PAR_parametros {
        pub fn new (tipo: PAR_nombres) -> PAR_parametros {
            match tipo {
                PAR_nombres::Bupa  => PAR_parametros {
                    tipo,
                    archivo_datos:            PathBuf::from("./data/PAR/bupa_set.dat"),
                    archivo_restricciones_10: PathBuf::from("./data/PAR/bupa_set_const_10.const"),
                    archivo_restricciones_20: PathBuf::from("./data/PAR/bupa_set_const_20.const"),
                    atributos: 5,
                    clusters: 16,
                    instancias: 345
                },

                PAR_nombres::Glass => PAR_parametros {
                    tipo,
                    archivo_datos:            PathBuf::from("./data/PAR/glass_set.dat"),
                    archivo_restricciones_10: PathBuf::from("./data/PAR/glass_set_const_10.const"),
                    archivo_restricciones_20: PathBuf::from("./data/PAR/glass_set_const_20.const"),
                    atributos: 9,
                    clusters: 7,
                    instancias: 214
                },

                PAR_nombres::Zoo   => PAR_parametros {
                    tipo,
                    archivo_datos:            PathBuf::from("./data/PAR/zoo_set.dat"),
                    archivo_restricciones_10: PathBuf::from("./data/PAR/zoo_set_const_10.const"),
                    archivo_restricciones_20: PathBuf::from("./data/PAR/zoo_set_const_20.const"),
                    atributos: 16,
                    clusters: 7,
                    instancias: 101
                }
            }
        }
    }

    // ────────────────────────────────────────────────────────────────────────────────

    #[allow(non_camel_case_types)]
    pub struct Algoritmos {
        pub greedy: bool,
        pub greedy_10_csv_path: String,
        pub greedy_20_csv_path: String,

        pub BL: bool,
        pub BL_10_csv_path: String,
        pub BL_20_csv_path: String,

        pub benchmark: bool,
    }

    impl Algoritmos {
        pub fn new() -> Algoritmos {
            Algoritmos {
                greedy: false,
                greedy_10_csv_path: String::from("../data/csv/greedy_10.csv"),
                greedy_20_csv_path: String::from("../data/csv/greedy_20.csv"),

                BL: false,
                BL_10_csv_path: String::from("../data/csv/bl_10.csv"),
                BL_20_csv_path: String::from("../data/csv/bl_20.csv"),

                benchmark: false}
        }
    }

    // ────────────────────────────────────────────────────────────────────────────────

    #[allow(non_camel_case_types)]
    pub struct InfoExecution {
        pub tasa_inf_zoo: u32,
        pub error_dist_zoo: f64,
        pub agr_zoo: f64,
        pub tiempo_zoo: std::time::Duration,

        pub tasa_inf_glass: u32,
        pub error_dist_glass: f64,
        pub agr_glass: f64,
        pub tiempo_glass: std::time::Duration,

        pub tasa_inf_bupa: u32,
        pub error_dist_bupa: f64,
        pub agr_bupa: f64,
        pub tiempo_bupa: std::time::Duration
    }

    impl InfoExecution {
        pub fn new() -> InfoExecution {
            InfoExecution {
                tasa_inf_zoo: 0,
                error_dist_zoo: 0.0,
                agr_zoo: 0.0,
                tiempo_zoo: time::Duration::new(0, 0),

                tasa_inf_glass: 0,
                error_dist_glass: 0.0,
                agr_glass: 0.0,
                tiempo_glass: time::Duration::new(0, 0),

                tasa_inf_bupa: 0,
                error_dist_bupa: 0.0,
                agr_bupa: 0.0,
                tiempo_bupa: time::Duration::new(0, 0),
            }
        }
    }

//
// ──────────────────────────────────────────────────────────────── FUNCIONES ─────
//

    pub fn distancia(p1: &Punto, p2: &Punto) -> f64 {
        assert_eq!(p1.len(), p2.len());

        p1.metric_distance(p2)
    }
