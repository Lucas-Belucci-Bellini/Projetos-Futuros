# Revisão — Álgebra Linear

**Disciplina:** Álgebra Linear — UNIFIL, Departamento de Computação
**Professor:** Me. Guilherme C. Agostinetti
**Material de origem:** quatro conjuntos de slides, na ordem em que foram dados:

| # | Slides | Data | Lâminas |
| --- | --- | --- | --- |
| 1 | Matrizes I | 14/07/2026 | 46 |
| 2 | Matrizes II | 14/07/2026 | 32 |
| 3 | Determinantes | 19/08/2026 | 48 |
| 4 | Sistemas Lineares | 02/09/2026 | 33 |

**O que este documento é:** a matéria dos quatro conjuntos reorganizada em ordem de estudo,
com **todos os exercícios propostos resolvidos e conferidos por cálculo independente**, a
errata dos slides, e um simulado com gabarito no fim.

> **Aviso sobre três respostas dos slides.** Recalculei todo exercício numérico destes
> slides por dois caminhos distintos (eliminação gaussiana em frações exatas e expansão por
> cofatores). **Três respostas impressas nos slides não fecham** — estão listadas e
> demonstradas na seção 13. Não são erros seus se você chegou a outro número.

---

## Índice

| Seção | Assunto |
| --- | --- |
| 0 | O fio que liga tudo: por que estes quatro assuntos são um só |
| 1–5 | **Matrizes I** — definição, tipos, igualdade, transposta, adição, escalar |
| 6–8 | **Matrizes II** — produto, inversa, matrizes especiais |
| 9–11 | **Determinantes** — ordem 2 e 3, propriedades, Laplace, inversa por cofator |
| 12 | **Sistemas Lineares** — classificação e escalonamento |
| 13 | Errata conferida dos slides |
| 14 | Exercícios resolvidos (todos os dos slides) |
| 15 | Simulado + gabarito |

---

## 0. O fio que liga tudo

Os quatro conjuntos de slides parecem quatro assuntos. São **um só**, e perceber isso vale
mais do que decorar qualquer fórmula. A cadeia é esta:

```
  Matriz A (n x n)
        |
        v
   det(A) = 0 ?
        |
   +----+-----------------------------+
   |                                  |
  NÃO                                SIM
   |                                  |
A admite inversa A^-1            A não admite inversa
   |                                  |
Sistema A·X = B tem            Sistema A·X = B é
UMA única solução              SPI ou SI (nunca SPD)
   = SPD                        (nunca solução única)
        |
   X = A^-1 · B
```

**Traduzindo:** o determinante é um **único número** que diz, de uma vez, se a matriz é
inversível **e** se o sistema associado tem solução única. É por isso que a ordem dos slides
é essa: primeiro você aprende a manipular matrizes, depois a multiplicá-las e invertê-las,
depois ganha o teste (`det ≠ 0`) que decide se a inversa existe, e só então aplica tudo a
sistemas.

As **operações elementares** também aparecem nos três últimos conjuntos, sempre fazendo a
mesma coisa:

| onde aparece | para quê |
| --- | --- |
| Matrizes II | achar `A⁻¹` — escreve-se `A` ao lado de `I` e transforma-se `A` em `I` |
| Determinantes | triangular a matriz e ler a diagonal |
| Sistemas Lineares | escalonar e resolver por substituição |

**Uma técnica, três usos.** Se você dominar as operações elementares, dominou metade da
disciplina.

---

# PARTE I — MATRIZES

## 1. Definição e notação

Uma tabela de dados, **desconsiderado o significado** de linhas e colunas, é uma matriz:

| Aluno | 1º Bim | 2º Bim | 3º Bim | 4º Bim |
| --- | --- | --- | --- | --- |
| Aluno 1 | 7.5 | 8.0 | 9.0 | 6.5 |
| Aluno 2 | 6.0 | 7.5 | 8.5 | 7.0 |
| Aluno 3 | 9.0 | 8.5 | 7.0 | 8.0 |

**Definição.** Sejam `m, n` inteiros positivos. Uma matriz `m` por `n` sobre `K` é dada por
`m × n` valores `aᵢⱼ`, com `1 ≤ i ≤ m` e `1 ≤ j ≤ n`, agrupados em `m` linhas e `n` colunas:

```
A = [aij]mxn =  | a11  a12  ...  a1n |
                | a21  a22  ...  a2n |
                |  .    .    .    .  |
                | am1  am2  ...  amn |
```

**Pontos que caem em prova:**

- A **ordem** é sempre **linhas × colunas**, nessa ordem. `A` de ordem `2×3` tem 2 linhas.
- O índice é sempre `aᵢⱼ` = **linha i, coluna j**. Trocar a ordem dos índices é o erro nº 1.
- Os elementos **não precisam ser números**: podem ser polinômios, funções, números
  complexos.
- Notações aceitas: parênteses `( )`, colchetes `[ ]` ou duas barras `‖ ‖`.

### 1.1 Lei de formação

Em vez de listar os elementos, dá-se uma regra. Exemplo: `A = (aᵢⱼ)₂ₓ₃` com `aᵢⱼ = i² − j²`
significa "para cada posição, calcule `i² − j²`". Resolvido no E2 da seção 14.

## 2. Os onze tipos de matriz

| Tipo | Condição | Exemplo |
| --- | --- | --- |
| **Nula** | `aᵢⱼ = 0` para todo `i, j` | `[[0,0],[0,0]]` |
| **Quadrada** | `m = n` | `[[1,2],[3,4]]` |
| **Linha** | uma única linha | `[1 2 3]` |
| **Coluna** | uma única coluna | `[1; 2; 3]` |
| **Diagonal** | quadrada com `aᵢⱼ = 0` para `i ≠ j` | `[[1,0,0],[0,5,0],[0,0,9]]` |
| **Identidade `Iₙ`** | diagonal com `aᵢᵢ = 1` | `[[1,0],[0,1]]` |
| **Triangular superior** | `aᵢⱼ = 0` para `i > j` (zeros **abaixo**) | `[[1,2,3],[0,4,5],[0,0,6]]` |
| **Triangular inferior** | `aᵢⱼ = 0` para `i < j` (zeros **acima**) | `[[1,0,0],[2,4,0],[3,5,6]]` |
| **Simétrica** | `A = Aᵀ`, isto é `aᵢⱼ = aⱼᵢ` | `[[1,3,-10],[3,0,5],[-10,5,2]]` |
| **Antissimétrica** | `Aᵀ = −A`, isto é `aᵢⱼ = −aⱼᵢ` | `[[0,2,-3],[-2,0,4],[3,-4,0]]` |
| **Ortogonal** | `A⁻¹ = Aᵀ`, isto é `A·Aᵀ = I` | ver §8.1 |

**Macete para não trocar triangular superior com inferior:** o nome descreve **onde estão os
números**, não onde estão os zeros. Triangular **superior** = os números ocupam a parte de
cima.

**Detalhe da antissimétrica que cai em prova:** como `aᵢᵢ = −aᵢᵢ`, a diagonal principal é
**sempre toda zero**. Não é uma escolha do exemplo — é consequência da definição.

## 3. Igualdade de matrizes

`A = B` **se e somente se** as duas condições valerem juntas:

1. **Mesma ordem** (`m = v` e `n = u`)
2. **Todos os elementos correspondentes iguais**

Os slides dão um exemplo bonito de que a igualdade é de **valor**, não de aparência:

```
A = | √25    sen30° |        B = |  5     1/2 |
    | -3/4      1   |            | -0,75   π⁰ |
```

São iguais, porque `√25 = 5`, `sen30° = 1/2`, `−3/4 = −0,75` e `π⁰ = 1`.

A aplicação prática é **montar sistemas**: igualar duas matrizes com incógnitas gera uma
equação por posição. (E3 na seção 14.)

## 4. Transposição

**Definição.** `B = Aᵀ` quando `bⱼᵢ = aᵢⱼ` — ou seja, **as linhas de B são as colunas de A**.

> ⚠️ O slide 24 de Matrizes I escreve "se as **linhas** de B forem iguais às **linhas** de
> A". Está errado — seria a matriz original. O correto é **linhas de B = colunas de A**.
> (Item 13.4 da errata.)

Se `A` é `m × n`, então `Aᵀ` é `n × m`. **A ordem inverte.**

### 4.1 As cinco propriedades da transposta

| Propriedade | Fórmula |
| --- | --- |
| Transposta da transposta | `(Aᵀ)ᵀ = A` |
| Soma | `(A + B)ᵀ = Aᵀ + Bᵀ` |
| Produto por escalar | `(αA)ᵀ = αAᵀ` |
| Identidade | `Iᵀ = I` |
| Diagonal | se `D` é diagonal, `Dᵀ = D` |

E, de Matrizes II, a que mais cai: **transposta do produto — `(AB)ᵀ = Bᵀ Aᵀ`**, em que a
ordem dos fatores **inverte**.

> **A inversão de ordem em `(AB)ᵀ = BᵀAᵀ` é a pegadinha mais cobrada da matéria.** Não é
> `AᵀBᵀ`. A mesma inversão acontece na inversa: `(AB)⁻¹ = B⁻¹A⁻¹`.

## 5. Adição e multiplicação por escalar

**Adição** (só entre matrizes de **mesma ordem**): `A + B = [aᵢⱼ + bᵢⱼ]` — soma elemento a
elemento.

**Escalar**: `αA = [α · aᵢⱼ]` — multiplica **todos** os elementos.

### 5.1 As oito propriedades

| # | Propriedade | Nome |
| --- | --- | --- |
| I | `A + (B + C) = (A + B) + C` | associatividade |
| II | `A + B = B + A` | comutatividade |
| III | `A + 0ₘₓₙ = A` | elemento nulo |
| IV | `A + (−A) = 0ₘₓₙ` | inverso aditivo |
| V | `µ(A + B) = µA + µB` | distributividade do escalar |
| VI | `0·A = 0ₘₓₙ` | multiplicação por zero |
| VII | `(µκ)A = µ(κA)` | associatividade escalar |
| VIII | `1·A = A` | elemento neutro |

Note que a **soma é comutativa** (II). Guarde isso, porque o **produto de matrizes não é** —
e essa é a diferença mais importante entre as duas operações.

---

## 6. Produto de matrizes

### 6.1 De onde vem

