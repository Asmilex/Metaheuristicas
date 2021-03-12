use nalgebra::*;

//
// ───────────────────────────────────────────────────── TIPOS PERSONALIZADOS ─────
//


    pub type Punto = DVector<f64>;
    pub type MatrizDinamica<Tipo> = MatrixMN<Tipo, Dynamic, Dynamic>;
    pub type Restriccion = (Punto, Punto);


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
        pub atributos: usize,
        pub clusters: usize,
        pub instancias: usize
    }

    impl PAR_parametros {
        pub fn new (tipo: PAR_nombres) -> PAR_parametros {
            match tipo {
                PAR_nombres::Bupa  => PAR_parametros {tipo, atributos: 5,  clusters: 16, instancias: 345},
                PAR_nombres::Glass => PAR_parametros {tipo, atributos: 9,  clusters: 7,  instancias: 214},
                PAR_nombres::Zoo   => PAR_parametros {tipo, atributos: 16, clusters: 7,  instancias: 101}
            }
        }
    }

//
// ──────────────────────────────────────────────────────────────── FUNCIONES ─────
//


    pub fn distancia(p1: &Punto, p2: &Punto) -> f64 {
        assert_eq!(p1.len(), p2.len());

        let mut sum_cuadrados: f64 = 0.0;

        for i in 0 .. p1.len() {
            sum_cuadrados += (p1[i] - p2[i]) * (p1[i] - p2[i]);
        }

        sum_cuadrados.sqrt()
    }
