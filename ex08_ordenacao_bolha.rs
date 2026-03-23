pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n: usize = lista.len();

    if n < 2 {
        return;
    }

    // A cada passada, o maior elemento restante "borbulha" para o fim.
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ordenacao_bolha;

    #[test]
    fn ordena_lista_desordenada() {
        let mut valores: Vec<i32> = vec![5, 1, 4, 2, 8];
        ordenacao_bolha(&mut valores);
        assert_eq!(valores, vec![1, 2, 4, 5, 8]);
    }
}
