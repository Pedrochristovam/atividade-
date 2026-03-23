pub fn pares_com_soma(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n: usize = lista.len();
    let mut pares_encontrados: Vec<(i32, i32)> = Vec::new();

    // Testa os pares da lista.
    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("{} + {} = {}", lista[i], lista[j], alvo);
                pares_encontrados.push((lista[i], lista[j]));
            }
        }
    }

    pares_encontrados
}

#[cfg(test)]
mod tests {
    use super::pares_com_soma;

    #[test]
    fn encontra_pares_que_somam_alvo() {
        let pares: Vec<(i32, i32)> = pares_com_soma(&[1, 2, 3, 4, 5], 6);
        assert_eq!(pares, vec![(1, 5), (2, 4)]);
    }
}
