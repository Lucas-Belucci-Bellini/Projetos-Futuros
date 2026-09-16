# Sistemas Digitais — Caderno de Estudo nº 2

> Baseado na **Lista de Exercícios — Sistemas Digitais 1** (9 páginas,
> questões 2 a 20). A questão 1 não consta do PDF original (o gabarito da lista
> marca `X` para ela).
>
> **Verificação:** resolvi as 19 questões do zero e comparei com o gabarito da
> própria lista. **As 19 batem.** Pode estudar por ele com confiança — o que
> este caderno acrescenta são os mapas desenhados, o porquê de cada distrator
> e a teoria que as questões cobram sem dizer.

**Como usar:** faça a **Parte B** de caneta, sem olhar nada. Só depois abra a
**Parte C**. Corrigir antes de tentar não treina nada.

---

# PARTE A — Revisão

## A.1 As leis, organizadas por utilidade

| Lei | Forma | Serve para |
|---|---|---|
| Identidade | `A + 0 = A` · `A · 1 = A` | Limpar termos neutros |
| Elemento nulo | `A + 1 = 1` · `A · 0 = 0` | Colapsar expressão inteira |
| Idempotência | `A + A = A` · `A · A = A` | Eliminar repetição |
| Complemento | `A + A' = 1` · `A · A' = 0` | **O motor de quase toda simplificação** |
| Involução | `(A')' = A` | Desfazer negação dupla |
| Absorção | `A + A·B = A` · `A·(A + B) = A` | Engolir termo redundante |
| **Absorção com complemento** | **`A + A'·B = A + B`** | ⚠️ A que mais confunde |
| Adjacência | `A·B + A·B' = A` | Base algébrica do mapa de Karnaugh |
| Distributiva (produto) | `A·(B + C) = A·B + A·C` | Expandir |
| **Distributiva (soma)** | **`(A + B)·(A + C) = A + B·C`** | ⚠️ Não existe em álgebra comum |

## A.2 As quatro que esta lista cobra de verdade

### 1. Absorção aplicada mais de uma vez (questão 2)

```
  A + A·B' + A·B
  = A · (1 + B' + B)     ← fatora A
  = A · 1                ← 1 + qualquer coisa = 1
  = A
```

**A chave:** assim que aparece um `1` numa soma, **tudo vira 1**. Você nem
precisa saber quanto vale `B' + B`.

### 2. Distributiva da soma sobre o produto (questão 3)

```
  (A + B) · (A + C) = A + B·C
```

Em álgebra comum isso é falso — `(2+3)(2+4) ≠ 2 + 3·4`. Na booleana **vale**.
Se não confiar, expanda:

```
  (A+B)(A+C) = A·A + A·C + A·B + B·C
             = A + A·C + A·B + B·C      ← A·A = A (idempotência)
             = A·(1 + C + B) + B·C
             = A + B·C
```

### 3. Adjacência — a ponte entre álgebra e Karnaugh (questão 5)

```
  A'·B + A·B = B·(A' + A) = B·1 = B
```

**Isto é exatamente o que o mapa de Karnaugh faz graficamente.** Duas células
vizinhas que diferem em uma variável → essa variável some. Quando você agrupa
no mapa, está aplicando esta lei sem escrever.

### 4. Absorção com complemento (questão 7) — a pegadinha

```
  B + B'·C = B + C
```

Parece errado. Não é. Prova pela distributiva da soma:

```
  B + B'·C = (B + B')·(B + C) = 1·(B + C) = B + C
```

> **Como lembrar:** o `B'` "não consegue" segurar o `C`, porque quando `B=1` a
> soma já é 1 de qualquer jeito, e quando `B=0` sobra `C`. Resultado: `B + C`.

## A.3 DeMorgan — o que esta lista cobra

```
  (A · B)' = A' + B'        (A + B)' = A' · B'
```

### Três passos, sempre os mesmos

1. Remova a negação externa.
2. Troque o operador (`·` ↔ `+`).
3. Negue **cada** termo interno.

### Quando a variável já vem complementada (questões 9 e 12)

O passo 3 vale para todos, inclusive os que já têm linha — e aí a negação
dupla cancela:

