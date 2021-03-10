use nalgebra::*;

pub type Punto = DVector<f64>;
pub type MatrizDinamica<Tipo> = MatrixMN<Tipo, Dynamic, Dynamic>;
pub type Restriccion = (Punto, Punto);

pub fn distancia(p1: &Punto, p2: &Punto) -> f64 {
    assert_eq!(p1.len(), p2.len());

    let mut sum_cuadrados: f64 = 0.0;

    for i in 0..p1.len() {
        sum_cuadrados += (p1[i] - p2[i]) * (p1[i] - p2[i]);
    }

    sum_cuadrados.sqrt()
}