pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total: i32 = 0;

    // Cada elemento e visitado exatamente uma vez.
    for &elemento in lista {
        total += elemento;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::somar_lista;

    #[test]
    fn soma_elementos_da_lista() {
        assert_eq!(somar_lista(&[1, 2, 3, 4]), 10);
    }
}