```
  (A' · B · C')'
  = (A')' + B' + (C')'      ← troca · por + e nega cada um
  = A + B' + C              ← (A')' = A  e  (C')' = C
```

**Regra prática:** toda variável **inverte de estado**. Quem tinha linha perde,
quem não tinha ganha.

### Negação de produto de somas (questão 10)

Duas aplicações, de fora para dentro:

```
  ((A + B) · (A + C))'
  = (A + B)' + (A + C)'      ← DeMorgan na camada externa
  = A'·B' + A'·C'            ← DeMorgan em cada parêntese
  = A'·(B' + C')             ← fatora A'
```

### Negações sucessivas (questão 13)

Cada par se cancela. Conte quantas são:

| Nº de negações | Equivale a |
|---|---|
| Par (2, 4, 6…) | A expressão **original** |
| Ímpar (1, 3, 5…) | **Uma** negação da original |

```
  (((A + B)')')'   →  3 negações (ímpar)  →  (A + B)'  =  A'·B'
```

## A.4 Mapas de Karnaugh — o que esta lista cobra

### A ordem Gray (não negociável)

```
                 BC=00   BC=01   BC=11   BC=10
          A=0  │   m0      m1      m3      m2
          A=1  │   m4      m5      m7      m6
```

Colunas em `00, 01, 11, 10`. **Nunca** em ordem binária.

Para 4 variáveis, linhas e colunas em Gray:

```
                 CD=00   CD=01   CD=11   CD=10
         AB=00 │   m0      m1      m3      m2
         AB=01 │   m4      m5      m7      m6
         AB=11 │  m12     m13     m15     m14
         AB=10 │   m8      m9     m11     m10
```

### Tamanho do grupo × variáveis eliminadas

| Grupo | Elimina | Num mapa de 3 var | Num mapa de 4 var |
|---:|---:|---|---|
| 1 célula | 0 | termo com 3 variáveis | termo com 4 variáveis |
| 2 células | 1 | termo com 2 | termo com 3 |
| 4 células | 2 | termo com 1 | termo com 2 |
| 8 células | 3 | **a função é 1** | termo com 1 |
| 16 células | 4 | — | **a função é 1** |

**Sempre procure o maior grupo primeiro.** Grupo maior = termo menor = menos
portas lógicas.

### Os quatro casos especiais desta lista

| Caso | Questão | O que acontece |
|---|:---:|---|
| Linha inteira preenchida | 15 | Grupo de 4 num mapa de 3 var → sobra 1 variável (`A'`) |
| **1's isolados** | **16** | **Nenhum agrupamento possível — a expressão não simplifica** |
| Função depende de só 2 das 4 variáveis | 18 | Duas variáveis somem inteiras |
| Grupo de 8 | 20 | Metade do mapa de 4 var → sobra 1 variável |

> **Questão 16 é a mais importante de entender.** Nem toda função simplifica.
> Quando os 1's estão isolados (cada um difere dos outros em **duas ou mais**
> variáveis), não existe par adjacente, e a resposta é a soma dos mintermos
> originais. Quem "força" um agrupamento erra.

### Caminho inverso: da expressão para o mapa (questão 19)

Cada termo produto de 3 variáveis ocupa **uma célula**. Marque todas, depois
agrupe normalmente:

```
  A'·B'·C'  →  ABC = 000  →  m0
  A'·B ·C'  →  ABC = 010  →  m2
  A ·B'·C   →  ABC = 101  →  m5
  A ·B ·C   →  ABC = 111  →  m7
```

---

# PARTE B — A lista, para resolver

> Faça sem consultar. Anote a alternativa na folha de respostas do fim da
> Parte B. Tempo sugerido: **45 minutos** para as 19.

## Parte I — Álgebra Booleana

**2.** Simplifique: `A + A·B' + A·B`
 (A) `A·B`  (B) `A`  (C) `A+B`  (D) `B`  (E) `A·B'`

**3.** Forma equivalente de: `(A + B)·(A + C)`
 (A) `A·B+C`  (B) `A+B+C`  (C) `A·B·C`  (D) `A + B·C`  (E) `A·(B+C)`