O produto não é "multiplicar elemento a elemento". Ele nasce de uma operação concreta:
**média ponderada**. Com as notas em `N` e os pesos em `P`:

```
N (3x4)                          P (4x1)              N·P (3x1)
| 7.5  8.0  6.5  9.0 |          | 1/10 |             | 7.875 |
| 6.0  7.0  8.5  7.5 |    x     | 2/10 |     =       | 7.375 |
| 9.0  9.5  8.0  8.5 |          | 3/10 |             | 8.375 |
                                 | 4/10 |
```

Cada elemento do resultado é **uma linha vezes uma coluna**, somando os produtos. É
literalmente o cálculo da média ponderada de cada aluno.

### 6.2 A definição

Sejam `A = [aᵢⱼ]ₘₓₙ` e `B = [bᵢⱼ]ₙₓₛ`. O produto `A·B = [cᵢⱼ]ₘₓₛ` tem

```
          n
  c_ij =  Σ  a_ik · b_kj
         k=1
```

### 6.3 A regra de compatibilidade (a mais importante)

```
      A (m x n)  ·  B (n x s)  =  C (m x s)
             ^^^      ^^^
             estes DOIS têm que ser iguais

             ^^^              ^^^
             estes DOIS formam a ordem do resultado
```

**Só existe produto se o número de colunas de A for igual ao número de linhas de B.** O
resultado tem as linhas de A e as colunas de B.

Consequência: **`AB` pode existir e `BA` não existir.** E mesmo quando os dois existem,
**`AB ≠ BA` em geral.** O produto de matrizes **não é comutativo** — é a diferença
fundamental em relação a tudo o que você viu antes.

### 6.4 As seis propriedades do produto

| # | Propriedade | Nome |
| --- | --- | --- |
| I | `(AB)C = A(BC)` | associatividade |
| II | `A(B + C) = AB + AC` | distributiva à direita |
| III | `(A + B)C = AC + BC` | distributiva à esquerda |
| IV | `κ(AB) = (κA)B` | compatibilidade com escalar |
| V | `A·0 = 0` | matriz nula |
| VI | **`(AB)ᵀ = Bᵀ Aᵀ`** | transposta do produto — **ordem inverte** |

**Não está na lista, e é de propósito:** comutatividade. `AB ≠ BA`.

## 7. Matriz inversa

**Definição.** `A` (quadrada, `n × n`) **admite inversa** se existe `B` tal que

```
A · B  =  B · A  =  Iₙ
```

Nesse caso `B = A⁻¹`. Repare que a definição exige **os dois produtos** — é por isso que só
matriz quadrada pode ter inversa.

### 7.1 As três operações elementares

Chamando **fila** o que pode ser linha ou coluna:

1. **Permutar** duas filas.
2. **Multiplicar** uma fila por um número real **não nulo**.
3. **Somar** a uma fila outra fila multiplicada por um real não nulo.

Duas matrizes são **equivalentes** (`A ~ B`) se uma sucessão finita de operações elementares
leva de `A` a `B`.

### 7.2 O teorema e o método

> **Teorema.** `A` admite inversa **se e somente se** `A ~ Iₙ`. E: *a mesma sucessão de
> operações elementares que transforma `A` em `Iₙ` transforma `Iₙ` na inversa de `A`.*

**Método prático:**

1. Escreva `A` e, ao lado, `Iₙ` da mesma ordem, separadas por uma linha tracejada:
   `[ A | I ]`.
2. Por operações elementares, transforme `A` em `Iₙ`, **aplicando simultaneamente as mesmas
   operações** à matriz da direita.
3. Quando a esquerda virar `I`, a direita é `A⁻¹`:  `[ I | A⁻¹ ]`.

Se em algum momento surgir uma **linha inteira de zeros** à esquerda, `A` não é equivalente
a `I` e **não tem inversa**.

### 7.3 As três propriedades da inversa

| Propriedade | Fórmula |
| --- | --- |
| Inversa da inversa | `(A⁻¹)⁻¹ = A` |
| Produto | **`(A·B)⁻¹ = B⁻¹ · A⁻¹`** — ordem **inverte** |
| Transposta | `(Aᵀ)⁻¹ = (A⁻¹)ᵀ` |

## 8. Mais tipos de matrizes

### 8.1 Ortogonal

`M` é ortogonal quando **`M⁻¹ = Mᵀ`**, ou seja `M·Mᵀ = Mᵀ·M = I`.

**O teste rápido, que os slides não dão mas economiza muito tempo:** `M` é ortogonal se e
somente se suas **linhas formam um conjunto ortonormal** — cada linha tem norma 1, e o
produto escalar entre duas linhas distintas é 0. (O mesmo vale para as colunas.)

### 8.2 Potência de matriz

`Aⁿ` = `A` multiplicada por si mesma `n` vezes. **Só faz sentido para matriz quadrada.**

### 8.3 Periódica, idempotente, nihilpotente

| Tipo | Condição | Período / índice |
| --- | --- | --- |
| **Periódica** | `Aⁿ = A` com `n ≥ 2` | período = `n − 1`, com `n` o **menor** que satisfaz |
| **Idempotente** | `A² = A` | caso particular da periódica: período `2 − 1 = 1` |
| **Nihilpotente** | `Aᵖ = 0` para algum inteiro positivo `p` | índice = **menor** `p` que zera |

**Consequência prática da idempotência:** se `A² = A`, então `A³ = A⁴ = ... = Aⁿ = A`. Uma
matriz idempotente elevada a **qualquer** potência continua sendo ela mesma.

**A nihilpotente é o contraexemplo mais importante da álgebra linear:** ela mostra que
`A² = 0` **não** implica `A = 0`. Nos números reais, `x² = 0 => x = 0`. Em matrizes, não.
(Ver E22 na seção 14: uma matriz 3×3 sem um único zero cujo quadrado é a matriz nula.)

---

# PARTE II — DETERMINANTES

## 9. Definição e cálculo

**História:** desenvolvido **simultaneamente** na Alemanha e no Japão — Leibniz (1646–1716)
e Seki Shinsuke Kowa (1642–1708), ambos resolvendo problemas de eliminação necessários à
solução de sistemas lineares (BOYER, 1988).

**Definição (SOARES, 1979).** *"Determinante é a somatória de todos os produtos possíveis
dos n elementos de uma matriz quadrada, de maneira que em cada parcela — formada por um
produto — não haja dois elementos pertencentes a uma mesma linha e/ou coluna."*

> **Determinante só existe para matriz QUADRADA.** Não existe determinante de uma matriz
> `2 × 3`.

### 9.1 Ordem 2

```
        | a11  a12 |
det A = |          | = a11·a22 - a12·a21
        | a21  a22 |
```

Produto da diagonal principal menos produto da diagonal secundária.

### 9.2 Ordem 3 — Regra de Sarrus

1. **Repita as duas primeiras colunas** à direita da matriz.
2. Some os produtos das três diagonais no sentido da **principal** (↘).
3. Some os produtos das três diagonais no sentido da **secundária** (↙).
4. `det A` = (passo 2) − (passo 3).

```
det A = (a11·a22·a33 + a12·a23·a31 + a13·a21·a32)
      - (a13·a22·a31 + a11·a23·a32 + a12·a21·a33)
```

> **Sarrus só vale para ordem 3.** Não existe "Sarrus de ordem 4". Para `n ≥ 4` use Laplace
> ou triangulação. Tentar estender Sarrus é um erro clássico e dá número errado.

## 10. As onze propriedades

Todas valem para matriz quadrada. As três marcadas com (*) são as que mais aparecem em prova.

| # | Propriedade | Uso |
| --- | --- | --- |
| 1 | O determinante é **único** — não importa o caminho de cálculo | garante que Sarrus, Laplace e triangulação dão o mesmo número |
| 2 | `det A = det Aᵀ` | tudo que vale para linha vale para coluna |
| 3 | Fila nula ⟹ `det A = 0` | atalho visual |
| 4 | (*) Matriz **triangular** ⟹ `det` = **produto da diagonal principal** | base da triangulação |
| 5 | Multiplicar **uma fila** por `β` ⟹ `det B = β · det A` | cuidado: é **uma** fila, não a matriz |
| 6 | **Permutar** duas filas ⟹ `det B = −det A` | cada troca inverte o sinal |
| 7 | Duas filas **iguais ou proporcionais** ⟹ `det A = 0` | atalho visual |
| 8 | (*) Somar a uma fila **um múltiplo de outra** ⟹ `det B = det A` | **não muda nada** — é o que permite escalonar de graça |
| 9 | (*) **Teorema de Binet:** `det(A·B) = det A · det B` | |
| 10 | `det(A⁻¹) = 1 / det A` | |
| 11 | (*) **`A` tem inversa ⟺ `det(A) ≠ 0`** | o teste decisivo |

### 10.1 As três operações elementares e o determinante

Esta tabela é o resumo operacional das propriedades 5, 6 e 8 — e é o que você usa ao
triangular:

| operação elementar | efeito no determinante |
| --- | --- |
| permutar duas filas | **troca o sinal** |
| multiplicar uma fila por `β ≠ 0` | **multiplica por `β`** |
| somar a uma fila um múltiplo de outra | **não altera** |

> É por isso que a **triangulação** funciona: você usa só a terceira operação (de graça),
> chega a uma triangular, e lê o produto da diagonal. Se precisou permutar, conte os sinais.

### 10.2 Demonstração de `det(A⁻¹) = 1/det A`

Curta e cai em prova:

```
A · A⁻¹ = Iₙ         e        det Iₙ = 1

det(A · A⁻¹) = det A · det A⁻¹ = 1     (Binet)

                       =>  det A⁻¹ = 1 / det A
```

E daí sai a propriedade 11: se `det A = 0`, a igualdade `det A · det A⁻¹ = 1` é impossível
(daria `0 = 1`). Logo **`det A = 0` ⟹ não existe inversa**.

## 11. Laplace e inversa por cofator

### 11.1 Menor complementar e cofator

- **`Mᵢⱼ`** (menor complementar): a submatriz que sobra ao **remover a linha `i` e a coluna
  `j`**.
- **`Cᵢⱼ`** (cofator): `Cᵢⱼ = (−1)^(i+j) · det(Mᵢⱼ)`

O sinal `(−1)^(i+j)` segue o **tabuleiro de xadrez**, começando com `+` no canto superior
esquerdo:

