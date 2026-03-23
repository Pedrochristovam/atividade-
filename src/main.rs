use atividade_rust::ex01_verificar_primeiro::verificar_primeiro;
use atividade_rust::ex02_somar_lista::somar_lista;
use atividade_rust::ex03_busca_binaria::busca_binaria;
use atividade_rust::ex04_pares_com_soma::pares_com_soma;
use atividade_rust::ex05_imprimir_pares_e_pares::imprimir_pares_e_pares;
use atividade_rust::ex06_potencias_de_dois::potencias_de_dois;
use atividade_rust::ex07_fibonacci_recursivo::fibonacci_recursivo;
use atividade_rust::ex08_ordenacao_bolha::ordenacao_bolha;
use atividade_rust::ex09_produto_de_matrizes::produto_de_matrizes;
use atividade_rust::ex10_merge_sort::merge_sort;

fn main() {
    println!("Exercicio 1 - Verificar Primeiro");
    let lista_ex1: Vec<i32> = vec![10, 20, 30];
    println!("Primeiro elemento: {:?}\n", verificar_primeiro(&lista_ex1));

    println!("Exercicio 2 - Somar Lista");
    let lista_ex2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Soma total: {}\n", somar_lista(&lista_ex2));

    println!("Exercicio 3 - Busca Binaria");
    let lista_ex3: Vec<i32> = vec![1, 3, 5, 7, 9, 11];
    println!("Indice do 7: {:?}\n", busca_binaria(&lista_ex3, 7));

    println!("Exercicio 4 - Pares com Soma");
    let lista_ex4: Vec<i32> = vec![1, 2, 3, 4, 5];
    let pares: Vec<(i32, i32)> = pares_com_soma(&lista_ex4, 6);
    println!("Pares encontrados: {:?}\n", pares);

    println!("Exercicio 5 - Imprimir Pares e Pares");
    let lista_ex5: Vec<i32> = vec![1, 2, 3];
    let (elementos, pares_ordenados): (Vec<i32>, Vec<(i32, i32)>) =
        imprimir_pares_e_pares(&lista_ex5);
    println!("Elementos impressos: {:?}", elementos);
    println!("Quantidade de pares gerados: {}\n", pares_ordenados.len());

    println!("Exercicio 6 - Potencias de Dois");
    let potencias: Vec<u64> = potencias_de_dois(20);
    println!("Potencias geradas: {:?}\n", potencias);

    println!("Exercicio 7 - Fibonacci Recursivo");
    println!("Fibonacci(10): {}\n", fibonacci_recursivo(10));

    println!("Exercicio 8 - Ordenacao Bolha");
    let mut lista_ex8: Vec<i32> = vec![5, 1, 4, 2, 8];
    ordenacao_bolha(&mut lista_ex8);
    println!("Lista ordenada: {:?}\n", lista_ex8);

    println!("Exercicio 9 - Produto de Matrizes");
    let matriz_a: Vec<Vec<i64>> = vec![vec![1, 2], vec![3, 4]];
    let matriz_b: Vec<Vec<i64>> = vec![vec![5, 6], vec![7, 8]];
    let matriz_c: Vec<Vec<i64>> = produto_de_matrizes(&matriz_a, &matriz_b);
    println!("Resultado: {:?}\n", matriz_c);

    println!("Exercicio 10 - Merge Sort");
    let lista_ex10: Vec<i32> = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Lista ordenada: {:?}", merge_sort(lista_ex10));
}
