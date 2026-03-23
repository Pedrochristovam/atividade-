# Atividade Rust

Nessa atividade eu reescrevi em Rust os algoritmos que tinham sido passados em Python. Tambem deixei testes com `#[test]` e salvei as saidas dos comandos na pasta `prints_execucao`.

## Comandos usados

```bash
cargo build
cargo test
cargo run
```

## Exercício 1 — Verificar Primeiro
**Complexidade:** `O(1)`

**Lógica do algoritmo:**
O algoritmo olha se a lista esta vazia. Se estiver, retorna `None`. Se nao estiver, retorna o primeiro elemento.

**Justificativa da complexidade:**
Nao tem loop e nem recursao. O tamanho da entrada nao muda quase nada aqui porque so acontece uma verificacao e um acesso direto.

## Exercício 2 — Somar Lista
**Complexidade:** `O(n)`

**Lógica do algoritmo:**
Passa por todos os elementos da lista e vai somando em uma variavel chamada `total`.

**Justificativa da complexidade:**
Tem um loop so. Se a lista tiver `n` elementos, o loop roda `n` vezes, entao o tempo cresce junto com a entrada.

## Exercício 3 — Busca Binária
**Complexidade:** `O(log n)`

**Lógica do algoritmo:**
O algoritmo compara o alvo com o meio da lista e vai descartando metade da busca a cada passo ate encontrar ou acabar os elementos.

**Justificativa da complexidade:**
Tem um `while`, mas ele nao passa por tudo. Cada repeticao corta a busca pela metade, por isso o crescimento e logaritmico.

## Exercício 4 — Pares com Soma
**Complexidade:** `O(n²)`

**Lógica do algoritmo:**
Compara os pares possiveis da lista e imprime os que somam o valor alvo.

**Justificativa da complexidade:**
Tem dois loops, um dentro do outro. Isso faz o numero de comparacoes crescer bastante quando `n` aumenta, ficando quadratico.

## Exercício 5 — Imprimir Pares e Pares
**Complexidade:** `O(n²)`

**Lógica do algoritmo:**
Primeiro imprime os valores da lista. Depois imprime todos os pares possiveis entre os elementos.

**Justificativa da complexidade:**
O primeiro bloco e `O(n)` e o segundo tem dois loops aninhados, entao e `O(n²)`. No final, o que domina e `O(n²)`.

## Exercício 6 — Potências de Dois
**Complexidade:** `O(log n)`

**Lógica do algoritmo:**
Comeca em `1` e vai dobrando o valor enquanto ele for menor que `n`.

**Justificativa da complexidade:**
Tem um loop `while`, mas a variavel cresce em dobro. Por isso nao precisa de muitas repeticoes para chegar em `n`.

## Exercício 7 — Fibonacci Recursivo
**Complexidade:** `O(2^n)`

**Lógica do algoritmo:**
Se `n` for `0` ou `1`, retorna o proprio valor. Senao, chama a funcao para `n - 1` e `n - 2` e soma os resultados.

**Justificativa da complexidade:**
Nao tem loop, mas tem varias chamadas recursivas repetidas. Conforme `n` cresce, a quantidade de chamadas cresce muito rapido.

## Exercício 8 — Ordenação Bolha
**Complexidade:** `O(n²)`

**Lógica do algoritmo:**
Vai comparando elementos vizinhos e trocando de lugar quando estao fora de ordem, ate a lista ficar ordenada.

**Justificativa da complexidade:**
Tem dois loops aninhados. Mesmo sendo simples, ele faz muitas comparacoes no pior caso.

## Exercício 9 — Produto de Matrizes
**Complexidade:** `O(n³)`

**Lógica do algoritmo:**
Cria uma matriz nova e calcula cada posicao multiplicando linha por coluna.

**Justificativa da complexidade:**
Tem tres loops aninhados. Se a matriz for `n x n`, a quantidade de operacoes fica cubica.

## Exercício 10 — Merge Sort
**Complexidade:** `O(n log n)`

**Lógica do algoritmo:**
Divide a lista em partes menores, ordena cada parte e depois junta tudo em ordem.

**Justificativa da complexidade:**
A lista vai sendo dividida em varias partes e, depois, os elementos sao todos percorridos na hora de juntar. Isso gera `O(n log n)`.