**4.** Simplifique: `A·(B + B')·C`
 (A) `A·C`  (B) `A+C`  (C) `A·B·C`  (D) `A`  (E) `C`

**5.** Forma minimizada de: `A'·B + A·B`
 (A) `A`  (B) `A+B`  (C) `A·B`  (D) `A'+B`  (E) `B`

**6.** Simplifique: `(A + A·B)·(B + A·B)`
 (A) `A+B`  (B) `A`  (C) `A·B`  (D) `B`  (E) `A'·B'`

**7.** Simplifique: `A·B·C + A·B·C' + A·B'·C`
 (A) `B·C + A·C`  (B) `A·B + A·C`  (C) `A·B·C`  (D) `A + B·C`  (E) `A·(B·C)`

## Parte II — Teorema de DeMorgan

**8.** Forma equivalente de: `(A + B')'`
 (A) `A·B'`  (B) `A'+B`  (C) `A'·B'`  (D) `A'·B`  (E) `A+B'`

**9.** Forma equivalente de: `(A' · B · C')'`
 (A) `A+B'+C`  (B) `A'+B+C'`  (C) `A·B'·C`  (D) `A'+B+C`  (E) `A+B+C`

**10.** Forma equivalente de: `((A + B)·(A + C))'`
 (A) `A'+(B'·C')`  (B) `A·(B+C)`  (C) `A'·B'·C'`  (D) `(A·B)'+(A·C)'`  (E) `A'·(B'+C')`

**11.** Forma equivalente de: `((A·B) + C)'`
 (A) `A'·B'·C'`  (B) `(A'·B')+C'`  (C) `(A'+B')·C'`  (D) `A'+B'+C'`  (E) `(A+B)·C'`

**12.** Forma equivalente de: `(A' + B + C')'`
 (A) `A'·B·C'`  (B) `A·B'·C`  (C) `A+B'+C`  (D) `A'·B'·C'`  (E) `A·B·C`

**13.** Forma equivalente de: `(((A + B)')')'`
 (A) `A'·B'`  (B) `A+B`  (C) `A·B`  (D) `A'+B'`  (E) `(A·B)'`

## Parte III — Mapas de Karnaugh

**14.** Minimize em SoP a função da tabela:

| A | B | F |
|:-:|:-:|:-:|
| 0 | 0 | 1 |
| 0 | 1 | 0 |
| 1 | 0 | 1 |
| 1 | 1 | 1 |

 (A) `A'+B`  (B) `A·B'`  (C) `A+B`  (D) `A+B'`  (E) `A'·B' + A·B`

**15.** Minimize `F(A,B,C)`:

| A | B | C | F | | A | B | C | F |
|:-:|:-:|:-:|:-:|---|:-:|:-:|:-:|:-:|
| 0 | 0 | 0 | 1 | | 1 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 | | 1 | 0 | 1 | 0 |
| 0 | 1 | 0 | 1 | | 1 | 1 | 0 | 0 |
| 0 | 1 | 1 | 1 | | 1 | 1 | 1 | 1 |

 (A) `A'+A·C`  (B) `A'·B+C`  (C) `A+B·C`  (D) `A'·C+B·C`  (E) `A'+B·C`

**16.** `F(A,B,C)` vale 1 quando **exatamente uma** das três variáveis for 1.
 (A) `A·B+A·C+B·C`  (B) `A'·B'+B'·C'`  (C) `A'·B'·C + A'·B·C' + A·B'·C'`
 (D) `A+B+C`  (E) `A'·B'·C'`

**17.** Minimize `F(A,B,C,D)` em SoP:

