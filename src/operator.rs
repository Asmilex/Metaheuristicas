use rand::{Rng, rngs::StdRng};
use std::cmp::{min, max};


#[derive(Debug)]
pub enum Operadores {
    Uniforme,
    SegmentoFijo
}


pub fn cruce_uniforme(padre_1: &Vec<usize>, padre_2: &Vec<usize>, generador: &mut StdRng) -> Vec<usize> {
    let mut descendencia = vec![0; padre_1.len()];

    let mut genes_a_copiar = Vec::new();

    for _ in 0 .. padre_1.len()/2 {
        loop {
            let pos_gen = generador.gen_range(0..padre_1.len());

            if !genes_a_copiar.contains(&pos_gen) {
                genes_a_copiar.push(pos_gen);
                break;
            }
        }
    }

    // Copiar cromosomas del padre 1
    for i in genes_a_copiar.iter() {
        descendencia[*i] = padre_1[*i];
    }

    for i in 0 .. descendencia.len() {
        if descendencia[i] == 0 {           // Están marcados con 0 los que todavía no se han copiado
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

    let mut genes_a_copiar = Vec::new();

    for _ in 0 .. fin - inicio + 1 {
        loop {
            let pos_gen = generador.gen_range(inicio..=fin);

            if !genes_a_copiar.contains(&pos_gen) {
                genes_a_copiar.push(pos_gen);
                break;
            }
        }
    }


    for i in genes_a_copiar.iter() {
        descendencia[*i] = padre_1[*i];
    }

    for i in 0 .. descendencia.len() {
        if descendencia[i] == 0 {           // Están marcados con 0 los que todavía no se han copiado
            descendencia[i] = padre_2[i];
        }
    }

    descendencia
}


pub fn reparar(hijo: &mut Vec<usize>, k: usize, generador: &mut StdRng) {
    // Mover el primero
    let mut recuento= vec![0; k];

    for c in hijo.iter() {
        recuento[c - 1] = recuento[c - 1] + 1;
    }

    for indice_cluster_vacio in 0 .. recuento.len() {
        if recuento[indice_cluster_vacio] == 0 {
            loop {
                // Buscar elemento aleatorio y moverlo al clúster vacío.
                let i = generador.gen_range(0..hijo.len());

                if recuento[hijo[i]-1] > 1 {
                    recuento[hijo[i]-1] = recuento[hijo[i]-1] - 1;
                    hijo[i] = indice_cluster_vacio + 1;
                    recuento[indice_cluster_vacio] = recuento[indice_cluster_vacio] + 1;

                    break;
                }
            }
        }
    }
}