```
 +  -  +  -
 -  +  -  +
 +  -  +  -
 -  +  -  +
```

### 11.2 Desenvolvimento de Laplace

```
det A = Σ aᵢⱼ · Cᵢⱼ      (fixando uma linha i, variando j;
                          ou fixando uma coluna j, variando i)
```

Você escolhe **qualquer** linha ou coluna. O resultado é o mesmo (propriedade 1).

> **A estratégia que economiza metade do tempo:** escolha a linha ou coluna com **mais
> zeros**. Cada zero mata um cofator inteiro, porque `0 · Cᵢⱼ = 0` e você nem precisa
> calcular aquele determinante. No exercício 5×5 dos slides, a segunda linha é
> `(0, 4, 0, 0, 0)` — desenvolvendo por ela, **um único cofator** precisa ser calculado, em
> vez de cinco.

### 11.3 Inversa pelo método dos cofatores

Seja `A` quadrada com `det(A) ≠ 0`:

```
             1
  A⁻¹  =  ------- · adj(A)
           det(A)
```

onde **`adj(A)` é a matriz adjunta** = **transposta da matriz dos cofatores**:

```
adj(A) = Cᵀ ,   com  C = (Cᵢⱼ) ,  Cᵢⱼ = (−1)^(i+j) · det(Mᵢⱼ)
```

> **Não esqueça de transpor.** O erro mais comum neste método é calcular a matriz dos
> cofatores e dividir por `det(A)` sem transpor. A adjunta é `Cᵀ`, não `C`.

**Atalho para ordem 2** (vale a pena decorar):

```
       | a  b |                     1     |  d  -b |
A =    |      |    =>    A⁻¹ =  --------- |        |
       | c  d |                 ad - bc   | -c   a |
```

Troca os elementos da diagonal principal, inverte o sinal da secundária, divide pelo
determinante.

---

# PARTE III — SISTEMAS LINEARES

## 12. Equações e sistemas

### 12.1 Equação linear

```
a₁x₁ + a₂x₂ + ... + aₙxₙ = b
```

- `a₁...aₙ` — **coeficientes**
- `x₁...xₙ` — **incógnitas**
- `b` — **termo independente**

**O teste de linearidade:** toda incógnita aparece elevada **à primeira potência** e
**nunca multiplicada por outra incógnita**.

| Lineares | Não lineares | Por quê |
| --- | --- | --- |
| `x + y = 2` | `xy + y = 2` | produto de incógnitas |
| `3x + 2y − z = −1` | `x² + z = 2` | incógnita ao quadrado |
| `½x + 4y − 2z + a − 3b = −5` | | |
| `x₁ + 2x₂ + 3x₃ + 4x₄ + 5x₅ = 0` | | |

**Homogênea:** termo independente igual a zero (`... = 0`). Toda equação linear homogênea
admite a **solução trivial** — todas as incógnitas valendo zero.

Uma equação linear pode ter **infinitas soluções**, **uma solução** ou **nenhuma**.

### 12.2 Sistema linear e forma matricial

Um conjunto de uma ou mais equações lineares nas mesmas incógnitas. Todo sistema se escreve
como `A · X = B`:

```
| 2x + 3y = -1 |          | 2   3 |   | x |     | -1 |
| 3x + 2y = -4 |    <=>     | 3   2 | · |   |  =  | -4 |
| 4x -  y = -5 |          | 4  -1 |   | y |     | -5 |
                         coeficientes  incóg.   independentes
```

**Matriz ampliada:** junta-se a coluna dos termos independentes à matriz dos coeficientes,
`[A | B]`. É nela que se aplicam as operações elementares.

### 12.3 A classificação (o item mais cobrado)

| Sigla | Nome | Soluções | Geometria (2 incógnitas) |
| --- | --- | --- | --- |
| **SPD** | Sistema Possível e Determinado | **uma única** | retas **concorrentes** — um ponto |
| **SPI** | Sistema Possível e Indeterminado | **infinitas** | retas **coincidentes** — uma sobre a outra |
| **SI** | Sistema Impossível | **nenhuma** | retas **paralelas** — sem interseção |

**Como reconhecer ao escalonar** — este é o quadro que resolve praticamente toda questão de
classificação:

| o que aparece na matriz escalonada | conclusão |
| --- | --- |
| uma linha de coeficientes toda zero com termo independente `k ≠ 0` (ex.: `0y = 7`) | **SI** |
| uma linha inteiramente nula, termo independente incluído (ex.: `0x = 0`) | **SPI** |
| nº de equações não nulas = nº de incógnitas | **SPD** |
| nº de equações não nulas < nº de incógnitas | **SPI** |

> **Sistema homogêneo nunca é SI.** Ele sempre aceita a solução trivial, então é
> **sempre possível** — SPD (só a trivial) ou SPI (a trivial mais infinitas outras).
> Se uma questão pedir "para que valores o sistema homogêneo é impossível", a resposta é
> **nenhum**.

### 12.4 Forma escalonada

Uma matriz está **escalonada** quando:

1. `a₁₁ ≠ 0`;
2. o número de zeros no início de cada linha **aumenta estritamente** de uma linha para a
   seguinte, exceto se a linha for toda nula;
3. as linhas nulas, se existirem, são as **últimas** (e podem ser ignoradas).

Para escalonar, usam-se as **mesmas três operações elementares** já vistas.

### 12.5 Resolução: escalonar e substituir de baixo para cima

```
| 2x + y - z =  3 |                | 2  1  -1 |  3 |
|      6y + 2z = -2 |     <=>        | 0  6   2 | -2 |
|           3z =  6 |              | 0  0   3 |  6 |
```

Está escalonada. Resolve-se de **trás para frente** (substituição retroativa):

```
3z = 6           =>  z = 2
6y + 2(2) = -2   =>  6y = -6   =>  y = -1
2x + (-1) - 2 = 3 => 2x = 6    =>  x = 3
```

**O método, em uma frase:** use operações elementares para trocar o sistema por um sistema
**equivalente** na forma escalonada; resolva por substituição; a solução do escalonado é
também a solução do original.

---

## 13. Errata conferida dos slides

Recalculei todo exercício numérico por eliminação gaussiana em **frações exatas** e,
independentemente, por expansão de Laplace. Estes três resultados **não batem** com o que
está impresso:

### 13.1 Determinantes, Exemplo 18 — o slide diz 33; o correto é **34**

```
      | 3   1   0   1 |
A  =  | 0  -1   3   4 |
      | 1   1   0   2 |
      | 0   1   1  -1 |
```

Desenvolvendo pela **terceira coluna** (entradas `0, 3, 0, 1` — só dois cofatores a
calcular):

```
M23 = | 3  1   1 |            M43 = | 3   1  1 |
      | 1  1   2 |  = -7            | 0  -1  4 |  = -13
      | 0  1  -1 |                  | 1   1  2 |

det A = a23·(-1)^(2+3)·M23  +  a43·(-1)^(4+3)·M43
      = 3·(-1)·(-7)  +  1·(-1)·(-13)
      = 21 + 13
      = 34
```

**Conferência independente, por triangulação:** trocando `L1 ↔ L3` (um sinal), depois
`L3 ← L3 − 3L1`, `L3 ← L3 − 2L2`, `L4 ← L4 + L2`, `L4 ← L4 + ⅔L3`, chega-se à diagonal
`(1, −1, −6, −17/3)`, cujo produto é `−34`; com a troca de linhas, `det A = +34`.

**Os dois caminhos dão 34.** O `33` do slide está errado.

### 13.2 Matrizes II, potência — o slide diz `A³ = [[41, 34],[84, 83]]`; o correto é `[[41, 42],[84, 83]]`

```
A = | 1  2 |       A² = |  9   8 |      (este o slide acerta)
    | 4  3 |            | 16  17 |

A³ = A²·A = |  9·1 + 8·4    9·2 + 8·3 |  = |  41   42 |
            | 16·1 + 17·4  16·2 + 17·3 |    |  84   83 |
```

O elemento `a₁₂` é `9·2 + 8·3 = 18 + 24 = **42**`, não 34. Os outros três estão certos.

### 13.3 Matrizes II, Exemplo 6 — os dois lados da identidade não fecham no slide

```
A = | 1  0 |   B = | 3  -1 |   C = | 2  1 |
    | 2  1 |       | 0   2 |       | 0  3 |

A+B = | 4  -1 |        (A+B)·C = | 8   1 |
      | 2   3 |                  | 4  11 |

A·C = | 2  1 |   B·C = | 6  0 |   A·C + B·C = | 8   1 |
      | 4  5 |         | 0  6 |               | 4  11 |
```

O slide imprime `(A+B)C = [[8, −1],[4, 11]]` mas `AC + BC = [[8, 1],[4, 11]]` — ou seja, os
dois lados de uma identidade que **deveria** dar igual aparecem diferentes. O valor correto
é **`+1`** nos dois: `[[8, 1],[4, 11]]`. (Possivelmente só um sinal de menos indevido na
digitação do slide — vale conferir no original.)

### 13.4 Erros de redação (não mudam resultado, mas atrapalham o estudo)

| Slide | Está escrito | Deveria ser |
| --- | --- | --- |
| Matrizes I, Def. 1 | "com `1 ≤ i ≤ m`, `1 ≤ i ≤ n`" | o segundo é **`1 ≤ j ≤ n`** |
| Matrizes I, Ex. 1 | "`A = aᵢ,ⱼ ∈ M₃ₓ₃(R)`" seguido de uma matriz **3×4** | a matriz é `M₃ₓ₄` |
| Matrizes I, Def. 13 | "as **linhas** de B iguais às **linhas** de A" | linhas de B = **colunas** de A |
| Matrizes I, §4 | seção chamada "Transformação" | o termo técnico é **transposição** |
| Matrizes II, Def. 1 | "com `i = {1..m}` e `i = {1..n}`" | o segundo é **`j = {1..s}`** |
| Matrizes II, Prop. 1 | "Seja as Matrizes A, B e C, todas de **mesma ordem**" | para **produto** basta serem **compatíveis** (`colunas de A = linhas de B`); exigir mesma ordem é falso |
| Sist. Lineares, Def. 10 | "escalonada (**matriz triangular superior**)" | escalonada ≠ triangular superior: a escalonada **não precisa ser quadrada** |
| Sist. Lineares, Ex. 2 | `(x,y,z) = (2; −1, −5)` | ponto-e-vírgula é vírgula |

