use rand::{Rng, rngs::StdRng, distributions::Uniform};

#[derive(Debug)]
pub enum Operadores {
    Uniforme,
    SegmentoFijo
}


pub fn cruce_uniforme(padre_1: &Vec<usize>, padre_2: &Vec<usize>, generador: &mut StdRng) -> Vec<usize> {
    let rango = Uniform::new(0, padre_1.len());

    let mut descendencia = vec![0; padre_1.len()];
    let mut genes_a_copiar = Vec::new();

    for _ in 0 .. padre_1.len()/2 {
        loop {
            let pos_gen = generador.sample(rango);

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

    let rango = Uniform::new(0, padre_1.len());

    let inicio_segmento = generador.sample(rango);            // Inicio del segmento
    let tamano_segmento = generador.sample(rango);

    let mut i = inicio_segmento;
    let mut copias: usize = 0;

    while copias < tamano_segmento {
        descendencia[i] = padre_1[i];

        i = (i+1)%padre_1.len();
        copias = copias + 1;
    }

    let copias_restantes = descendencia.len() - tamano_segmento;

    let mut genes_a_copiar = Vec::new();

    for _ in 0 .. copias_restantes/2 {
        loop {
            let pos_gen = generador.sample(rango);

            if !genes_a_copiar.contains(&pos_gen) && descendencia[pos_gen] == 0 {
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

    let rango = Uniform::new(0, hijo.len());
    for indice_cluster_vacio in 0 .. recuento.len() {
        if recuento[indice_cluster_vacio] == 0 {
            loop {
                // Buscar elemento aleatorio y moverlo al clúster vacío.
                let i = generador.sample(rango);

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