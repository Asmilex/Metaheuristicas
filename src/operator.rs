pub enum Operadores {
    Uniforme,
    SegmentoFijo
}

pub fn cruce_uniforme(padre_1: &Vec<usize>, padre_2: &Vec<usize>, cromosomas_padre_1: Vec<usize>) -> Vec<usize> {
    let mut descendencia = vec![0; padre_1.len()];

    // Copiar cromosomas del padre 1
    for i in cromosomas_padre_1.iter() {
        descendencia[*i] = padre_1[*i];
    }

    for i in 0 .. descendencia.len() {
        if descendencia[i] != 0 {
            descendencia[i] = padre_2[i];
        }
    }

    descendencia
}


pub fn cruce_segmento_fijo(padre_1: &Vec<usize>, padre_2: &Vec<usize>, cromosomas_padre_1: Vec<usize>) -> Vec<usize> {
    let mut descendencia = vec![0; padre_1.len()];


}