---

## 14. Exercícios dos slides, resolvidos

Todos conferidos por cálculo independente.

### MATRIZES I

**E1.** *Dada a matriz de notas, identifique `a₁₂`, `a₂₃`, `a₃₃`, o significado de cada um,
a média do aluno 2 e o total de pontos do 1º bimestre.*

A matriz é `3 × 4` (3 alunos, 4 bimestres) — o slide escreve `M₃ₓ₃`, mas a matriz mostrada
tem quatro colunas.

| elemento | valor | significado |
| --- | --- | --- |
| `a₁₂` | **8.0** | nota do **aluno 1** no **2º bimestre** |
| `a₂₃` | **8.5** | nota do **aluno 2** no **3º bimestre** |
| `a₃₃` | **7.0** | nota do **aluno 3** no **3º bimestre** |

- **Média do aluno 2** = média da **linha 2** = `(6.0 + 7.5 + 8.5 + 7.0) / 4 = 29.0 / 4 =`
  **`7.25`**
- **Total do 1º bimestre** = soma da **coluna 1** = `7.5 + 6.0 + 9.0 =` **`22.5`**

> Generalizando: **média de um aluno** percorre uma **linha**; **estatística de um bimestre**
> percorre uma **coluna**. Essa leitura é o que a matéria toda quer instalar.

**E2.** *Descreva a matriz pela lei de formação.*

**a)** `A = (aᵢⱼ)₂ₓ₃`, `aᵢⱼ = i² − j²`

```
a11 = 1-1 =  0     a12 = 1-4 = -3     a13 = 1-9 = -8
a21 = 4-1 =  3     a22 = 4-4 =  0     a23 = 4-9 = -5
```

```
A = |  0  -3  -8 |
    |  3   0  -5 |
```

**b)** `B = (bᵢⱼ)₃ₓ₃`, com `bᵢⱼ = 3` se `i ≤ j`, e `bᵢⱼ = 2j − 2i` se `i > j`

A condição `i ≤ j` cobre a diagonal e tudo acima dela; `i > j` cobre o que está abaixo.

```
b11 = 3   b12 = 3   b13 = 3
b21 = 2(1)-2(2) = -2      b22 = 3   b23 = 3
b31 = 2(1)-2(3) = -4      b32 = 2(2)-2(3) = -2      b33 = 3
```

```
B = |  3   3   3 |
    | -2   3   3 |
    | -4  -2   3 |
```

**E3.** *Determine `x, y, z, w` tais que `A = B`.*

```
A = | 2x - y   w² - 3z |     B = | -4   0 |
    |   3y      5w - 7 |         | 12   3 |
```

Igualdade elemento a elemento gera quatro equações. **Resolva na ordem que destrava**:

```
posição (2,1):  3y = 12           =>  y = 4
posição (1,1):  2x - y = -4  =>  2x - 4 = -4  =>  2x = 0   =>  x = 0
posição (2,2):  5w - 7 = 3        =>  5w = 10  =>  w = 2
posição (1,2):  w² - 3z = 0  =>  4 - 3z = 0   =>  z = 4/3
```

**Resposta: `x = 0`, `y = 4`, `z = 4/3`, `w = 2`.**

**E4 / E5.** *Algoritmo que verifique se duas matrizes são iguais (2×2, depois `n × m`).*

```
função saoIguais(A, B):
    se linhas(A) != linhas(B) ou colunas(A) != colunas(B):
        retorna falso                 # ordem diferente: nem compara elementos
    para i de 1 até linhas(A):
        para j de 1 até colunas(A):
            se A[i][j] != B[i][j]:
                retorna falso         # saída antecipada no primeiro diferente
    retorna verdadeiro
```

**O ponto do exercício:** testar a **ordem primeiro** e sair no primeiro elemento diferente.
A versão `n × m` é a mesma — é por isso que o professor pede as duas: para você perceber que
fixar `2×2` foi uma limitação desnecessária desde o início.

**E6.** *Transposta de `A` (3×2).*

```
A = | -1   2 |                        | -1   6   4 |
    |  6   0 |        =>      Aᵀ  =   |  2   0  -3 |
    |  4  -3 |
```

`A` é `3×2`; `Aᵀ` é `2×3`. **A ordem inverte.**

**E7.** *Transposta de `A` (3×3).*

```
A = |  6   1  -8 |
    |  1  -3   2 |
    | -8   2   0 |
```

Esta matriz é **simétrica** (`aᵢⱼ = aⱼᵢ`: veja o `1` em (1,2) e (2,1); o `−8` em (1,3) e
(3,1); o `2` em (2,3) e (3,2)). Logo **`Aᵀ = A`** — a transposta é a própria matriz.

**E8–E12.** *Verificações das propriedades da transposta* — já resolvidas nos slides.
Resumo do que cada uma mostra:

| Ex | Propriedade | Verificação |
| --- | --- | --- |
| 8 | `(Aᵀ)ᵀ = A` | transpor duas vezes volta ao início |
| 9 | `(A+B)ᵀ = Aᵀ + Bᵀ` | ambos dão `[[6,10],[8,12]]` |
| 10 | `(αA)ᵀ = αAᵀ` | ambos dão `[[8,0],[−4,12]]` |
| 11 | `Iᵀ = I` | a identidade é simétrica |
| 12 | `Dᵀ = D` | toda diagonal é simétrica |

**E13.** *Algoritmo de transposição.*

```
função transposta(A):
    B = nova matriz de ordem colunas(A) x linhas(A)     # ordem INVERTIDA
    para i de 1 até linhas(A):
        para j de 1 até colunas(A):
            B[j][i] = A[i][j]                          # índices trocados
    retorna B
```

**A linha que importa** é `B[j][i] = A[i][j]`. E note que `B` precisa ser alocada com a ordem
invertida — transpor "no lugar" (*in place*) só funciona para matriz quadrada.

**E14.** *`C = A + B`.*

```
A = | 1  2  3 |       B = | 9  8  7 |       C = | 10  10  10 |
    | 4  5  6 |           | 6  5  4 |           | 10  10  10 |
    | 7  8  9 |           | 3  2  1 |           | 10  10  10 |
```

**E15 / E16.** *Algoritmo de soma (2×2, depois `n × n`).*

```
função soma(A, B):
    se ordem(A) != ordem(B):
        erro "matrizes de ordens diferentes não podem ser somadas"
    C = nova matriz de mesma ordem
    para i de 1 até linhas(A):
        para j de 1 até colunas(A):
            C[i][j] = A[i][j] + B[i][j]
    retorna C
```

**E17.** *`B = αA`, com `α = √2`.*

```
A = |  √2    2   1/2 |              B = √2·A = |    2     2√2   √2/2 |
    | 1/√2  18    6  |                         |    1    18√2   6√2  |
    |   7    0   -2  |                         |  7√2      0   -2√2  |
```

Contas úteis: `√2·√2 = 2`; `√2·(1/√2) = 1`; `√2·(1/2) = √2/2`.

> Os valores fracionários desta matriz foram reconstruídos da extração do PDF — confira as
> frações contra o slide original antes de usar como gabarito.

**E18.** *`X = 4A − 3B + 5C`.*

```
A = | 2   3   8 |    B = | 5  -7  -9 |    C = | 0  9  8 |
    | 4  -1  -6 |        | 0   4   1 |        | 1  4  6 |
```

Calcule elemento a elemento — é a operação mais segura:

```
x11 = 4( 2) - 3( 5) + 5(0) =   8 - 15 +  0 =  -7
x12 = 4( 3) - 3(-7) + 5(9) =  12 + 21 + 45 =  78
x13 = 4( 8) - 3(-9) + 5(8) =  32 + 27 + 40 =  99
x21 = 4( 4) - 3( 0) + 5(1) =  16 -  0 +  5 =  21
x22 = 4(-1) - 3( 4) + 5(4) =  -4 - 12 + 20 =   4
x23 = 4(-6) - 3( 1) + 5(6) = -24 -  3 + 30 =   3
```

```
X = | -7  78  99 |
    | 21   4   3 |
```

> Os sinais são a única dificuldade aqui: `−3B` onde `B` já tem negativos vira soma.
> `−3 · (−7) = +21`.

**E19–E26.** *Verificações das oito propriedades da adição e do escalar* — resolvidas nos
slides. Todas conferem. Nada a corrigir.

**E27.** *Algoritmo de multiplicação por escalar.*

```
função multiplicaEscalar(alfa, A):
    B = nova matriz de mesma ordem que A
    para i de 1 até linhas(A):
        para j de 1 até colunas(A):
            B[i][j] = alfa * A[i][j]
    retorna B
```

O mais simples dos três: nenhuma verificação de compatibilidade, nenhuma troca de índice.

---

### MATRIZES II

**E1.** *`C = A · B`.*

```
A (2x3) = | 1  2  3 |        B (3x2) = |  7   8 |
          | 4  5  6 |                  |  9  10 |
                                       | 11  12 |
```

Compatível: colunas de `A` (3) = linhas de `B` (3). Resultado `2 × 2`.

```
c11 = 1(7) + 2(9)  + 3(11) =   7 + 18 + 33 =  58
c12 = 1(8) + 2(10) + 3(12) =   8 + 20 + 36 =  64
c21 = 4(7) + 5(9)  + 6(11) =  28 + 45 + 66 = 139
c22 = 4(8) + 5(10) + 6(12) =  32 + 50 + 72 = 154
```

```
C = |  58   64 |
    | 139  154 |
```

**E2.** *`A · B` com `A` de ordem 4×2 e `B` de ordem 2×4.*

```
A = |  1  -2 |          B = | 1  3  -5  -7 |
    |  3   1 |              | 6  2  -8   3 |
    |  7  -4 |
    |  5   9 |
```

Compatível (2 = 2); resultado **`4 × 4`**:

```
A·B = | -11   -1   11  -13 |
      |   9   11  -23  -18 |
      | -17   13   -3  -61 |
      |  59   33  -97   -8 |
```

> Repare: duas matrizes "magras" produziram uma `4×4`. E **`B·A` também existe** aqui, mas é
> `2×2` — ordem completamente diferente. Demonstração direta de que `AB ≠ BA`.

