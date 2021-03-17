use nalgebra::*;
use std::path::{PathBuf};

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
        pub BL: bool,
    }

    impl Algoritmos {
        pub fn new() -> Algoritmos {
            Algoritmos {greedy: false, BL: false}
        }
    }

//
// ──────────────────────────────────────────────────────────────── FUNCIONES ─────
//

    pub fn distancia(p1: &Punto, p2: &Punto) -> f64 {
        assert_eq!(p1.len(), p2.len());

        p1.metric_distance(p2)
    }
