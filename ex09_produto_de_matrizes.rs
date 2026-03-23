pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n: usize = a.len();

    if n == 0 {
        return Vec::new();
    }

    let mut c: Vec<Vec<i64>> = vec![vec![0i64; n]; n];

    // Faz a multiplicacao seguindo a formula normal de matrizes.
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::produto_de_matrizes;

    #[test]
    fn multiplica_matrizes_2x2() {
        let a: Vec<Vec<i64>> = vec![vec![1, 2], vec![3, 4]];
        let b: Vec<Vec<i64>> = vec![vec![5, 6], vec![7, 8]];
        let resultado: Vec<Vec<i64>> = produto_de_matrizes(&a, &b);

        assert_eq!(resultado, vec![vec![19, 22], vec![43, 50]]);
    }
}
