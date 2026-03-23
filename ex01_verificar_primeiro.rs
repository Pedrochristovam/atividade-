pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    // O algoritmo acessa somente a primeira posicao, sem percorrer a lista.
    if lista.is_empty() {
        None
    } else {
        Some(lista[0])
    }
}

#[cfg(test)]
mod tests {
    use super::verificar_primeiro;

    #[test]
    fn lista_vazia() {
        assert_eq!(verificar_primeiro(&[]), None);
    }

    #[test]
    fn retorna_primeiro_elemento() {
        assert_eq!(verificar_primeiro(&[10, 20, 30]), Some(10));
    }
}