| A | B | C | D | F | | A | B | C | D | F |
|:-:|:-:|:-:|:-:|:-:|---|:-:|:-:|:-:|:-:|:-:|
| 0 | 0 | 0 | 0 | 0 | | 1 | 0 | 0 | 0 | 0 |
| 0 | 0 | 0 | 1 | 0 | | 1 | 0 | 0 | 1 | 0 |
| 0 | 0 | 1 | 0 | 1 | | 1 | 0 | 1 | 0 | 1 |
| 0 | 0 | 1 | 1 | 0 | | 1 | 0 | 1 | 1 | 0 |
| 0 | 1 | 0 | 0 | 1 | | 1 | 1 | 0 | 0 | 0 |
| 0 | 1 | 0 | 1 | 1 | | 1 | 1 | 0 | 1 | 0 |
| 0 | 1 | 1 | 0 | 1 | | 1 | 1 | 1 | 0 | 1 |
| 0 | 1 | 1 | 1 | 1 | | 1 | 1 | 1 | 1 | 0 |

 (A) `C·D'+A'·B`  (B) `C'·D+A'·B`  (C) `C·D'+A·B'`  (D) `C·D+A'·B`  (E) `A'·C+B·D'`

**18.** Minimize `F(A,B,C,D)` em SoP:

| A | B | C | D | F | | A | B | C | D | F |
|:-:|:-:|:-:|:-:|:-:|---|:-:|:-:|:-:|:-:|:-:|
| 0 | 0 | 0 | 0 | 1 | | 1 | 0 | 0 | 0 | 1 |
| 0 | 0 | 0 | 1 | 0 | | 1 | 0 | 0 | 1 | 0 |
| 0 | 0 | 1 | 0 | 1 | | 1 | 0 | 1 | 0 | 1 |
| 0 | 0 | 1 | 1 | 0 | | 1 | 0 | 1 | 1 | 0 |
| 0 | 1 | 0 | 0 | 0 | | 1 | 1 | 0 | 0 | 0 |
| 0 | 1 | 0 | 1 | 1 | | 1 | 1 | 0 | 1 | 1 |
| 0 | 1 | 1 | 0 | 0 | | 1 | 1 | 1 | 0 | 0 |
| 0 | 1 | 1 | 1 | 1 | | 1 | 1 | 1 | 1 | 1 |

 (A) `B·D'+B'·D`  (B) `B'·C'+B·C`  (C) `A·D+A'·D'`  (D) `B'·D'+A·D`  (E) `B·D+B'·D'`

**19.** Transfira para o mapa e simplifique:
`A'·B'·C' + A'·B·C' + A·B'·C + A·B·C`
 (A) `A·C'+A'·C`  (B) `A'·C'+A·C`  (C) `C'+A·C`  (D) `A'+C`  (E) `A'·C'+B·C`

**20.** `F(A,B,C,D)` vale 1 sempre que `A = 0`, e também quando as quatro
variáveis forem simultaneamente 1.
 (A) `A'·B·C·D`  (B) `A'+C·D`  (C) `A+B·C·D`  (D) `A'+B·C·D`  (E) `A'·C·D+B`

## Folha de respostas

```
   2 ___    3 ___    4 ___    5 ___    6 ___    7 ___

   8 ___    9 ___   10 ___   11 ___   12 ___   13 ___

  14 ___   15 ___   16 ___   17 ___   18 ___   19 ___   20 ___


  Acertos: ____ / 19
```

---

# PARTE C — Gabarito comentado

## Parte I — Álgebra Booleana

### 2 — **(B)** `A`

```
  A + A·B' + A·B
  = A·(1 + B' + B)     ← fatora A
  = A·1                ← elemento nulo: 1 + qualquer coisa = 1
  = A
```

### 3 — **(D)** `A + B·C`

Distributiva da soma sobre o produto, aplicada direto.
Se preferir expandir: `A·A + A·C + A·B + B·C` → `A(1 + C + B) + B·C` → `A + B·C`.

> **Distrator (A) `A·B + C`** troca as posições de propósito. Confira sempre em
> `A=1, B=0, C=0`: a expressão original dá `1·1 = 1`; `A·B+C` dá `0`. Elimina.

### 4 — **(A)** `A·C`

```
  A·(B + B')·C
  = A·1·C              ← B + B' = 1
  = A·C
```

> `B` é isca: some inteiro porque aparece com o próprio complemento.

### 5 — **(E)** `B`

```
  A'·B + A·B = B·(A' + A) = B·1 = B
```