**E3.** *(STEINBRUCH; WINTERLE) Efetue `A · X`.*

**a)**

```
| 2   6 |   | x |     | 2x + 6y |
|       | · |   |  =  |         |
| -5  4 |   | y |     | -5x + 4y |
```

**b)**

```
| 1   2   3 |   | x1 |     |  x1 + 2x2 + 3x3 |
| 2  -5   7 | · | x2 |  =  | 2x1 - 5x2 + 7x3 |
| 3   9  -8 |   | x3 |     | 3x1 + 9x2 - 8x3 |
```

> **O que este exercício está ensinando:** o produto `A·X` reconstrói exatamente o **lado
> esquerdo de um sistema linear**. É a ponte entre Matrizes II e Sistemas Lineares — todo
> sistema é `A·X = B`.

**E4–E9.** *Verificações das seis propriedades do produto* — resolvidas nos slides. Todas
conferem, **exceto o Exemplo 6**, cujo item `(A+B)C` está com um sinal trocado (ver 13.3).

**E10.** *Verifique se `B` é a inversa de `A`.*

**Par 1:**

```
A = | -0,5  -1,5   1   |        B = | -12  -4   14 |
    | -0,5  -2,5   0,5 |            |   2   0   -2 |
    | -0,5  -2     1   |            |  -2  -2    4 |
```

Calculando `A·B`:

```
(1,1): (-0,5)(-12) + (-1,5)(2) + (1)(-2)    =  6 - 3 - 2 = 1  ✓
(1,2): (-0,5)(-4)  + (-1,5)(0) + (1)(-2)    =  2 + 0 - 2 = 0  ✓
(1,3): (-0,5)(14)  + (-1,5)(-2) + (1)(4)    = -7 + 3 + 4 = 0  ✓
(2,1): (-0,5)(-12) + (-2,5)(2) + (0,5)(-2)  =  6 - 5 - 1 = 0  ✓
(2,2): (-0,5)(-4)  + (-2,5)(0) + (0,5)(-2)  =  2 + 0 - 1 = 1  ✓
(2,3): (-0,5)(14)  + (-2,5)(-2) + (0,5)(4)  = -7 + 5 + 2 = 0  ✓
(3,1): (-0,5)(-12) + (-2)(2)   + (1)(-2)    =  6 - 4 - 2 = 0  ✓
(3,2): (-0,5)(-4)  + (-2)(0)   + (1)(-2)    =  2 + 0 - 2 = 0  ✓
(3,3): (-0,5)(14)  + (-2)(-2)  + (1)(4)     = -7 + 4 + 4 = 1  ✓
```

`A·B = I₃`. **Sim, `B = A⁻¹`.**

**Par 2:**

```
A = | -2  -4  -6 |         B = | -1,5   2   -1,5 |
    | -4  -6  -6 |             |  2    -2,5  1,5 |
    | -4  -4  -2 |             | -1     1   -0,5 |
```

Mesma verificação, elemento a elemento, dá `A·B = I₃`. **Sim, `B = A⁻¹`.**

> **Atalho legítimo:** pela definição, `A·B = B·A = I`. Mas para matrizes **quadradas**, se
> `A·B = I` então automaticamente `B·A = I`. Basta calcular **um** dos produtos. Em prova
> isso economiza metade do trabalho.

**E11.** *Inversa de `A = [[1,2],[0,2]]` por operações elementares.*

```
[ A | I ]      | 1  2 | 1  0 |
               | 0  2 | 0  1 |

L2 ← L2 / 2    | 1  2 | 1   0   |
               | 0  1 | 0   1/2 |

L1 ← L1 - 2L2  | 1  0 | 1  -1   |
               | 0  1 | 0   1/2 |
```

```
A⁻¹ = | 1   -1   |
      | 0    0,5 |
```

**Conferência:** `[[1,2],[0,2]]·[[1,−1],[0,0.5]] = [[1, −1+1],[0, 1]] = I` ✓

**E12.** *Inversa de `A = [[1,2,0],[2,3,0],[0,0,3]]`.*

Esta matriz é **diagonal por blocos**: um bloco `2×2` no canto superior esquerdo e um bloco
`1×1` isolado. Cada bloco se inverte separadamente.

Bloco `[[1,2],[2,3]]`: determinante `= 1·3 − 2·2 = −1`. Pelo atalho de ordem 2:

```
 1   |  3  -2 |      | -3   2 |
--- ·|        |  =   |        |
 -1  | -2   1 |      |  2  -1 |
```

Bloco `[3]`: inversa `[1/3]`.

```
A⁻¹ = | -3   2   0   |
      |  2  -1   0   |
      |  0   0   1/3 |
```

**Conferência:** linha 1 de `A·A⁻¹` = `(1(−3)+2(2)+0, 1(2)+2(−1)+0, 0) = (1, 0, 0)` ✓

**E13.** *Verifique se `M` é ortogonal.*

```
M = | 1/2    √3/2 |
    | √3/2   1/2  |
```

`M` é simétrica, então `Mᵀ = M`, e o teste vira `M² = I`:

```
(1,1): (1/2)(1/2)   + (√3/2)(√3/2) = 1/4 + 3/4 = 1        ✓
(1,2): (1/2)(√3/2)  + (√3/2)(1/2)  = √3/4 + √3/4 = √3/2   ✗  (deveria ser 0)
```

```
M·Mᵀ = |  1     √3/2 |   ≠   I
       | √3/2    1   |
```

**Resposta: `M` NÃO é ortogonal.**

**Por quê, conceitualmente:** as duas linhas têm norma 1 (`√(1/4 + 3/4) = 1`), mas **não são
perpendiculares** — o produto escalar entre elas é `√3/2 ≠ 0`. Ortogonalidade exige as duas
coisas: **normalizadas E perpendiculares**.

> **Como seria a versão ortogonal:** trocando o sinal de um elemento da antidiagonal,
> `[[1/2, −√3/2],[√3/2, 1/2]]` **é** ortogonal — é a matriz de **rotação de 60°**. Toda
> matriz de rotação é ortogonal. A diferença entre passar e não passar no teste foi um único
> sinal de menos.

**E14.** *Potência: calcule `A²` e `A³` para `A = [[1,2],[4,3]]`.*

```
A² = A·A = | 1·1 + 2·4    1·2 + 2·3 |  = |  9   8 |
           | 4·1 + 3·4    4·2 + 3·3 |    | 16  17 |

A³ = A²·A = |  9·1 + 8·4     9·2 + 8·3 |  = | 41  42 |
            | 16·1 + 17·4   16·2 + 17·3 |   | 84  83 |
```

**`A³ = [[41, 42],[84, 83]]`** — o slide traz `34` no lugar do `42` (ver 13.2).

**E15.** *Verifique que `A` é idempotente.*

```
A = |  2  -1   1 |
    | -3   4  -3 |
    | -5   5  -4 |
```

Calculando `A²`, linha por linha:

```
linha 1: ( 2)(2)+(-1)(-3)+( 1)(-5) =  4+3-5 =  2   ✓
         ( 2)(-1)+(-1)(4)+( 1)(5)  = -2-4+5 = -1   ✓
         ( 2)(1)+(-1)(-3)+( 1)(-4) =  2+3-4 =  1   ✓
linha 2: (-3)(2)+( 4)(-3)+(-3)(-5) = -6-12+15 = -3 ✓
         (-3)(-1)+( 4)(4)+(-3)(5)  =  3+16-15 =  4 ✓
         (-3)(1)+( 4)(-3)+(-3)(-4) = -3-12+12 = -3 ✓
linha 3: (-5)(2)+( 5)(-3)+(-4)(-5) = -10-15+20 = -5 ✓
         (-5)(-1)+( 5)(4)+(-4)(5)  =  5+20-20 =  5 ✓
         (-5)(1)+( 5)(-3)+(-4)(-4) = -5-15+16 = -4 ✓
```

**`A² = A`** ⟹ `A` é **idempotente**, com período `2 − 1 = 1`. Logo
`A³ = A⁴ = ... = Aⁿ = A` para todo `n`.

**E16.** *Verifique se `A` é nihilpotente.*

```
A = |  1  -1   1 |
    | -3   3  -3 |
    | -4   4  -4 |
```

```
linha 1: (1)(1)+(-1)(-3)+(1)(-4)   =  1+3-4 = 0
         (1)(-1)+(-1)(3)+(1)(4)    = -1-3+4 = 0
         (1)(1)+(-1)(-3)+(1)(-4)   =  1+3-4 = 0
linha 2: (-3)(1)+(3)(-3)+(-3)(-4)  = -3-9+12 = 0
         (-3)(-1)+(3)(3)+(-3)(4)   =  3+9-12 = 0
         (-3)(1)+(3)(-3)+(-3)(-4)  = -3-9+12 = 0
linha 3: (-4)(1)+(4)(-3)+(-4)(-4)  = -4-12+16 = 0
         (-4)(-1)+(4)(3)+(-4)(4)   =  4+12-16 = 0
         (-4)(1)+(4)(-3)+(-4)(-4)  = -4-12+16 = 0
```

```
A² = | 0  0  0 |
     | 0  0  0 |
     | 0  0  0 |
```

**Sim: `A` é nihilpotente de índice `p = 2`.**

> **Guarde este exemplo.** Uma matriz **sem um único zero** tem quadrado **inteiramente
> nulo**. Nos reais, `x² = 0 => x = 0`; em matrizes, **não**. É o contraexemplo mais útil da
> disciplina, e aparece em questões do tipo "verdadeiro ou falso".

---

### DETERMINANTES

**E1.** `det [[−2, 4],[2, 3]] = (−2)(3) − (4)(2) = −6 − 8 =` **`−14`** ✓ (confere com o slide)

**E2.** `det [[−3, −2],[7, −4]] = (−3)(−4) − (−2)(7) = 12 + 14 =` **`26`** ✓

**E3.** Por Sarrus, `A = [[3,4,1],[−5,−2,−9],[7,8,6]]`:

```
principais:  (3)(-2)(6) + (4)(-9)(7) + (1)(-5)(8) = -36 - 252 - 40 = -328
secundárias: (1)(-2)(7) + (3)(-9)(8) + (4)(-5)(6) = -14 - 216 - 120 = -350

det A = -328 - (-350) = 22                                            ✓
```

**E4.** `det [[1,2,3],[2,5,6],[2,5,8]]`:

