pub fn potencias_de_dois(n: u64) -> Vec<u64> {
    let mut valores: Vec<u64> = Vec::new();
    let mut i: u64 = 1;

    // Vai dobrando o valor ate chegar perto de n.
    while i < n {
        println!("{}", i);
        valores.push(i);
        i *= 2;
    }

    valores
}

#[cfg(test)]
mod tests {
    use super::potencias_de_dois;

    #[test]
    fn gera_potencias_ate_limite() {
        assert_eq!(potencias_de_dois(20), vec![1, 2, 4, 8, 16]);
    }
}
