pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    if lista.is_empty() {
        return None;
    }

    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    // O intervalo de busca e reduzido pela metade a cada iteracao.
    while esquerda <= direita {
        let meio: isize = esquerda + (direita - esquerda) / 2;
        let indice: usize = meio as usize;

        if lista[indice] == alvo {
            return Some(indice);
        } else if lista[indice] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::busca_binaria;

    #[test]
    fn encontra_elemento_existente() {
        assert_eq!(busca_binaria(&[1, 3, 5, 7, 9], 7), Some(3));
    }

    #[test]
    fn retorna_none_quando_nao_encontra() {
        assert_eq!(busca_binaria(&[1, 3, 5, 7, 9], 4), None);
    }
}