```
= 1(5·8 - 6·5) - 2(2·8 - 6·2) + 3(2·5 - 5·2)
= 1(40-30) - 2(16-12) + 3(10-10)
= 10 - 8 + 0 = 2                                                      ✓
```

**E5–E12.** *As conjecturas das propriedades* — cada exemplo existe para você **descobrir** a
propriedade antes de lê-la:

| Ex | Matrizes | Resultados | Propriedade descoberta |
| --- | --- | --- | --- |
| 5 | `A` e `Aᵀ` | ambos `−14` | `det A = det Aᵀ` |
| 6 | 3×3 com última linha nula | `0` | fila nula ⟹ `det = 0` |
| 7 | `[[−2,4,1],[0,3,−5],[0,0,1]]` | `(−2)(3)(1) = −6` | triangular ⟹ produto da diagonal |
| 8 | `A` = `−14`; `B` (2ª linha × 4) = `−56` | `−56 = 4·(−14)` | fila × β ⟹ `det × β` |
| 10 | `[[1,4],[2,3]]` = `−5`; linhas trocadas = `+5` | sinal invertido | permuta ⟹ troca o sinal |
| 11 | `[[1,4],[2,8]]` (linhas proporcionais) | `8 − 8 = 0` | filas proporcionais ⟹ `det = 0` |
| 12 | `A` = `−14`; `B` (`L2 ← L2 + 4L1`) = `−14` | iguais | somar múltiplo de outra fila **não altera** |

**E9.** *Sabendo que `det [[1,2,3],[2,5,6],[2,5,8]] = 2`, qual é o determinante de
`[[2,4,6],[14,35,42],[−2,−5,−8]]`?*

Não calcule — **reconheça os fatores**. Cada linha da nova matriz é um múltiplo de uma linha
da original:

```
linha 1:  ( 2,  4,  6)  =   2 · (1, 2, 3)
linha 2:  (14, 35, 42)  =   7 · (2, 5, 6)
linha 3:  (-2, -5, -8)  =  -1 · (2, 5, 8)
```

Pela propriedade 5, **cada** fator sai multiplicando:

```
det = 2 · 7 · (-1) · 2  =  -28
```

**Resposta: `−28`.**

> **Atenção à pegadinha conceitual:** `det(kA) = kⁿ · det(A)` para uma matriz `n × n`, porque
> multiplicar a matriz inteira por `k` multiplica **cada uma das n linhas** por `k`. Só sai
> `k·det(A)` quando **uma única fila** foi multiplicada.

**E13.** *Teorema de Binet.*

```
A = | -1   1 |      B = |  4  -1 |
    | -3   1 |          | -2   1 |

det A = (-1)(1) - (1)(-3) = -1 + 3 = 2
det B = (4)(1) - (-1)(-2) =  4 - 2 = 2

A·B = | (-1)(4)+(1)(-2)   (-1)(-1)+(1)(1) |  = |  -6   2 |
      | (-3)(4)+(1)(-2)   (-3)(-1)+(1)(1) |    | -14   4 |

det(A·B) = (-6)(4) - (2)(-14) = -24 + 28 = 4
```

**`det(A·B) = 4 = 2 × 2 = det A · det B`** ✓ — Teorema de Binet confirmado.

**E14.** *Calcule `A⁻¹`, `det(A)` e `det(A⁻¹)` para `A = [[−1,1],[−3,1]]`.*

`det A = 2` (calculado acima). Pelo atalho de ordem 2:

```
       1    |  1  -1 |     | 0,5  -0,5 |
A⁻¹ = --- · |        |  =  |           |
       2    |  3  -1 |     | 1,5  -0,5 |
```

**Conferência:** `A·A⁻¹ = [[−0.5+1.5, 0.5−0.5],[−1.5+1.5, 1.5−0.5]] = [[1,0],[0,1]]` ✓

```
det(A⁻¹) = (0,5)(-0,5) - (-0,5)(1,5) = -0,25 + 0,75 = 0,5 = 1/2
```

**`det(A⁻¹) = 1/2 = 1/det(A)`** ✓

**E15.** *Demonstre que `det A⁻¹ = 1/det A`.* — feita na seção 10.2.

**E16.** *Usando as propriedades, calcule `det A` (solução: 165).*

```
A = | 0   1   5 |
    | 3  -6   9 |
    | 2   6   1 |
```

Por Sarrus:

```
principais:  (0)(-6)(1) + (1)(9)(2) + (5)(3)(6) =  0 + 18 + 90 = 108
secundárias: (5)(-6)(2) + (0)(9)(6) + (1)(3)(1) = -60 + 0 + 3  = -57

det A = 108 - (-57) = 165                                             ✓
```

**Pelo caminho das propriedades** (que é o que o enunciado pede): a segunda linha é
`3·(1, −2, 3)`, então o `3` sai para fora, e você trabalha com um determinante de números
menores. Qualquer caminho dá 165 (propriedade 1).

**E17.** *(STEINBRUCH; WINTERLE) Calcule `det A` por triangulação (solução: 2).*

```
A = | -2   3   1  -1 |
    |  0   1   2   3 |
    |  1  -1   1  -2 |
    |  4  -3   5   1 |
```

Passos (guardando os sinais das permutas):

```
L1 ↔ L3              (1 permuta => sinal invertido)
    |  1  -1   1  -2 |
    |  0   1   2   3 |
    | -2   3   1  -1 |
    |  4  -3   5   1 |

L3 ← L3 + 2L1        (não altera o determinante)
L4 ← L4 - 4L1        (não altera)
    |  1  -1   1  -2 |
    |  0   1   2   3 |
    |  0   1   3  -5 |
    |  0   1   1   9 |

L3 ← L3 - L2   e   L4 ← L4 - L2      (não alteram)
    |  1  -1   1  -2 |
    |  0   1   2   3 |
    |  0   0   1  -8 |
    |  0   0  -1   6 |

L4 ← L4 + L3         (não altera)
    |  1  -1   1  -2 |
    |  0   1   2   3 |
    |  0   0   1  -8 |
    |  0   0   0  -2 |

produto da diagonal = 1 · 1 · 1 · (-2) = -2
uma permuta => det A = -(-2) = 2                                       ✓
```

**E18.** *Calcule `det A` de ordem 4 por Laplace.*

Resolvido em detalhe na seção 13.1. **`det A = 34`** (o slide traz 33).

**E19.** *Calcule `det A` de ordem 5 por Laplace (solução: −48).*

```
A = | 1   2   3  -3   1 |
    | 0   4   0   0   0 |     <-- quatro zeros!
    | 0   1   0   1   1 |
    | 0  -6   6   1   3 |
    | 0   2   0  -1   1 |
```

**A estratégia inteira deste exercício é enxergar a segunda linha.** Ela tem quatro zeros —
desenvolvendo por ela, **um único cofator** precisa ser calculado, em vez de cinco.

```
det A = a22 · (-1)^(2+2) · det(M22)  =  4 · det(M22)
```

onde `M22` remove a linha 2 e a coluna 2:

```
M22 = | 1   3  -3   1 |
      | 0   0   1   1 |
      | 0   6   1   3 |
      | 0   0  -1   1 |
```

Esta, por sua vez, tem a primeira coluna com um só elemento não nulo — desenvolva por ela:

```
det(M22) = 1 · det | 0   1   1 |
                   | 6   1   3 |
                   | 0  -1   1 |
```

Desenvolvendo essa 3×3 pela **primeira coluna** (um só não nulo, o `6`):

```
= 6 · (-1)^(2+1) · det | 1   1 |  =  -6 · (1·1 - 1·(-1))  =  -6 · 2  =  -12
                       | -1  1 |
```

Logo `det(M22) = −12` e **`det A = 4 · (−12) = −48`** ✓ (confere com o slide)

**E20.** *Calcule a inversa de `A = [[1,2],[3,4]]` pelo método dos cofatores.*

**Passo 1 — determinante:** `det A = (1)(4) − (2)(3) = 4 − 6 = −2`. Como `≠ 0`, a inversa
existe.

**Passo 2 — matriz dos cofatores.** `Cᵢⱼ = (−1)^(i+j) · det(Mᵢⱼ)`; em ordem 2 cada menor é um
único elemento:

```
C11 = (+1)·det[4] =  4        C12 = (-1)·det[3] = -3
C21 = (-1)·det[2] = -2        C22 = (+1)·det[1] =  1

C = |  4  -3 |
    | -2   1 |
```

**Passo 3 — adjunta (transposta dos cofatores).** *Este é o passo que se esquece:*

```
adj(A) = Cᵀ = |  4  -2 |
              | -3   1 |
```

**Passo 4 — dividir pelo determinante:**

```
         1     |  4  -2 |     | -2     1   |
A⁻¹ =  ---- ·  |        |  =  |            |
        -2     | -3   1 |     |  1,5  -0,5 |
```

**Conferência:** `[[1,2],[3,4]]·[[−2,1],[1.5,−0.5]] = [[−2+3, 1−1],[−6+6, 3−2]] = I` ✓

---

### SISTEMAS LINEARES

**E1.** *Classifique como linear ou não linear.* — tabela na seção 12.1.

**E2.** *Para `x + 3y − z = 4`, verifique se são soluções.*

```
(2, -1, -5):   2 + 3(-1) - (-5) = 2 - 3 + 5 = 4  ✓  É SOLUÇÃO
(3,  0,  2):   3 + 3(0)  - 2    = 3 + 0 - 2 = 1  ✗  NÃO É (1 ≠ 4)
```

**E3.** *Escreva `3(m + 2n) − 5(n − 4p + 1) + 2(m + 6p − 3) = 0` na forma canônica,
identifique coeficientes e termo independente, e verifique se `(7, 8, −1)` é solução.*

```
3(m + 2n) - 5(n - 4p + 1) + 2(m + 6p - 3) = 0
3m + 6n - 5n + 20p - 5 + 2m + 12p - 6 = 0
(3m + 2m) + (6n - 5n) + (20p + 12p) - 11 = 0
5m + n + 32p - 11 = 0
```

**Forma canônica: `5m + n + 32p = 11`**

| | valor |
| --- | --- |
| coeficiente de `m` | **5** |
| coeficiente de `n` | **1** |
| coeficiente de `p` | **32** |
| termo independente | **11** |