> Esta é a **lei da adjacência** — exatamente o que o mapa de Karnaugh faz ao
> agrupar duas células vizinhas.

### 6 — **(C)** `A·B`

Simplifique **cada fator** antes de multiplicar:

```
  A + A·B = A          ← absorção
  B + A·B = B          ← absorção
  ────────────────
  (A)·(B) = A·B
```

> Erro comum: expandir tudo primeiro. Dá no mesmo, mas com quatro vezes mais
> trabalho e muito mais chance de errar sinal.

### 7 — **(B)** `A·B + A·C`

Dois passos. Agrupe primeiro os dois termos que diferem em **uma** variável:

```
  A·B·C + A·B·C' = A·B·(C + C') = A·B
```

Agora junte com o que sobrou:

```
  A·B + A·B'·C
  = A·(B + B'·C)
  = A·(B + C)          ← absorção com complemento: B + B'·C = B + C
  = A·B + A·C
```

> **O passo crítico é `B + B'·C = B + C`.** Se você não conhecer essa
> identidade, trava aqui. Ela está explicada na [Parte A.2](#4-absorção-com-complemento-questão-7).

---

## Parte II — DeMorgan

### 8 — **(D)** `A' · B`

```
  (A + B)' = A'·B'  →  (A + B')' = A'·(B')' = A'·B
```

> **Distrator (C) `A'·B'`** é a resposta de quem esqueceu que o `B` já vinha
> complementado. A negação dupla devolve `B`.

### 9 — **(A)** `A + B' + C`

```
  (A' · B · C')'
  = (A')' + B' + (C')'
  = A + B' + C
```

> **Toda variável inverte de estado.** `A'` vira `A`, `B` vira `B'`, `C'` vira `C`.

### 10 — **(E)** `A' · (B' + C')`

```
  ((A + B)·(A + C))'
  = (A + B)' + (A + C)'      ← DeMorgan externo: · vira +
  = A'·B' + A'·C'            ← DeMorgan em cada parêntese
  = A'·(B' + C')             ← fatora A'
```

> **Distrator (D) `(A·B)' + (A·C)'`** é a armadilha da questão: seria a negação
> de `(A·B)·(A·C)`, não de `(A+B)·(A+C)`. Parece DeMorgan e não é.

### 11 — **(C)** `(A' + B') · C'`

```
  ((A·B) + C)'
  = (A·B)' · C'        ← DeMorgan na soma externa: + vira ·
  = (A' + B') · C'     ← DeMorgan no produto interno
```

> **Distrator (D) `A'+B'+C'`** é o erro de aplicar a lei errada na camada
> externa (tratar a soma como produto).

### 12 — **(B)** `A · B' · C`

```
  (A' + B + C')'
  = (A')' · B' · (C')'
  = A · B' · C
```

### 13 — **(A)** `A' · B'`

Três negações — número **ímpar** — equivalem a uma só:

```
  (((A + B)')')'  =  (A + B)'  =  A'·B'
```

> Cancele de dentro para fora: as duas internas `((X)')'` viram `X`, sobra a
> externa.

---

## Parte III — Mapas de Karnaugh

### 14 — **(D)** `A + B'`

Uns em `(0,0)`, `(1,0)` e `(1,1)`:

```
          B=0   B=1
   A=0  │  1     0
   A=1  │  1     1
           │     │
           │     └─ linha A=1 inteira → grupo de 2 → A
           └─ coluna B=0 inteira → grupo de 2 → B'
```

**`F = A + B'`**

> Confira no único zero, `A=0, B=1`: `A + B'` = `0 + 0` = `0`. ✓

### 15 — **(E)** `A' + B·C`

Uns em m0, m1, m2, m3 e m7:

```
           BC=00   BC=01   BC=11   BC=10
   A=0  │    1       1       1       1     ← linha inteira → grupo de 4 → A'
   A=1  │    0       0       1       0
                             ↑
                       m7 sobrou: agrupa com m3 (acima)
                       A muda, B=1, C=1  →  B·C
```

**`F = A' + B·C`**

> **Distrator (A) `A' + A·C`**: teste em `A=1,B=0,C=1` (m5). A tabela diz **0**,
> mas `A·C` daria 1. Elimina.

### 16 — **(C)** `A'·B'·C + A'·B·C' + A·B'·C'`

"Exatamente uma variável igual a 1" → uns em **m1 (001)**, **m2 (010)** e
**m4 (100)**:

```
           BC=00   BC=01   BC=11   BC=10
   A=0  │    0       1       0       1
   A=1  │    1       0       0       0
```

Nenhum par é adjacente — cada 1 difere dos outros em **duas** variáveis.
**Não há agrupamento possível.**

**`F = A'B'C + A'BC' + AB'C'`**

> **Esta é a questão mais importante da lista.** Ela ensina que *nem toda
> função simplifica*. Quem chuta que sempre dá para reduzir, erra.
>
> **Distrator (A) `AB + AC + BC`** é a função **maioria** — vale 1 com **pelo
> menos duas** variáveis em 1. É exatamente o oposto desta. Não confunda as
> duas: "exatamente uma" e "pelo menos duas" são funções complementares no
> conjunto dos mintermos de peso ≥ 1.

### 17 — **(A)** `C·D' + A'·B`

Uns em m2, m4, m5, m6, m7, m10, m14:

```
            CD=00   CD=01   CD=11   CD=10
   AB=00  │   0       0       0       1    ← m2
   AB=01  │   1       1       1       1    ← m4,m5,m7,m6  ┐
   AB=11  │   0       0       0       1    ← m14          │ grupo de 4: A'B
   AB=10  │   0       0       0       1    ← m10          ┘
                                     └──┴── coluna CD=10 inteira
                                            grupo de 4: C·D'
```

- **Coluna `CD=10` inteira** (m2, m6, m14, m10) → `C·D'`
- **Linha `AB=01` inteira** (m4, m5, m7, m6) → `A'·B`
- As duas se sobrepõem em m6 — **sobreposição é permitida**.

**`F = C·D' + A'·B`**

### 18 — **(E)** `B·D + B'·D'`

Olhe o padrão sem desenhar o mapa:

| | `D=0` | `D=1` |
|---|:---:|:---:|
| **B=0** | **1** | 0 |
| **B=1** | 0 | **1** |

`A` e `C` variam nas 16 linhas e **não mudam nada**. A saída é 1 sempre que
`B` e `D` têm o mesmo valor.

**`F = B·D + B'·D'`**

> É a função **XNOR** entre B e D. Mesma estrutura da questão 13 da prova
> anterior (que era XNOR entre A e D). Reconhecer o padrão economiza o mapa
> inteiro de 4 variáveis.
>
> **Distrator (A) `B·D' + B'·D`** é o XOR — exatamente o **complemento** da
> resposta. Ler a tabela na diagonal errada leva direto nele.

### 19 — **(B)** `A'·C' + A·C`

Marque as quatro células:

```
  A'·B'·C'  →  000  →  m0
  A'·B ·C'  →  010  →  m2
  A ·B'·C   →  101  →  m5
  A ·B ·C   →  111  →  m7
```

```
           BC=00   BC=01   BC=11   BC=10
   A=0  │    1       0       0       1     ← m0 e m2: A=0, C=0, B muda → A'·C'
   A=1  │    0       1       1       0     ← m5 e m7: A=1, C=1, B muda → A·C
```

`B` varia dentro dos dois grupos, então some nos dois.

**`F = A'·C' + A·C`**

> Também é XNOR — agora entre `A` e `C`. Três questões da lista (18, 19 e a 13
> da prova anterior) são a mesma função com nomes diferentes.

### 20 — **(D)** `A' + B·C·D`

```
            CD=00   CD=01   CD=11   CD=10
   AB=00  │   1       1       1       1   ┐
   AB=01  │   1       1       1       1   ┘ metade do mapa (A=0) → grupo de 8 → A'
   AB=11  │   0       0       1       0   ← m15 (1111)
   AB=10  │   0       0       0       0
```

- **Grupo de 8** — as duas linhas com `A=0` → elimina B, C e D → **`A'`**
- **m15 sozinho?** Não: agrupa com **m7 (0111)**, que já está marcado.
  `A` muda, `B=C=D=1` → **`B·C·D`**

**`F = A' + B·C·D`**

> **Reutilizar célula já coberta é permitido e desejável** — foi o que
> transformou um termo de 4 variáveis (`A·B·C·D`) em um de 3 (`B·C·D`).
>
> **Distrator (A) `A'·B·C·D`** é o erro de trocar o `+` por `·`. Teste em
> `A=0,B=0,C=0,D=0`: a função vale **1** (porque A=0), mas `A'·B·C·D` daria 0.

---

# Gabarito rápido

| 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| **B** | **D** | **A** | **E** | **C** | **B** | **D** | **A** | **E** | **C** |

| 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| **B** | **A** | **D** | **E** | **C** | **A** | **E** | **B** | **D** |

**Distribuição:** A ×4 · B ×5 · C ×3 · D ×4 · E ×3

> ✅ **Confere com o gabarito da lista original nas 19 questões.**

---

# PARTE D — Treino extra

Questões novas, no mesmo padrão. Resposta no fim.

**E1.** Simplifique: `A·B + A·B' + A'·B`
**E2.** Simplifique: `(A + B)·(A + B')·(A' + B)`
**E3.** Aplique DeMorgan: `((A·B')' + C')'`
**E4.** Quantas negações tem `((((A·B)')')')'` e a que equivale?
**E5.** `F(A,B,C)` vale 1 quando **exatamente duas** variáveis forem 1.
  Escreva a forma minimizada.
**E6.** `F(A,B,C,D)` vale 1 quando `C = 1` **ou** quando `A = B = 1`.
  Minimize.
**E7.** Simplifique: `A'·B'·C + A'·B·C + A·B'·C + A·B·C`

### Respostas

| | Resultado | Caminho |
|:-:|---|---|
| **E1** | `A + B` | `A(B+B') + A'B = A + A'B = A + B` (absorção com complemento) |
| **E2** | `A·B` | `(A+B)(A+B') = A`; depois `A·(A'+B) = A·A' + A·B = A·B` |
| **E3** | `A·B' · C` | Externo: `((A·B')')' · (C')'` = `A·B' · C` |
| **E4** | 4 negações — **par** → volta à original: `A·B` | Cada par cancela |
| **E5** | `A·B·C' + A·B'·C + A'·B·C` | m3, m5, m6 — **isolados**, não simplifica (igual à questão 16) |
| **E6** | `C + A·B` | Metade do mapa (`C=1`) → grupo de 8 → `C`; sobram m12 e m14 → `A·B·D'`… que se junta a `C` → `A·B` |
| **E7** | `C` | Os quatro mintermos são todos com `C=1` → grupo de 4 → `C` |

---

# Erros que custam ponto nesta lista

1. **Achar que toda função simplifica** (questão 16). 1's isolados ficam como
   estão.
2. **Esquecer `B + B'·C = B + C`** (questão 7). Sem ela, a questão trava.
3. **Confundir XOR com XNOR** (questão 18). `B·D + B'·D'` é *iguais*;
   `B·D' + B'·D` é *diferentes*.
4. **Montar o mapa em ordem binária** em vez de Gray.
5. **Não reutilizar célula já agrupada** (questão 20). Sobreposição é permitida
   e gera termos menores.
6. **Aplicar DeMorgan na camada errada** (questões 10 e 11). Sempre de fora
   para dentro.
7. **Esquecer a negação dupla** em variável já complementada (questões 8, 9, 12).

## O teste que salva em 10 segundos

Está em dúvida entre duas alternativas? **Escolha uma linha da tabela em que
`F = 0`** e substitua. A alternativa errada quase sempre devolve 1 ali. Foi o
que eliminou o distrator nas questões 3, 15 e 20 deste gabarito.

---

## Material relacionado

- [`REVISAO-SISTEMAS-DIGITAIS.md`](REVISAO-SISTEMAS-DIGITAIS.md) — a prova
  anterior (15 questões), com a mesma estrutura de três blocos. As duas se
  complementam: a prova cobra mais álgebra pura, esta lista cobra mais
  Karnaugh de 4 variáveis.
