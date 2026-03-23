# Atividade Rust

Nessa atividade eu peguei os algoritmos que estavam em Python e reescrevi tudo em Rust. A ideia foi manter a mesma logica, so adaptando pro jeito que Rust funciona, com tipos explicitos, `Option`, `Vec`, referencia e essas coisas.

Tambem deixei testes com `#[test]` e salvei as saidas do terminal na pasta `prints_execucao`, pra ficar como prova de que rodou certinho.

## Comandos que usei

```bash
cargo build
cargo test
cargo run
```

## Exercício 1 — Verificar Primeiro
**Complexidade:** `O(1)`

**Lógica do algoritmo:**
Esse aqui e bem direto. Ele so olha se a lista esta vazia. Se estiver, retorna `None`. Se nao estiver, devolve o primeiro valor da lista.

**Justificativa da complexidade:**
Nao tem loop, nao tem recursao, nao tem nada se repetindo. O algoritmo faz so uma verificacao e no maximo pega a posicao zero, entao o tempo praticamente nao muda com o tamanho da entrada.

## Exercício 2 — Somar Lista
**Complexidade:** `O(n)`

**Lógica do algoritmo:**
Aqui eu percorro a lista inteira e vou acumulando os valores em uma variavel `total` ate chegar na soma final.

**Justificativa da complexidade:**
Tem um loop simples. Se a lista tiver `n` elementos, ele vai passar pelos `n`, um por um. Por isso o tempo cresce de forma linear.

## Exercício 3 — Busca Binária
**Complexidade:** `O(log n)`

**Lógica do algoritmo:**
A busca binaria pega o meio da lista e compara com o valor procurado. Dependendo do resultado, ela joga fora metade da busca e continua so na parte que ainda faz sentido olhar.

**Justificativa da complexidade:**
Mesmo tendo um `while`, ele nao anda item por item. A cada repeticao o tamanho da busca cai pela metade, entao a quantidade de passos cresce bem devagar em relacao ao tamanho da lista.

## Exercício 4 — Pares com Soma
**Complexidade:** `O(n²)`

**Lógica do algoritmo:**
Esse algoritmo testa os pares possiveis da lista e imprime quando a soma de dois valores bate com o alvo pedido.

**Justificativa da complexidade:**
Aqui tem dois loops aninhados. Isso significa que, conforme a lista cresce, a quantidade de comparacoes cresce bem mais rapido, ficando quadratica.

## Exercício 5 — Imprimir Pares e Pares
**Complexidade:** `O(n²)`

**Lógica do algoritmo:**
Primeiro ele imprime cada elemento separado. Depois disso, faz todos os pares possiveis entre os elementos da lista.

**Justificativa da complexidade:**
O primeiro bloco sozinho seria `O(n)`, mas o segundo bloco tem dois loops, entao vira `O(n²)`. No fim, o que mais pesa e esse segundo bloco.

## Exercício 6 — Potências de Dois
**Complexidade:** `O(log n)`

**Lógica do algoritmo:**
Comeca em `1` e vai dobrando o valor: `1`, `2`, `4`, `8` e assim por diante, enquanto esse numero ainda for menor que `n`.

**Justificativa da complexidade:**
Mesmo sendo um loop, ele cresce muito rapido porque dobra a cada volta. Entao nao precisa repetir tantas vezes para chegar perto de `n`.

## Exercício 7 — Fibonacci Recursivo
**Complexidade:** `O(2^n)`

**Lógica do algoritmo:**
Se `n` for `0` ou `1`, ele retorna o proprio numero. Se nao for, chama a funcao de novo para `n - 1` e `n - 2` e soma os dois resultados.

**Justificativa da complexidade:**
Nao tem loop, mas tem muita chamada recursiva repetida. Conforme o valor de `n` aumenta, o numero de chamadas explode, por isso esse algoritmo fica bem pesado.

## Exercício 8 — Ordenação Bolha
**Complexidade:** `O(n²)`

**Lógica do algoritmo:**
Vai comparando elementos vizinhos e trocando eles de lugar quando estao fora de ordem. Faz isso varias vezes ate a lista ficar organizada.

**Justificativa da complexidade:**
Tem dois loops um dentro do outro. Entao, no pior caso, ele acaba fazendo muitas comparacoes e trocas, deixando o tempo quadratico.

## Exercício 9 — Produto de Matrizes
**Complexidade:** `O(n³)`

**Lógica do algoritmo:**
Cria uma matriz resultado e calcula cada posicao dela usando a multiplicacao da linha de uma matriz pela coluna da outra.

**Justificativa da complexidade:**
Aqui aparecem tres loops aninhados. Se a matriz for `n x n`, a quantidade de operacoes cresce em `n³`, entao fica cubico.

## Exercício 10 — Merge Sort
**Complexidade:** `O(n log n)`

**Lógica do algoritmo:**
O merge sort vai dividindo a lista em partes menores, ordena essas partes e depois junta tudo de volta na ordem certa.

**Justificativa da complexidade:**
Ele divide varias vezes a lista, mas na hora de juntar precisa percorrer os elementos. Essa combinacao faz a complexidade ficar em `O(n log n)`, que e bem melhor que varios algoritmos quadrados.