**Verificação de `(m, n, p) = (7, 8, −1)`:**

```
5(7) + 8 + 32(-1) = 35 + 8 - 32 = 11  ✓
```

**Sim, `(7, 8, −1)` é solução.**

**E4.** *Mostre que `3x + y = 5` admite infinitas soluções.*

Isole uma incógnita: `y = 5 − 3x`. Para **cada** valor real atribuído a `x` existe um `y` que
satisfaz a equação. Como os reais são infinitos, as soluções são infinitas:

```
solução geral:  (x, y) = (k, 5 - 3k),  k ∈ R

k = 0  →  (0,  5)        k = 1  →  (1,  2)
k = 2  →  (2, -1)        k = -1 →  (-1, 8)
```

Geometricamente: a equação descreve **uma reta**, e toda reta tem infinitos pontos.

**E5.** *A equação `(a² − 4)x + (a + b − 5)y = a − 2` é impossível. Determine `a` e `b`.*

Uma equação linear é **impossível** quando **todos os coeficientes são zero** mas o **termo
independente não é** — vira `0 = k` com `k ≠ 0`. Três condições simultâneas:

```
(1)  a² - 4  = 0     =>  a = 2  ou  a = -2
(2)  a + b - 5 = 0   =>  b = 5 - a
(3)  a - 2  ≠ 0      =>  a ≠ 2        <-- elimina a = 2
```

De (1) e (3): **`a = −2`**. Substituindo em (2): `b = 5 − (−2) =` **`7`**.

**Resposta: `a = −2`, `b = 7`.**

**Conferência:** a equação vira `0x + 0y = −4`, ou seja `0 = −4`. Impossível ✓

**E6.** *A equação homogênea `(p − 1)x + (2p − 3)y = q − 4` tem `(x, y) = (−1, 3)` como
solução. Determine `p` e `q`.*

Duas informações, uma de cada vez:

```
(1) HOMOGÊNEA => termo independente = 0:
    q - 4 = 0   =>   q = 4

(2) (-1, 3) É SOLUÇÃO:
    (p - 1)(-1) + (2p - 3)(3) = 0
    -p + 1 + 6p - 9 = 0
    5p - 8 = 0
    p = 8/5
```

**Resposta: `p = 8/5`, `q = 4`.**

**E7.** *No sistema `x + 3y = m`, `5x − my = n − 3`, determine `m` e `n` sabendo que
`(x, y) = (2, −1)` é solução.*

```
1ª equação:  2 + 3(-1) = m      =>   2 - 3 = m   =>   m = -1

2ª equação (já com m = -1):
             5(2) - (-1)(-1) = n - 3
             10 - 1 = n - 3
             9 = n - 3          =>   n = 12
```

**Resposta: `m = −1`, `n = 12`.**

> **A ordem importa.** Comece pela equação que tem **só uma** incógnita nova (`m`), depois
> leve o valor para a outra. Atacar a segunda equação primeiro deixaria duas incógnitas (`m`
> e `n`) numa só equação.

**E8.** *Mostre por substituição que `2a + b = −1`, `4a + 2b = −2` é SPI, obtenha a solução
geral, e os valores de `m` e `n` sabendo que `(−3, m)` e `(n, −3)` são soluções.*

**Passo 1 — mostrar que é SPI.** Da 1ª equação, `b = −1 − 2a`. Substituindo na 2ª:

```
4a + 2(-1 - 2a) = -2
4a - 2 - 4a = -2
       0a = 0        <-- identidade: vale para TODO a
```

Chegar a `0 = 0` (e não a `0 = k ≠ 0`) significa que a segunda equação **não acrescenta
informação** — é a primeira multiplicada por 2. O sistema tem **duas equações mas só uma é
independente**: uma equação, duas incógnitas ⟹ **SPI**.

**Passo 2 — solução geral:**

```
(a, b) = (k, -1 - 2k),   k ∈ R
```

**Passo 3 — os valores pedidos:**

```
(a, b) = (-3, m):   m = -1 - 2(-3) = -1 + 6  =>  m = 5
(a, b) = (n, -3):   -3 = -1 - 2n  =>  -2 = -2n  =>  n = 1
```

**Resposta: SPI; solução geral `(k, −1 − 2k)`; `m = 5`, `n = 1`.**

**E9.** *Verifique se os ternos são soluções do sistema.*

```
| m +  n - 2p = -1 |
| 2m -  n +  p =  6 |
|       n + 3p =  2 |
```

**a) `(2, −1, 1)`:**

```
eq1:   2 + (-1) - 2(1) = 2 - 1 - 2 = -1   ✓
eq2:  2(2) - (-1) + 1  = 4 + 1 + 1 =  6   ✓
eq3:      (-1) + 3(1)  = -1 + 3    =  2   ✓
```

**É solução** — satisfaz as três.

**b) `(3, 4, 4)`:**

```
eq1:   3 + 4 - 2(4) = 3 + 4 - 8 = -1      ✓
eq2:  2(3) - 4 + 4  = 6 - 4 + 4 =  6      ✓
eq3:      4 + 3(4)  = 4 + 12    = 16 ≠ 2  ✗
```

**NÃO é solução.**

> **A armadilha deste exercício** é parar depois de duas equações darem certo. A definição
> exige que **todas** as equações sejam satisfeitas. Duas de três não é solução.

**E10.** *`r` é paralela a `s`, e `t` intersecta `r` e `s`. Classifique os sistemas.*

| sistema | geometria | classificação |
| --- | --- | --- |
| **a)** `r` e `s` | paralelas — **nenhum** ponto comum | **SI** |
| **b)** `r` e `t` | concorrentes — **um** ponto comum | **SPD** |
| **c)** `r`, `s` e `t` | não existe ponto comum às **três** (`r // s` já impede) | **SI** |

> **O item (c) é a pegadinha.** Cada par `r`–`t` e `s`–`t` tem interseção, mas a solução de
> um sistema precisa satisfazer **todas** as equações **ao mesmo tempo** — um ponto comum às
> três retas. Como `r // s` nunca se encontram, esse ponto não existe. **SI**.

**E11.** *Verifique se a matriz está escalonada.*

```
| 4  -7   0   8   3 |    <- 0 zeros no início
| 0   3   0  -2  -1 |    <- 1 zero
| 0   0  -2   7   2 |    <- 2 zeros
```

Checando as três condições: `a₁₁ = 4 ≠ 0` ✓; a contagem de zeros iniciais é `0 → 1 → 2`,
**estritamente crescente** ✓; não há linhas nulas ✓.

**Sim, está escalonada.**

**E12.** *Transforme na forma escalonada.*

```
| 4  -7   0  -14   4 |    <- 0 zeros
| 3   0   4    0  -1 |    <- 0 zeros  ← viola a condição 2
| 0   0   0  -13   6 |    <- 3 zeros
```

A segunda linha começa com `3 ≠ 0`, igual à primeira — a contagem não cresce. É preciso
zerar essa posição:

```
L2 ← 4·L2 - 3·L1

  4(3) - 3(4)   = 12 - 12 =  0
  4(0) - 3(-7)  =  0 + 21 = 21
  4(4) - 3(0)   = 16 -  0 = 16
  4(0) - 3(-14) =  0 + 42 = 42
  4(-1) - 3(4)  = -4 - 12 = -16
```

```
| 4  -7    0  -14    4 |    <- 0 zeros
| 0  21   16   42  -16 |    <- 1 zero
| 0   0    0  -13    6 |    <- 3 zeros
```

Contagem `0 → 1 → 3`: estritamente crescente ✓. **Escalonada.**

> Note que a contagem **não precisa** subir de 1 em 1 — basta que **aumente**. Pular de 1
> para 3 é perfeitamente válido.

**E13.** *Resolva `x + 2y = 14`, `x − 2y = 6`.*

```
somando as duas:     2x = 20   =>   x = 10
subtraindo:          4y =  8   =>   y =  2
```

**Solução: `(x, y) = (10, 2)`.** **SPD.**

**Conferência:** `10 + 2(2) = 14` ✓ e `10 − 2(2) = 6` ✓

**E14.** *Resolva o sistema 3×3.*

```
| 2x₁ + 1x₂ + 3x₃ =   8 |
| 4x₁ + 2x₂ + 2x₃ =   4 |
| 2x₁ + 5x₂ + 3x₃ = -12 |
```

Escalonando a matriz ampliada:

```
| 2  1  3 |   8 |
| 4  2  2 |   4 |
| 2  5  3 | -12 |

L2 ← L2 - 2L1        L3 ← L3 - L1

| 2  1   3 |   8 |
| 0  0  -4 | -12 |
| 0  4   0 | -20 |

L2 ↔ L3   (para respeitar a forma escalonada)

| 2  1   3 |   8 |
| 0  4   0 | -20 |
| 0  0  -4 | -12 |
```

Substituição retroativa:

```
-4x₃ = -12   =>   x₃ =  3
 4x₂ = -20   =>   x₂ = -5
 2x₁ + (-5) + 3(3) = 8  =>  2x₁ + 4 = 8  =>  x₁ = 2
```

**Solução: `(x₁, x₂, x₃) = (2, −5, 3)`** ✓ (confere com o slide). **SPD.**

**E15.** *Resolva o sistema (solução: SI).*

```
|  x + 2y - 3z = -1 |
| -3x +  y - 2z = -7 |
|  5x + 3y - 4z =  2 |
```

```
L2 ← L2 + 3L1:   ( 0)x + (1+6)y + (-2-9)z = -7 + 3(-1)
                        7y - 11z = -10

L3 ← L3 - 5L1:   ( 0)x + (3-10)y + (-4+15)z = 2 - 5(-1)
                       -7y + 11z = 7
```

Somando as duas equações resultantes:

```
(7y - 7y) + (-11z + 11z) = -10 + 7
                       0 = -3        ← ABSURDO
```

Uma linha `0 0 0 | −3` na matriz escalonada. **Sistema Impossível (SI)** ✓

**E16.** *Resolva o sistema com 2 equações e 4 incógnitas.*

```
| 2x₁ -  8x₂ + 24x₃ + 18x₄ =  84 |
| 4x₁ + 14x₂ + 52x₃ + 42x₄ = 190 |
```

**Antes de calcular, classifique:** 2 equações, 4 incógnitas. Com menos equações que
incógnitas, **nunca** pode ser SPD. Será **SPI** (ou SI, se aparecer contradição).

