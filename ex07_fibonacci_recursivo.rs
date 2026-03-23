pub fn fibonacci_recursivo(n: u64) -> u64 {
    // Os casos base encerram a recursao.
    if n <= 1 {
        return n;
    }

    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

#[cfg(test)]
mod tests {
    use super::fibonacci_recursivo;

    #[test]
    fn calcula_fibonacci_de_dez() {
        assert_eq!(fibonacci_recursivo(10), 55);
    }
}
