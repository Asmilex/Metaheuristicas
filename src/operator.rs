use rand::{Rng, rngs::StdRng};
use std::cmp::{min, max};


#[derive(Debug)]
pub enum Operadores {
    Uniforme,
    SegmentoFijo
}

pub fn cruce_uniforme(padre_1: &Vec<usize>, padre_2: &Vec<usize>, generador: &mut StdRng) -> Vec<usize> {
    let mut descendencia = vec![0; padre_1.len()];

    let cromosomas_a_copiar: Vec<usize> = vec![generador.gen_range(0..padre_1.len()); padre_1.len()/2];
    /* FIXME mirar si lo de arriba funciona. Si no, usar esto de abajo.
    for _ in 0 .. cluster.num_elementos/2 {
        cromosomas.push(generador.gen_range(0..cluster.num_elementos));
    }
    */

    // Copiar cromosomas del padre 1
    for i in cromosomas_a_copiar.iter() {
        descendencia[*i] = padre_1[*i];
    }

    for i in 0 .. descendencia.len() {
        if descendencia[i] != 0 {
            descendencia[i] = padre_2[i];
        }
    }

    descendencia
}


pub fn cruce_segmento_fijo(padre_1: &Vec<usize>, padre_2: &Vec<usize>, generador: &mut StdRng) -> Vec<usize> {
    let mut descendencia = vec![0; padre_1.len()];

    let inicio_segmento = generador.gen_range(0 .. padre_1.len());            // Inicio del segmento
    let tamano_segmento = generador.gen_range(0.. padre_1.len());

    let mut i = inicio_segmento;
    let mut copias: usize = 0;

    while copias < tamano_segmento {
        descendencia[i] = padre_1[i];

        i = (i+1)%padre_1.len();
        copias = copias + 1;
    }

    // Mezclar el vector resultante que se queda entre medias de forma aleatoria como hacemos en el cruce uniforme
    let inicio = min((inicio_segmento + 1)%padre_1.len(), (inicio_segmento + tamano_segmento + 1)%padre_1.len());
    let fin = max((inicio_segmento + 1)%padre_1.len(), (inicio_segmento + tamano_segmento + 1)%padre_1.len());

    let cromosomas_a_copiar = vec![generador.gen_range(inicio..=fin); fin - inicio + 1];

    for i in cromosomas_a_copiar.iter() {
        descendencia[*i] = padre_1[*i];
    }

    for i in inicio ..= fin {
        if descendencia[i] != 0 {
            descendencia[i] = padre_2[i];
        }
    }

    descendencia
}