```
L1 ← L1 / 2:     x₁ - 4x₂ + 12x₃ +  9x₄ = 42

L2 ← L2 - 4·L1:  (4-4)x₁ + (14+16)x₂ + (52-48)x₃ + (42-36)x₄ = 190 - 168
                        30x₂ + 4x₃ + 6x₄ = 22
L2 ← L2 / 2:            15x₂ + 2x₃ + 3x₄ = 11
```

Duas equações independentes, quatro incógnitas ⟹ **SPI com 2 graus de liberdade**. Tomando
`x₃ = s` e `x₄ = t` como parâmetros livres:

```
          11 - 2s - 3t
x₂  =  ------------------
              15

x₁  =  42 + 4x₂ - 12s - 9t

        674 - 188s - 147t
    =  -------------------
               15
```

**Solução geral:**

```
        | 674 - 188s - 147t      11 - 2s - 3t          |
(X) =   | ------------------ ,  --------------- ,  s , t |,   s, t ∈ R
        |         15                   15              |
```

**Conferência com `s = t = 0`** (`x₁ = 674/15`, `x₂ = 11/15`):

```
eq1:  2(674/15) - 8(11/15) = 1348/15 - 88/15 = 1260/15 =  84   ✓
eq2:  4(674/15) + 14(11/15) = 2696/15 + 154/15 = 2850/15 = 190  ✓
```

---

## 15. Simulado (20 questões)

**1.** A matriz `A` tem ordem `3 × 5`. Quantos elementos ela tem, e qual a ordem de `Aᵀ`?
a) 8 elementos; `Aᵀ` é `3×5`
b) 15 elementos; `Aᵀ` é `5×3`
c) 15 elementos; `Aᵀ` é `3×5`
d) 35 elementos; `Aᵀ` é `5×3`

**2.** Numa matriz **antissimétrica**, os elementos da diagonal principal:
a) são todos iguais a 1
b) são todos iguais a zero
c) podem assumir qualquer valor
d) são iguais aos da diagonal secundária

**3.** `(AB)ᵀ` é igual a:
a) `AᵀBᵀ`
b) `BᵀAᵀ`
c) `AB`
d) `(BA)ᵀ`

**4.** `A` é `3×4` e `B` é `4×2`. Sobre `AB` e `BA`:
a) `AB` é `3×2` e `BA` é `2×3`
b) `AB` é `3×2` e `BA` não existe
c) ambos são `3×2`
d) nenhum dos dois existe

**5.** Se `A` é uma matriz `n × n` e `k` um escalar, então `det(kA)` é igual a:
a) `k · det(A)`
b) `kⁿ · det(A)`
c) `det(A)`
d) `k + det(A)`

**6.** Somar a uma linha de `A` um múltiplo de outra linha:
a) multiplica o determinante por esse múltiplo
b) troca o sinal do determinante
c) **não altera** o determinante
d) zera o determinante

**7.** `A` é quadrada com `det(A) = 0`. Então:
a) `A` tem inversa e o sistema `AX = B` é SPD
b) `A` não tem inversa, e `AX = B` não pode ser SPD
c) `A` é necessariamente a matriz nula
d) `A` é necessariamente triangular

**8.** A Regra de Sarrus vale para matrizes de ordem:
a) qualquer
b) 2 e 3
c) **apenas 3**
d) 3 e 4

**9.** `det(A) = 5` e `det(B) = −3`. Então `det(AB)` vale:
a) 2
b) −15
c) 15
d) não é possível determinar

**10.** No desenvolvimento de Laplace, a escolha mais eficiente é a linha ou coluna:
a) com mais zeros
b) com os maiores valores
c) sempre a primeira linha
d) sempre a diagonal principal

**11.** No método `A⁻¹ = (1/det A)·adj(A)`, a adjunta é:
a) a matriz dos cofatores
b) a **transposta** da matriz dos cofatores
c) a matriz dos menores complementares
d) a transposta de `A`

**12.** `A² = 0` (matriz nula), com `A` não nula. Isso:
a) é impossível
b) só ocorre se `A` for a identidade
c) é possível — `A` é nihilpotente de índice 2
d) implica que `A` é idempotente

**13.** Se `A` é idempotente (`A² = A`), então `A¹⁰⁰` vale:
a) `A`
b) `0`
c) `I`
d) `100A`

**14.** A equação `xy + y = 2` é:
a) linear, com coeficientes `x` e 1
b) **não linear**, por causa do produto de incógnitas
c) linear homogênea
d) linear com termo independente 2

**15.** Um sistema linear **homogêneo** nunca pode ser:
a) SPD
b) SPI
c) **SI**
d) resolvido por escalonamento

**16.** Ao escalonar, surge a linha `0 0 0 | 7`. O sistema é:
a) SPD
b) SPI
c) SI
d) homogêneo

**17.** Um sistema tem 3 equações e 5 incógnitas. Ele **nunca** poderá ser:
a) SI
b) SPI
c) SPD
d) homogêneo

**18.** Duas retas coincidentes representam um sistema:
a) SPD
b) SPI
c) SI
d) não linear

**19.** `det [[1,2,3],[2,5,6],[2,5,8]] = 2`. Qual é `det [[2,4,6],[2,5,6],[2,5,8]]`?
a) 2
b) 4
c) 8
d) 16

**20.** `M` é ortogonal quando:
a) `M = Mᵀ`
b) `M⁻¹ = Mᵀ`
c) `det(M) = 0`
d) `M` é triangular

### 15.1 Gabarito

| Q | Resp. | Por quê |
| --- | --- | --- |
| 1 | **b** | `3 × 5 = 15` elementos; transpor **inverte a ordem** → `5×3` |
| 2 | **b** | `aᵢᵢ = −aᵢᵢ` ⟹ `2aᵢᵢ = 0` ⟹ `aᵢᵢ = 0`. Consequência da definição |
| 3 | **b** | A ordem **inverte**. Mesma coisa em `(AB)⁻¹ = B⁻¹A⁻¹` |
| 4 | **b** | `AB`: `3×4 · 4×2` ✓ → `3×2`. `BA`: `4×2 · 3×4` — 2 ≠ 3, **não existe** |
| 5 | **b** | Multiplicar a matriz inteira multiplica **cada uma das n linhas**: `kⁿ` |
| 6 | **c** | Propriedade 8 — é o que torna a triangulação "de graça" |
| 7 | **b** | Propriedade 11. Sem inversa não há `X = A⁻¹B`, logo não há solução única |
| 8 | **c** | Sarrus é **exclusivo da ordem 3**. Para `n ≥ 4`: Laplace ou triangulação |
| 9 | **b** | Binet: `det(AB) = 5 · (−3) = −15` |
| 10 | **a** | Cada zero anula um cofator inteiro, que nem precisa ser calculado |
| 11 | **b** | `adj(A) = Cᵀ`. Esquecer de transpor é o erro clássico do método |
| 12 | **c** | Nihilpotente de índice 2 — ver E16. Contraexemplo essencial |
| 13 | **a** | `A² = A` ⟹ `Aⁿ = A` para todo `n`. Período 1 |
| 14 | **b** | O produto `xy` entre duas incógnitas quebra a linearidade |
| 15 | **c** | Sempre admite a solução trivial ⟹ **sempre possível** (SPD ou SPI) |
| 16 | **c** | `0 = 7` é absurdo ⟹ **SI** |
| 17 | **c** | Menos equações que incógnitas ⟹ nunca há informação para fixar solução única |
| 18 | **b** | Uma reta sobre a outra: infinitos pontos comuns ⟹ **SPI** |
| 19 | **b** | Só a **1ª linha** virou `2·(1,2,3)`; as outras duas não mudaram. Um fator 2 sai: `det = 2 × 2 = 4` |
| 20 | **b** | `M⁻¹ = Mᵀ`, isto é `M·Mᵀ = I`; equivale a linhas ortonormais |

> **Por que a 19 não é `2⁴` nem `2³`:** a propriedade `det(kA) = kⁿ·det(A)` vale quando a
> **matriz inteira** é multiplicada. Aqui apenas **uma fila** foi, então sai um único fator.
> Compare com o E9 da seção 14, onde as **três** linhas foram multiplicadas (por 2, 7 e −1) e
> os três fatores saíram: `2·7·(−1)·2 = −28`.

---

## Checklist de véspera

**Matrizes**

- [ ] Leio `aᵢⱼ` como **linha i, coluna j** sem hesitar
- [ ] Sei os 11 tipos, e que a antissimétrica tem **diagonal nula por definição**
- [ ] `Aᵀ`: linhas viram colunas, e a **ordem inverte**
- [ ] Sei que soma exige **mesma ordem**, e produto exige **compatibilidade**
- [ ] **`(AB)ᵀ = BᵀAᵀ`** e **`(AB)⁻¹ = B⁻¹A⁻¹`** — a ordem inverte nos dois
- [ ] `AB ≠ BA`, e `AB` pode existir sem `BA` existir
- [ ] Sei montar `[A | I] → [I | A⁻¹]`
- [ ] Sei que `A² = 0` **não** implica `A = 0` (nihilpotente)

**Determinantes**

- [ ] Só para matriz **quadrada**
- [ ] **Sarrus só para ordem 3**
- [ ] Triangular ⟹ produto da diagonal
- [ ] As três operações: permutar (**troca sinal**), × β (**× β**), somar múltiplo (**nada**)
- [ ] `det(kA) = kⁿ·det(A)`, e não `k·det(A)`
- [ ] Binet: `det(AB) = det A · det B`
- [ ] **`det(A) ≠ 0` ⟺ existe `A⁻¹`**
- [ ] Em Laplace, escolho a fila com mais zeros
- [ ] Na inversa por cofator, **não esqueço de transpor**

**Sistemas**

- [ ] Sei identificar linear x não linear (produto de incógnitas, potência)
- [ ] Sei escrever na forma `A·X = B` e montar a matriz ampliada
- [ ] Reconheço SPD / SPI / SI pela matriz escalonada
- [ ] `0 = k (k≠0)` ⟹ **SI**; linha nula ⟹ **SPI**
- [ ] **Homogêneo nunca é SI**
- [ ] Menos equações que incógnitas ⟹ **nunca SPD**
- [ ] Verifico solução em **todas** as equações, não em duas de três
