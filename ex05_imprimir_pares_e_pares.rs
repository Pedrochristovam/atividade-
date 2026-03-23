pub fn imprimir_pares_e_pares(lista: &[i32]) -> (Vec<i32>, Vec<(i32, i32)>) {
    let mut elementos_impressos: Vec<i32> = Vec::with_capacity(lista.len());
    let mut pares_impressos: Vec<(i32, i32)> = Vec::with_capacity(lista.len() * lista.len());

    // Primeiro imprime cada elemento sozinho.
    for &x in lista {
        println!("{}", x);
        elementos_impressos.push(x);
    }

    // Depois imprime todos os pares.
    for &x in lista {
        for &y in lista {
            println!("({}, {})", x, y);
            pares_impressos.push((x, y));
        }
    }

    (elementos_impressos, pares_impressos)
}

#[cfg(test)]
mod tests {
    use super::imprimir_pares_e_pares;

    #[test]
    fn gera_todos_os_pares_ordenados() {
        let (elementos, pares) = imprimir_pares_e_pares(&[1, 2]);
        assert_eq!(elementos, vec![1, 2]);
        assert_eq!(pares, vec![(1, 1), (1, 2), (2, 1), (2, 2)]);
    }
}
