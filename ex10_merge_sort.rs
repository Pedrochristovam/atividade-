pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }

    let meio: usize = lista.len() / 2;
    let esquerda: Vec<i32> = merge_sort(lista[..meio].to_vec());
    let direita: Vec<i32> = merge_sort(lista[meio..].to_vec());

    merge(esquerda, direita)
}

fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    let mut resultado: Vec<i32> = Vec::with_capacity(esquerda.len() + direita.len());
    let mut i: usize = 0;
    let mut j: usize = 0;

    // A fusao compara os menores elementos restantes de cada metade ordenada.
    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);

    resultado
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn ordena_lista_com_merge_sort() {
        let ordenada: Vec<i32> = merge_sort(vec![38, 27, 43, 3, 9, 82, 10]);
        assert_eq!(ordenada, vec![3, 9, 10, 27, 38, 43, 82]);
    }
}
