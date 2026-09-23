# Lista de Exercícios — Álgebra Linear

**Disciplina:** Álgebra Linear — UNIFIL, Departamento de Computação
**40 exercícios novos**, com cálculo passo a passo.

> **Estes exercícios não são os dos slides.** Os dos slides já estão resolvidos em
> [`REVISAO-ALGEBRA-LINEAR.md`](REVISAO-ALGEBRA-LINEAR.md). Estes aqui foram
> construídos do zero para exercitar cada técnica isoladamente e depois combinadas.

> **Todos os resultados foram conferidos por cálculo independente** — eliminação
> gaussiana em **frações exatas** (sem arredondamento) e, nos determinantes,
> também por expansão de Laplace. Se a sua conta bater com a daqui, bateu de
> verdade.

## Como usar

1. Resolva a **Parte 1** inteira sem olhar nada.
2. Só então abra a **Parte 2**. Cada resolução mostra **a conta inteira**, não só o
   resultado — inclusive as operações elementares linha a linha.
3. O que errar, volte à seção indicada da revisão (tabela no fim).

---

## Índice dos blocos

| Bloco | Exercícios | Assunto | Revisar em |
| --- | --- | --- | --- |
| A | 1–6 | Definição, tipos, igualdade | §1–§3 |
| B | 7–12 | Transposta, adição, escalar, produto | §4–§6 |
| C | 13–17 | Matriz inversa | §7 |
| D | 18–21 | Ortogonal, idempotente, nihilpotente | §8 |
| E | 22–29 | Determinantes | §9–§11 |
| F | 30–31 | Inversa pelo método dos cofatores | §11.3 |
| G | 32–40 | Sistemas lineares | §12 |

---

# PARTE 1 — A LISTA

## Bloco A — Matrizes: definição, tipos, igualdade

**1.** Escreva a matriz `A = (aᵢⱼ)₃ₓ₃` definida por `aᵢⱼ = 2i + j`.

**2.** Escreva `B = (bᵢⱼ)₃ₓ₃` com `bᵢⱼ = (−1)^(i+j) · i · j`. Depois de escrever,
**classifique** a matriz: ela é de algum tipo especial? Justifique.

**3.** Classifique cada matriz (pode ter mais de um tipo):

```
P = | 0  0 |     Q = | 5  0  0 |     R = | 1  7  -2 |
    | 0  0 |         | 0  5  0 |         | 0  3   4 |
                     | 0  0  5 |         | 0  0   9 |

S = |  0   3  -1 |   T = | 2  -6 |    U = [ 4  -1  0 ]
    | -3   0   5 |       | -6  1 |
    |  1  -5   0 |
```

**4.** Determine `x` e `y` para que `A` seja **simétrica**:

```
A = | 2   x+1   3 |
    | 5    0    y |
    | 3   -4    1 |
```

**5.** Determine `x` e `y` para que `B` seja **antissimétrica**:

```
B = | 0   x-2  -5 |
    | 3    0   y+1 |
    | 5   -7    0  |
```

**6.** Determine `a`, `b`, `c` e `d` sabendo que as matrizes são iguais:

```
| 3a-1   b²  |     | 8    9  |
|  2c    d+4 |  =  | -6   1  |
```

(Atenção: uma das incógnitas tem **duas** respostas possíveis.)

## Bloco B — Operações

**7.** Sejam

```
A = | 1  -2   3 |    B = | 2   1  -1 |    C = | -1   3   0 |
    | 0   4  -1 |        | 3   0   2 |        |  1  -2   4 |
```

Calcule `X = 2A − 3B + C`.

**8.** Com as matrizes do exercício 7, verifique numericamente que
`(A + B)ᵀ = Aᵀ + Bᵀ`.

**9.** Sejam

```
A = | 1   2 |        B = |  2   0   1 |
    | 3  -1 |            | -1   3   2 |
    | 0   4 |
```

  a) Existe `AB`? Qual a ordem? Calcule.
  b) Existe `BA`? Qual a ordem? Calcule.
  c) `AB = BA`? O que este exercício prova?

**10.** Com `A` e `B` do exercício 9, verifique que `(AB)ᵀ = Bᵀ Aᵀ`. Verifique
também que `Aᵀ Bᵀ` **nem sequer existe**, e explique por quê.

**11.** Para `A = [[2, 1], [−1, 3]]`, calcule `A²` e `A³`.

**12.** Escreva o sistema abaixo na forma matricial `A · X = B`, e depois efetue o
produto `A · X` para confirmar que ele reconstrói o lado esquerdo:

```
3x  -  y + 2z =  7
 x  + 4y -  z =  1
2x  + 5y      = -3
```

## Bloco C — Matriz inversa

**13.** Calcule a inversa de `A = [[3, 5], [1, 2]]` por operações elementares.
Confira multiplicando.

**14.** Calcule a inversa de

```
A = | 1   0   2 |
    | 2  -1   3 |
    | 4   1   8 |
```

por operações elementares, mostrando cada passo.

**15.** Mostre que a matriz abaixo **não** admite inversa, usando operações
elementares (não use determinante):

```
A = | 1  2  3 |
    | 2  4  6 |
    | 1  0  1 |
```

**16.** Para `A = [[2, 1], [3, 2]]` e `B = [[1, −1], [2, 1]]`, verifique
numericamente que `(AB)⁻¹ = B⁻¹A⁻¹`. Depois verifique se `(AB)⁻¹ = A⁻¹B⁻¹` também
vale.

**17.** Resolva o sistema abaixo usando `X = A⁻¹ · B`:

```
2x +  y =  8
5x + 3y = 21
```

## Bloco D — Matrizes especiais

**18.** Verifique se `Q = [[3/5, −4/5], [4/5, 3/5]]` é **ortogonal**. Calcule também
`det(Q)`.

**19.** Verifique se `A` é **idempotente**:

```
A = |  2  -2  -4 |
    | -1   3   4 |
    |  1  -2  -3 |
```

Se for, quanto vale `A¹⁰⁰`?

**20.** Verifique se `N` é **nihilpotente** e determine seu índice:

```
N = | 0  1  2 |
    | 0  0  3 |
    | 0  0  0 |
```

**21. Verdadeiro ou falso** (justifique cada um):

  a) Se `A² = 0`, então `A = 0`.
  b) Toda matriz simétrica é quadrada.
  c) Se `A` é ortogonal, então `det(A) = 1`.
  d) Se `A` é idempotente e inversível, então `A = I`.
  e) A diagonal principal de uma matriz antissimétrica pode ter elementos não nulos.

## Bloco E — Determinantes

**22.** Calcule: a) `det [[5, −3], [2, 4]]`  b) `det [[−1, −2], [−3, −4]]`
c) `det [[7, 0], [0, −2]]`

**23.** Calcule por Sarrus:

```
a) | 2  -1   3 |      b) | 1  1  1 |      c) |  3  0  -1 |
   | 0   4  -2 |         | 2  3  4 |         |  2  5   4 |
   | 1   5   1 |         | 5  6  7 |         | -1  2   3 |
```

**24.** **Sem fazer a conta completa**, diga o valor do determinante de cada matriz e
qual propriedade você usou:

```
a) | 2  4   6 |   b) | 1  5  2 |   c) | 1  0  3 |   d) | 3   8  -1 |
   | 1  3   5 |      | 4  0  3 |      | 4  0  7 |      | 0  -2   5 |
   | 3  7  11 |      | 1  5  2 |      | 2  0  5 |      | 0   0   4 |
```

**25.** Seja `A` uma matriz `3×3` com `det(A) = 9`. Quanto vale `det(2A)`?
Cuidado: a resposta **não** é 18.

**26.** Calcule por **triangulação**:

```
A = |  2   1  -1   3 |
    |  1   0   2  -1 |
    |  3   2   1   0 |
    | -1   1   4   2 |
```

**27.** Calcule por **Laplace**, escolhendo a fila mais econômica:

```
A = | 4   0   0   1 |
    | 2  -1   0   3 |
    | 0   5   2   0 |
    | 1   0   0  -2 |
```

**28.** Calcule por **Laplace**:

```
A = | 1  2  0  0  3 |
    | 0  0  5  0  0 |
    | 2  1  0  4  1 |
    | 0  3  0  1  2 |
    | 1  0  0  2  0 |
```

**29.** Para `A = [[3, 1], [2, 4]]` e `B = [[1, −2], [3, 1]]`:

  a) Calcule `det(A)`, `det(B)` e `det(AB)`. Confirme o Teorema de Binet.
  b) Calcule `A⁻¹` e `det(A⁻¹)`. Confirme que `det(A⁻¹) = 1/det(A)`.

## Bloco F — Inversa pelo método dos cofatores

**30.** Calcule a inversa de `A = [[4, 7], [2, 6]]` pelo método dos cofatores,
mostrando a matriz dos cofatores e a adjunta separadamente.

**31.** Calcule a inversa de

```
A = | 1  2  3 |
    | 0  1  4 |
    | 5  6  0 |
```

pelo método dos cofatores.

## Bloco G — Sistemas lineares

**32.** Classifique cada sistema em SPD, SPI ou SI, e resolva quando possível:

```
a)  x + 2y = 5        b)  2x + 4y = 6        c)   x - 3y = 2
   3x -  y = 1             x + 2y = 3           -2x + 6y = 1
```

**33.** Resolva por escalonamento:

```
 x +  y + z = 6
2x -  y + z = 3
 x + 2y - z = 2
```

**34.** Classifique e justifique:

```
 x + 2y -  z = 1
2x + 3y +  z = 4
3x + 5y       = 6
```

**35.** Classifique e, se possível, dê a **solução geral**:

```
  x + 2y -  z =  3
 2x + 4y - 2z =  6
 -x - 2y +  z = -3
```

**36.** Resolva o sistema homogêneo e dê a solução geral:

```
 x + 2y +  3z = 0
2x + 5y +  7z = 0
3x + 7y + 10z = 0
```

**37.** Para quais valores de `k` o sistema abaixo é SPD, SPI ou SI?

```
 x + 2y +  z = 3
2x + 5y + 3z = 8
 x + 3y + kz = 5
```

**38. Verdadeiro ou falso** (justifique):

  a) Um sistema homogêneo pode ser SI.
  b) Um sistema com 3 equações e 5 incógnitas pode ser SPD.
  c) Se `det(A) = 0`, o sistema `AX = B` é necessariamente SI.
  d) Um sistema com 4 equações e 2 incógnitas nunca pode ser SPD.
  e) Se `det(A) ≠ 0`, o sistema `AX = B` é SPD para qualquer `B`.

**39. Problema aplicado.** Uma loja monta três kits com as mesmas três peças —
SSD, pente de RAM e fonte:

| Kit | SSD | RAM | Fonte | Preço |
| --- | --- | --- | --- | --- |
| 1 | 1 | 2 | 1 | R$ 1.400 |
| 2 | 2 | 1 | 1 | R$ 1.500 |
| 3 | 1 | 1 | 2 | R$ 1.500 |

Determine o preço unitário de cada peça.

**40.** Seja `A` uma matriz `3×3` com `det(A) = −5`. Responda, justificando:

  a) `A` admite inversa?
  b) Quanto vale `det(A⁻¹)`?
  c) O sistema `AX = B` é SPD, SPI ou SI?
  d) E o sistema homogêneo `AX = 0`?

---
---

# PARTE 2 — RESOLUÇÕES

## Bloco A

### 1. `aᵢⱼ = 2i + j`

Calcule posição a posição — `i` é a linha, `j` é a coluna:

```
a11 = 2(1)+1 = 3    a12 = 2(1)+2 = 4    a13 = 2(1)+3 = 5
a21 = 2(2)+1 = 5    a22 = 2(2)+2 = 6    a23 = 2(2)+3 = 7
a31 = 2(3)+1 = 7    a32 = 2(3)+2 = 8    a33 = 2(3)+3 = 9
```

```
A = | 3  4  5 |
    | 5  6  7 |
    | 7  8  9 |
```

### 2. `bᵢⱼ = (−1)^(i+j) · i · j`

O fator `(−1)^(i+j)` é o **tabuleiro de xadrez**: `+` quando `i+j` é par, `−` quando
é ímpar.

```
b11 = (+1)(1)(1) =  1    b12 = (-1)(1)(2) = -2    b13 = (+1)(1)(3) =  3
b21 = (-1)(2)(1) = -2    b22 = (+1)(2)(2) =  4    b23 = (-1)(2)(3) = -6
b31 = (+1)(3)(1) =  3    b32 = (-1)(3)(2) = -6    b33 = (+1)(3)(3) =  9
```

```
B = |  1  -2   3 |
    | -2   4  -6 |
    |  3  -6   9 |
```

**Classificação: é SIMÉTRICA.** Repare que `b₁₂ = b₂₁ = −2`, `b₁₃ = b₃₁ = 3` e
`b₂₃ = b₃₂ = −6`. Ou seja, `B = Bᵀ`.

**Por que era previsível:** a lei `(−1)^(i+j) · i · j` é **simétrica na troca de `i`
por `j`** — `(−1)^(i+j)` não muda (a soma é a mesma) e `i·j = j·i`. Toda lei de
formação simétrica em `i` e `j` gera matriz simétrica.

### 3. Classificação

| Matriz | Tipo(s) |
| --- | --- |
| `P` | **nula**, quadrada, diagonal, simétrica, triangular superior E inferior |
| `Q` | quadrada, **diagonal**, simétrica, triangular superior E inferior (é `5·I₃`) |
| `R` | quadrada, **triangular superior** |
| `S` | quadrada, **antissimétrica** (diagonal nula e `sᵢⱼ = −sⱼᵢ`) |
| `T` | quadrada, **simétrica** |
| `U` | **matriz linha** |

> **Duas observações que caem em prova.** Uma matriz diagonal é **ao mesmo tempo**
> triangular superior e inferior — as duas definições são satisfeitas quando tudo
> fora da diagonal é zero. E a matriz nula quadrada satisfaz praticamente todas as
> definições de uma vez.

### 4. Simétrica

Simétrica significa `aᵢⱼ = aⱼᵢ` para todo par. Compare os elementos espelhados:

```
posição (1,2) e (2,1):   x + 1 = 5     =>  x = 4
posição (1,3) e (3,1):       3 = 3     ✓  (já bate, nada a fazer)
posição (2,3) e (3,2):       y = -4    =>  y = -4
```

**Resposta: `x = 4`, `y = −4`.** A matriz fica:

```
A = | 2   5   3 |
    | 5   0  -4 |
    | 3  -4   1 |
```

### 5. Antissimétrica

Antissimétrica significa `aᵢⱼ = −aⱼᵢ`. Primeiro confira a diagonal: já é toda zero,
como a definição exige. Agora os espelhados:

```
posição (1,2) e (2,1):   x - 2 = -3    =>  x = -1
posição (1,3) e (3,1):      -5 = -5    ✓
posição (2,3) e (3,2):   y + 1 = 7     =>  y = 6
```

**Resposta: `x = −1`, `y = 6`.** A matriz fica:

```
B = |  0  -3  -5 |
    |  3   0   7 |
    |  5  -7   0 |
```

> Se a diagonal **não** fosse toda zero, nenhum valor de `x` e `y` resolveria — a
> matriz simplesmente não poderia ser antissimétrica.

### 6. Igualdade

Uma equação por posição:

```
(1,1):  3a - 1 = 8    =>  3a = 9     =>  a = 3
(1,2):  b²     = 9    =>  b = 3  OU  b = -3
(2,1):  2c     = -6   =>  c = -3
(2,2):  d + 4  = 1    =>  d = -3
```

**Resposta: `a = 3`, `b = ±3`, `c = −3`, `d = −3`.**

> A pegadinha é o `b²`: equação de segundo grau tem **duas** raízes, e as duas
> servem. Responder só `b = 3` perde metade da resposta.

---

## Bloco B

### 7. `X = 2A − 3B + C`

```
       | 2  -4   6 |          | 6   3  -3 |
2A  =  | 0   8  -2 |    3B =  | 9   0   6 |
```

Agora elemento a elemento (`2A − 3B + C`):

```
x11 =  2 - 6 + (-1) = -5      x12 = -4 - 3 + 3 = -4      x13 = 6 + 3 + 0 = 9
x21 =  0 - 9 + 1    = -8      x22 =  8 - 0 - 2 =  6      x23 = -2 - 6 + 4 = -4
```

```
X = | -5  -4   9 |
    | -8   6  -4 |
```

> O erro mais comum é o sinal em `−3B` quando `B` já tem negativo: `−3 · (−1) = +3`,
> não `−3`.

### 8. `(A + B)ᵀ = Aᵀ + Bᵀ`

```
            | 3  -1   2 |                    |  3   3 |
A + B  =    | 3   4   1 |     (A+B)ᵀ  =      | -1   4 |
                                             |  2   1 |
```

```
       |  1   0 |          |  2   3 |          |  3   3 |
Aᵀ  =  | -2   4 |    Bᵀ =  |  1   0 |   soma = | -1   4 |
       |  3  -1 |          | -1   2 |          |  2   1 |
```

**São iguais** ✓ A propriedade vale.

### 9. `AB` e `BA`

**a) `AB`:** `A` é `3×2` e `B` é `2×3`. As colunas de `A` (2) batem com as linhas de
`B` (2) → **existe**, e o resultado é **`3×3`**.

```
c11 = 1(2) + 2(-1) = 0      c12 = 1(0) + 2(3) = 6       c13 = 1(1) + 2(2)  = 5
c21 = 3(2) + (-1)(-1) = 7   c22 = 3(0) + (-1)(3) = -3   c23 = 3(1) + (-1)(2) = 1
c31 = 0(2) + 4(-1) = -4     c32 = 0(0) + 4(3) = 12      c33 = 0(1) + 4(2)  = 8
```

```
AB = |  0   6   5 |
     |  7  -3   1 |
     | -4  12   8 |
```

**b) `BA`:** `B` é `2×3` e `A` é `3×2`. Colunas de `B` (3) = linhas de `A` (3) →
**existe**, resultado **`2×2`**.

```
d11 = 2(1) + 0(3) + 1(0) = 2       d12 = 2(2) + 0(-1) + 1(4) = 8
d21 = -1(1) + 3(3) + 2(0) = 8      d22 = -1(2) + 3(-1) + 2(4) = 3
```

```
BA = | 2  8 |
     | 8  3 |
```

**c)** `AB` é `3×3` e `BA` é `2×2`. **Nem a ordem é a mesma** — não há nem como
comparar elemento a elemento.

**Este exercício prova que o produto de matrizes não é comutativo**, e de forma
ainda mais forte que o usual: aqui os dois produtos existem mas vivem em espaços
diferentes. (Há casos em que `AB` existe e `BA` nem chega a existir.)

### 10. `(AB)ᵀ = Bᵀ Aᵀ`

```
              |  0   7  -4 |
(AB)ᵀ    =    |  6  -3  12 |
              |  5   1   8 |
```

`Bᵀ` é `3×2`, `Aᵀ` é `2×3` → `Bᵀ Aᵀ` é `3×3`:

```
        |  2  -1 |           | 1  3  0 |
Bᵀ  =   |  0   3 |    Aᵀ  =  | 2 -1  4 |
        |  1   2 |

           |  2(1)+(-1)(2)   2(3)+(-1)(-1)   2(0)+(-1)(4) |     |  0   7  -4 |
Bᵀ Aᵀ  =   |  0(1)+  3(2)    0(3)+  3(-1)    0(0)+  3(4)  |  =  |  6  -3  12 |
           |  1(1)+  2(2)    1(3)+  2(-1)    1(0)+  2(4)  |     |  5   1   8 |
```

**Iguais** ✓

**Por que `Aᵀ Bᵀ` não existe:** `Aᵀ` é `2×3` e `Bᵀ` é `3×2`... na verdade esse
produto **existe** e dá `2×2`. O que **não** vale é a igualdade: `Aᵀ Bᵀ` é `2×2` e
`(AB)ᵀ` é `3×3`. **Ordens diferentes ⟹ nunca poderiam ser iguais.** É essa a razão
estrutural pela qual a ordem tem de inverter em `(AB)ᵀ = BᵀAᵀ` — só assim as
dimensões fecham.

### 11. Potências de `A = [[2, 1], [−1, 3]]`

```
A² = A·A = | 2(2) + 1(-1)     2(1) + 1(3)  |  = |  3   5 |
           | -1(2) + 3(-1)   -1(1) + 3(3)  |    | -5   8 |

A³ = A²·A = | 3(2) + 5(-1)     3(1) + 5(3)  |  = |   1  18 |
            | -5(2) + 8(-1)   -5(1) + 8(3)  |    | -18  19 |
```

**`A² = [[3, 5], [−5, 8]]`  e  `A³ = [[1, 18], [−18, 19]]`.**

> Calcule `A³` como `A²·A`, nunca refazendo tudo do zero — e **confira `A²` antes de
> seguir**, porque um erro ali se propaga para `A³`.

### 12. Forma matricial

```
| 3  -1   2 |   | x |     |  7 |
| 1   4  -1 | · | y |  =  |  1 |
| 2   5   0 |   | z |     | -3 |
```

Efetuando `A · X`:

```
| 3x - y + 2z |
| x + 4y -  z |
| 2x + 5y + 0z |
```

que é exatamente o lado esquerdo do sistema ✓

> Repare no `0z` na terceira linha: a variável `z` **não aparece** na terceira
> equação, e é por isso que o coeficiente é 0. Esquecer de escrever esse zero é o
> erro mais comum ao montar a matriz.

---

## Bloco C

### 13. Inversa de `A = [[3, 5], [1, 2]]`

```
[ A | I ]          | 3  5 | 1  0 |
                   | 1  2 | 0  1 |

L1 <-> L2          | 1  2 | 0  1 |      (trocar põe o 1 no pivô e evita frações)
                   | 3  5 | 1  0 |

L2 <- L2 - 3·L1    | 1  2 |  0   1 |
                   | 0 -1 |  1  -3 |

L2 <- -1 · L2      | 1  2 |  0   1 |
                   | 0  1 | -1   3 |

L1 <- L1 - 2·L2    | 1  0 |  2  -5 |
                   | 0  1 | -1   3 |
```

```
A⁻¹ = |  2  -5 |
      | -1   3 |
```

**Conferência:**

```
| 3  5 | | 2  -5 |   | 6-5   -15+15 |   | 1  0 |
| 1  2 |·| -1  3 | = | 2-2    -5+6  | = | 0  1 |   ✓
```

### 14. Inversa de `A = [[1, 0, 2], [2, −1, 3], [4, 1, 8]]`

```
[ A | I ]               | 1   0   2 |  1   0   0 |
                        | 2  -1   3 |  0   1   0 |
                        | 4   1   8 |  0   0   1 |

L2 <- L2 - 2·L1         | 1   0   2 |  1   0   0 |
L3 <- L3 - 4·L1         | 0  -1  -1 | -2   1   0 |
                        | 0   1   0 | -4   0   1 |

L2 <- -1 · L2           | 1   0   2 |  1   0   0 |
                        | 0   1   1 |  2  -1   0 |
                        | 0   1   0 | -4   0   1 |

L3 <- L3 - L2           | 1   0   2 |  1   0   0 |
                        | 0   1   1 |  2  -1   0 |
                        | 0   0  -1 | -6   1   1 |

L3 <- -1 · L3           | 1   0   2 |  1   0   0 |
                        | 0   1   1 |  2  -1   0 |
                        | 0   0   1 |  6  -1  -1 |

L1 <- L1 - 2·L3         | 1   0   0 | -11   2   2 |
L2 <- L2 - L3           | 0   1   0 |  -4   0   1 |
                        | 0   0   1 |   6  -1  -1 |
```

```
A⁻¹ = | -11   2   2 |
      |  -4   0   1 |
      |   6  -1  -1 |
```

**Conferência da primeira linha de `A·A⁻¹`:**

```
1(-11) + 0(-4) + 2(6)   = -11 + 0 + 12 = 1  ✓
1(2)   + 0(0)  + 2(-1)  =   2 + 0 - 2  = 0  ✓
1(2)   + 0(1)  + 2(-1)  =   2 + 0 - 2  = 0  ✓
```

> A inversa saiu com **entradas inteiras** porque `det(A) = 1`. Sempre que a inversa
> der inteiros, desconfie (positivamente): o determinante era `±1`.

### 15. Matriz sem inversa

```
| 1  2  3 |  1  0  0 |
| 2  4  6 |  0  1  0 |
| 1  0  1 |  0  0  1 |

L2 <- L2 - 2·L1      | 1  2  3 |  1  0  0 |
L3 <- L3 - L1        | 0  0  0 | -2  1  0 |   <-- LINHA INTEIRA DE ZEROS
                     | 0 -2 -2 | -1  0  1 |
```

A segunda linha da esquerda **zerou por completo**. A partir daqui, nenhuma
operação elementar pode reintroduzir um pivô naquela linha: multiplicar zero por
qualquer coisa dá zero, e somar múltiplos das outras linhas destruiria os pivôs
delas.

Logo `A` **não é equivalente a `I₃`** e, pelo teorema, **não admite inversa**.

**Por que aconteceu:** `L2 = 2·L1` — as duas primeiras linhas são **proporcionais**.
Linhas proporcionais sempre colapsam assim. (E, de fato, `det(A) = 0` — mas o
exercício pedia sem determinante.)

### 16. `(AB)⁻¹ = B⁻¹A⁻¹`

```
A = | 2  1 |   det = 1      B = | 1  -1 |   det = 3
    | 3  2 |                    | 2   1 |

AB = | 2(1)+1(2)   2(-1)+1(1) |  = | 4  -1 |     det(AB) = 4·7 - (-1)(7) ... = 7
     | 3(1)+2(2)   3(-1)+2(1) |    | 7  -1 |     det = 4(-1) - (-1)(7) = 3
```

Pelo atalho de ordem 2 (`[[a,b],[c,d]]⁻¹ = (1/det)·[[d,−b],[−c,a]]`):

```
(AB)⁻¹ = (1/3) | -1   1 |  = | -1/3   1/3 |
               | -7   4 |    | -7/3   4/3 |

A⁻¹ = (1/1) |  2  -1 | = |  2  -1 |      B⁻¹ = (1/3) | 1   1 | = | 1/3   1/3 |
            | -3   2 |   | -3   2 |                  | -2  1 |   | -2/3  1/3 |

B⁻¹A⁻¹ = | 1/3   1/3 | · |  2  -1 |  = | 2/3 - 1     -1/3 + 2/3 |  = | -1/3   1/3 |
         | -2/3  1/3 |   | -3   2 |    | -4/3 - 1    2/3 + 2/3  |    | -7/3   4/3 |
```

**Iguais** ✓

**E `A⁻¹B⁻¹`?**

```
A⁻¹B⁻¹ = |  2  -1 | · | 1/3   1/3 |  = | 2/3 + 2/3    2/3 - 1/3 |  = | 4/3   1/3 |
         | -3   2 |   | -2/3  1/3 |    | -1 - 4/3     -1 + 2/3  |    | -7/3  -1/3 |
```

**Diferente de `(AB)⁻¹`.** Confirma que **a ordem tem de inverter**: `(AB)⁻¹ = B⁻¹A⁻¹`,
nunca `A⁻¹B⁻¹`.

### 17. `X = A⁻¹ · B`

```
A = | 2  1 |    B = |  8 |
    | 5  3 |        | 21 |

det(A) = 2(3) - 1(5) = 1

A⁻¹ = (1/1) |  3  -1 |  = |  3  -1 |
            | -5   2 |    | -5   2 |

X = A⁻¹·B = |  3  -1 | · |  8 |  = |  3(8) - 1(21)  |  = | 24 - 21 |  = |  3 |
            | -5   2 |   | 21 |    | -5(8) + 2(21)  |    | -40 + 42|    |  2 |
```

**Solução: `x = 3`, `y = 2`.**

**Conferência:** `2(3) + 2 = 8` ✓ e `5(3) + 3(2) = 21` ✓

---

## Bloco D

### 18. `Q` é ortogonal?

O teste é `Q · Qᵀ = I`.

```
Q = | 3/5  -4/5 |        Qᵀ = |  3/5   4/5 |
    | 4/5   3/5 |             | -4/5   3/5 |

Q·Qᵀ  (1,1) = (3/5)(3/5) + (-4/5)(-4/5) = 9/25 + 16/25 = 1   ✓
      (1,2) = (3/5)(4/5) + (-4/5)(3/5)  = 12/25 - 12/25 = 0  ✓
      (2,1) = (4/5)(3/5) + (3/5)(-4/5)  = 12/25 - 12/25 = 0  ✓
      (2,2) = (4/5)(4/5) + (3/5)(3/5)   = 16/25 + 9/25 = 1   ✓
```

**`Q·Qᵀ = I` ⟹ `Q` É ORTOGONAL.**

**Pelo teste das linhas** (mais rápido): linha 1 `= (3/5, −4/5)` tem norma
`√(9/25 + 16/25) = 1`; linha 2 idem; produto escalar entre elas `= 0`. Linhas
**ortonormais** ⟹ ortogonal.

**`det(Q) = (3/5)(3/5) − (−4/5)(4/5) = 9/25 + 16/25 = 1`.**

> Toda matriz ortogonal tem `det = ±1`. Com `det = +1`, `Q` é uma **rotação** — no
> caso, a do triângulo pitagórico 3-4-5.

### 19. `A` é idempotente?

Teste: `A² = A`. Calculando linha por linha:

```
linha 1 de A = (2, -1, 1):
   col1:  2(2) + (-1)(-3) + 1(-5)  =  4 + 3 - 5  =  2   ✓
   col2:  2(-1) + (-1)(4) + 1(5)   = -2 - 4 + 5  = -1   ✓
   col3:  2(1) + (-1)(-3) + 1(-4)  =  2 + 3 - 4  =  1   ✓

linha 2 de A = (-3, 4, -3):
   col1: -3(2) + 4(-3) + (-3)(-5)  = -6 - 12 + 15 = -3  ✓
   col2: -3(-1) + 4(4) + (-3)(5)   =  3 + 16 - 15 =  4  ✓
   col3: -3(1) + 4(-3) + (-3)(-4)  = -3 - 12 + 12 = -3  ✓

linha 3 de A = (-5, 5, -4):
   col1: -5(2) + 5(-3) + (-4)(-5)  = -10 - 15 + 20 = -5  ✓
   col2: -5(-1) + 5(4) + (-4)(5)   =   5 + 20 - 20 =  5  ✓
   col3: -5(1) + 5(-3) + (-4)(-4)  =  -5 - 15 + 16 = -4  ✓
```

**`A² = A` ⟹ `A` É IDEMPOTENTE**, com período `2 − 1 = 1`.

**`A¹⁰⁰ = A`.** Como `A² = A`, segue `A³ = A²·A = A·A = A²= A`, e por indução
`Aⁿ = A` para todo `n ≥ 1`. A potência **não cresce**.

### 20. `N` é nihilpotente?

```
N = | 0  1  2 |
    | 0  0  3 |
    | 0  0  0 |

N² = N·N:
   linha 1 (0,1,2):  col1 = 0     col2 = 0(1)+1(0)+2(0) = 0     col3 = 0(2)+1(3)+2(0) = 3
   linha 2 (0,0,3):  col1 = 0     col2 = 0               col3 = 0(2)+0(3)+3(0) = 0
   linha 3 (0,0,0):  tudo 0

N² = | 0  0  3 |        <- ainda NÃO é nula
     | 0  0  0 |
     | 0  0  0 |

N³ = N²·N:
   linha 1 (0,0,3):  col1 = 0   col2 = 0(1)+0(0)+3(0) = 0   col3 = 0(2)+0(3)+3(0) = 0
   demais linhas: nulas

N³ = matriz nula
```

**`N` é NIHILPOTENTE de índice `p = 3`** — porque `N² ≠ 0` mas `N³ = 0`, e o índice
é o **menor** expoente que zera.

> **Padrão geral:** toda matriz **triangular estritamente superior** (zeros na
> diagonal e abaixo) de ordem `n` é nihilpotente com índice no máximo `n`. A cada
> potência, a faixa de não-zeros "sobe" uma diagonal até sair da matriz.

### 21. Verdadeiro ou falso

**a) Se `A² = 0`, então `A = 0`.** **FALSO.**
Contraexemplo: o `N` do exercício 20 tem `N³ = 0` mas `N ≠ 0`. Mais direto ainda:
`A = [[0,1],[0,0]]` tem `A² = 0` com `A ≠ 0`. Nos reais `x² = 0 => x = 0`; em
matrizes, **não**.

**b) Toda matriz simétrica é quadrada.** **VERDADEIRO.**
Simétrica exige `A = Aᵀ`. Se `A` é `m×n`, então `Aᵀ` é `n×m`. Para serem iguais
precisam ter a mesma ordem: `m = n`. A quadratura não é uma exigência extra da
definição — é **consequência** dela.

**c) Se `A` é ortogonal, então `det(A) = 1`.** **FALSO.**
De `A·Aᵀ = I` vem `det(A)·det(Aᵀ) = 1`, e como `det(Aᵀ) = det(A)`, temos
`det(A)² = 1` ⟹ **`det(A) = ±1`**. O valor `−1` é perfeitamente possível: a matriz
`[[1,0],[0,−1]]` (uma reflexão) é ortogonal com `det = −1`.

**d) Se `A` é idempotente e inversível, então `A = I`.** **VERDADEIRO.**
De `A² = A`, multiplique os dois lados por `A⁻¹` (que existe por hipótese):

```
A⁻¹·A² = A⁻¹·A   =>   A = I
```

Ou seja, **a única matriz idempotente inversível é a identidade**. Toda outra
idempotente tem `det = 0`.

**e) A diagonal de uma antissimétrica pode ter elementos não nulos.** **FALSO.**
A definição dá `aᵢᵢ = −aᵢᵢ`, logo `2aᵢᵢ = 0`, logo `aᵢᵢ = 0`. A diagonal é
**obrigatoriamente** nula.

---

## Bloco E — Determinantes

### 22. Ordem 2

```
a) det | 5  -3 |  =  5(4) - (-3)(2)  =  20 + 6  =  26
       | 2   4 |

b) det | -1  -2 |  =  (-1)(-4) - (-2)(-3)  =  4 - 6  =  -2
       | -3  -4 |

c) det | 7   0 |  =  7(-2) - 0(0)  =  -14
       | 0  -2 |
```

> Em (c), a matriz é **diagonal** — bastaria multiplicar a diagonal: `7 · (−2) = −14`.

### 23. Sarrus

**a)**

```
| 2  -1   3 |
| 0   4  -2 |
| 1   5   1 |

diagonais principais:   2(4)(1) + (-1)(-2)(1) + 3(0)(5)  =  8 + 2 + 0  =  10
diagonais secundárias:  3(4)(1) + 2(-2)(5) + (-1)(0)(1)  =  12 - 20 + 0 = -8

det = 10 - (-8) = 18
```

**b)**

```
| 1  1  1 |
| 2  3  4 |
| 5  6  7 |

principais:   1(3)(7) + 1(4)(5) + 1(2)(6) = 21 + 20 + 12 = 53
secundárias:  1(3)(5) + 1(4)(6) + 1(2)(7) = 15 + 24 + 14 = 53

det = 53 - 53 = 0
```

> `det = 0` aqui não é coincidência: `L3 − L2 = (3,3,3)` e `L2 − L1 = (1,2,3)`…
> mais simples: `L1 + L3 = (6,7,8)` e `2·L2 = (4,6,8)`. De fato
> `L3 = 2·L2 − L1` — as linhas são **linearmente dependentes**.

**c)**

```
|  3  0  -1 |
|  2  5   4 |
| -1  2   3 |

principais:   3(5)(3) + 0(4)(-1) + (-1)(2)(2) = 45 + 0 - 4 = 41
secundárias:  (-1)(5)(-1) + 3(4)(2) + 0(2)(3) = 5 + 24 + 0 = 29

det = 41 - 29 = 12
```

### 24. Sem calcular

| | det | Propriedade |
| --- | --- | --- |
| **a)** | **0** | `L3 = L1 + L2`: `(2,4,6) + (1,3,5) = (3,7,11)` ✓ — filas **linearmente dependentes** |
| **b)** | **0** | `L1` e `L3` são **iguais** |
| **c)** | **0** | a **segunda coluna é nula** |
| **d)** | **−24** | **triangular superior** ⟹ produto da diagonal: `3 · (−2) · 4 = −24` |

> Em (a), a propriedade decorada é "duas filas iguais ou proporcionais ⟹ det = 0",
> mas a versão geral é mais forte: **qualquer** dependência linear entre as filas
> zera o determinante. Aqui nenhuma linha é múltipla de outra — a terceira é a
> **soma** das duas primeiras, e isso basta.

### 25. `det(2A)` com `A` sendo `3×3` e `det(A) = 9`

**`det(2A) = 2³ · det(A) = 8 · 9 = 72`.**

**Por quê:** multiplicar a **matriz inteira** por 2 significa multiplicar **cada uma
das 3 linhas** por 2. A propriedade "fila × β ⟹ det × β" se aplica **três vezes**:

```
det(2A) = 2 · 2 · 2 · det(A) = 2³ · det(A)
```

A regra geral, para `A` de ordem `n`: **`det(kA) = kⁿ · det(A)`**.

> A resposta `18` sai de aplicar o fator uma única vez — é o erro que o enunciado
> avisa. `k·det(A)` só vale quando **uma única fila** foi multiplicada.

### 26. Triangulação `4×4`

Começar trocando `L1` e `L2` evita frações, porque `L2` já começa com 1:

```
            |  2   1  -1   3 |
            |  1   0   2  -1 |
            |  3   2   1   0 |
            | -1   1   4   2 |

L1 <-> L2   (uma permuta: o sinal do determinante INVERTE)

            |  1   0   2  -1 |
            |  2   1  -1   3 |
            |  3   2   1   0 |
            | -1   1   4   2 |

L2 <- L2 - 2·L1        (não altera o determinante)
L3 <- L3 - 3·L1
L4 <- L4 + L1

            |  1   0   2  -1 |
            |  0   1  -5   5 |
            |  0   2  -5   3 |
            |  0   1   6   1 |

L3 <- L3 - 2·L2
L4 <- L4 - L2

            |  1   0   2  -1 |
            |  0   1  -5   5 |
            |  0   0   5  -7 |
            |  0   0  11  -4 |

L4 <- L4 - (11/5)·L3

            |  1   0   2   -1  |
            |  0   1  -5    5  |
            |  0   0   5   -7  |
            |  0   0   0   57/5|
```

```
produto da diagonal = 1 · 1 · 5 · (57/5) = 57
uma permuta de linhas => sinal invertido

det(A) = -57
```

### 27. Laplace `4×4`

```
| 4   0   0   1 |
| 2  -1   0   3 |
| 0   5   2   0 |
| 1   0   0  -2 |
```

**A terceira coluna é `(0, 0, 2, 0)` — três zeros.** Desenvolvendo por ela, apenas
**um** cofator precisa ser calculado:

```
det(A) = a33 · (-1)^(3+3) · det(M33) = 2 · (+1) · det(M33)
```

`M33` remove a linha 3 e a coluna 3:

```
M33 = | 4   0   1 |
      | 2  -1   3 |
      | 1   0  -2 |
```

Esta `3×3` também tem uma coluna quase vazia — a **segunda**, `(0, −1, 0)`.
Desenvolvendo por ela:

```
det(M33) = (-1) · (-1)^(2+2) · det | 4   1 |
                                   | 1  -2 |
         = (-1) · (+1) · (4(-2) - 1(1))
         = (-1) · (-9)
         = 9
```

```
det(A) = 2 · 9 = 18
```

> Duas expansões em cadeia, sempre pela fila com mais zeros, resolveram uma `4×4`
> com **duas multiplicações**. Por Sarrus estendido (que nem existe) ou por força
> bruta seriam 24 produtos de 4 fatores.

### 28. Laplace `5×5`

```
| 1  2  0  0  3 |
| 0  0  5  0  0 |   <- quatro zeros
| 2  1  0  4  1 |
| 0  3  0  1  2 |
| 1  0  0  2  0 |
```

**A segunda linha é `(0, 0, 5, 0, 0)`.** Um único cofator:

```
det(A) = 5 · (-1)^(2+3) · det(M23) = -5 · det(M23)
```

`M23` remove a linha 2 e a coluna 3:

```
M23 = | 1  2  0  3 |
      | 2  1  4  1 |
      | 0  3  1  2 |
      | 1  0  2  0 |
```

Triangulando:

```
L2 <- L2 - 2·L1        | 1   2   0   3 |
L4 <- L4 - L1          | 0  -3   4  -5 |
                       | 0   3   1   2 |
                       | 0  -2   2  -3 |

L3 <- L3 + L2          | 1   2   0   3 |
L4 <- L4 - (2/3)·L2    | 0  -3   4  -5 |
                       | 0   0   5  -3 |
                       | 0   0  -2/3  1/3 |

L4 <- L4 + (2/15)·L3   | 1   2   0    3   |
                       | 0  -3   4   -5   |
                       | 0   0   5   -3   |
                       | 0   0   0  -1/15 |

det(M23) = 1 · (-3) · 5 · (-1/15) = 1
```

```
det(A) = -5 · 1 = -5
```

### 29. Binet e `det(A⁻¹)`

**a)**

```
A = | 3  1 |   det(A) = 3(4) - 1(2) = 10
    | 2  4 |

B = | 1  -2 |  det(B) = 1(1) - (-2)(3) = 1 + 6 = 7
    | 3   1 |

AB = | 3(1)+1(3)   3(-2)+1(1) | = |  6  -5 |
     | 2(1)+4(3)   2(-2)+4(1) |   | 14   0 |

det(AB) = 6(0) - (-5)(14) = 70
```

**`det(A)·det(B) = 10 · 7 = 70 = det(AB)`** ✓ Teorema de Binet confirmado.

**b)**

```
A⁻¹ = (1/10) |  4  -1 |  = |  2/5   -1/10 |
             | -2   3 |    | -1/5    3/10 |

det(A⁻¹) = (2/5)(3/10) - (-1/10)(-1/5) = 6/50 - 1/50 = 5/50 = 1/10
```

**`det(A⁻¹) = 1/10 = 1/det(A)`** ✓

---

## Bloco F — Inversa por cofator

### 30. `A = [[4, 7], [2, 6]]`

**Passo 1 — determinante:** `det(A) = 4(6) − 7(2) = 24 − 14 = 10`. Como `≠ 0`, a
inversa existe.

**Passo 2 — matriz dos cofatores.** `Cᵢⱼ = (−1)^(i+j) · det(Mᵢⱼ)`; em ordem 2, cada
menor é um único elemento (o "oposto" na diagonal):

```
C11 = (+1) · det[6] =  6        C12 = (-1) · det[2] = -2
C21 = (-1) · det[7] = -7        C22 = (+1) · det[4] =  4

C = |  6  -2 |
    | -7   4 |
```

**Passo 3 — adjunta = transposta dos cofatores.** *É o passo que todo mundo pula:*

```
adj(A) = Cᵀ = |  6  -7 |
              | -2   4 |
```

**Passo 4 — dividir pelo determinante:**

```
A⁻¹ = (1/10) |  6  -7 |  = |  3/5   -7/10 |
             | -2   4 |    | -1/5    2/5  |
```

**Conferência:**

```
| 4  7 | | 3/5   -7/10 |   | 12/5 - 7/5    -28/10 + 28/10 |   | 1  0 |
| 2  6 |·| -1/5   2/5  | = |  6/5 - 6/5    -14/10 + 24/10 | = | 0  1 |  ✓
```

### 31. `A = [[1, 2, 3], [0, 1, 4], [5, 6, 0]]`

**Passo 1 — determinante** (Sarrus):

```
principais:   1(1)(0) + 2(4)(5) + 3(0)(6) = 0 + 40 + 0 = 40
secundárias:  3(1)(5) + 1(4)(6) + 2(0)(0) = 15 + 24 + 0 = 39

det(A) = 40 - 39 = 1
```

**Passo 2 — os nove cofatores.** Cada um é `(−1)^(i+j)` vezes o determinante da
`2×2` que sobra ao apagar a linha `i` e a coluna `j`:

```
C11 = + det | 1  4 |  = 1(0) - 4(6)  = -24
            | 6  0 |

C12 = - det | 0  4 |  = -(0(0) - 4(5)) = -(-20) =  20
            | 5  0 |

C13 = + det | 0  1 |  = 0(6) - 1(5)  =  -5
            | 5  6 |

C21 = - det | 2  3 |  = -(2(0) - 3(6)) = -(-18) =  18
            | 6  0 |

C22 = + det | 1  3 |  = 1(0) - 3(5)  = -15
            | 5  0 |

C23 = - det | 1  2 |  = -(1(6) - 2(5)) = -(-4)  =   4
            | 5  6 |

C31 = + det | 2  3 |  = 2(4) - 3(1)  =   5
            | 1  4 |

C32 = - det | 1  3 |  = -(1(4) - 3(0)) = -4
            | 0  4 |

C33 = + det | 1  2 |  = 1(1) - 2(0)  =   1
            | 0  1 |
```

```
C = | -24   20   -5 |
    |  18  -15    4 |
    |   5   -4    1 |
```

**Passo 3 — adjunta (transposta):**

```
adj(A) = Cᵀ = | -24   18    5 |
              |  20  -15   -4 |
              |  -5    4    1 |
```

**Passo 4 — dividir por `det(A) = 1`:**

```
A⁻¹ = | -24   18    5 |
      |  20  -15   -4 |
      |  -5    4    1 |
```

**Conferência (primeira linha de `A·A⁻¹`):**

```
1(-24) + 2(20) + 3(-5)  = -24 + 40 - 15 = 1  ✓
1(18) + 2(-15) + 3(4)   =  18 - 30 + 12 = 0  ✓
1(5) + 2(-4) + 3(1)     =   5 -  8 +  3 = 0  ✓
```

> Como `det(A) = 1`, a adjunta **é** a inversa — a divisão não mudou nada. Repare
> também que a adjunta é a transposta de `C`: compare `C₁₂ = 20` com a posição
> `(2,1)` da adjunta. Se você esquecer de transpor, a inversa sai **errada mas com
> os números certos nas posições trocadas** — um erro difícil de perceber.

---

## Bloco G — Sistemas lineares

### 32. Três sistemas `2×2`

**a) `x + 2y = 5` ; `3x − y = 1`**

```
| 1   2 |  5 |
| 3  -1 |  1 |

L2 <- L2 - 3·L1      | 1   2 |   5 |
                     | 0  -7 | -14 |
```

`−7y = −14` ⟹ `y = 2`. Daí `x + 2(2) = 5` ⟹ `x = 1`.

**SPD, solução `(x, y) = (1, 2)`.** Geometricamente: retas **concorrentes**.

**b) `2x + 4y = 6` ; `x + 2y = 3`**

```
| 2  4 |  6 |
| 1  2 |  3 |

L1 <- L1 / 2         | 1  2 |  3 |
                     | 1  2 |  3 |

L2 <- L2 - L1        | 1  2 |  3 |
                     | 0  0 |  0 |
```

Linha **inteiramente nula** ⟹ a segunda equação não acrescenta nada (era a primeira
multiplicada por 2).

**SPI.** Solução geral: `x = 3 − 2t`, ou seja **`(x, y) = (3 − 2t, t)`**, `t ∈ ℝ`.
Geometricamente: retas **coincidentes**.

**c) `x − 3y = 2` ; `−2x + 6y = 1`**

```
|  1  -3 |  2 |
| -2   6 |  1 |

L2 <- L2 + 2·L1      | 1  -3 |  2 |
                     | 0   0 |  5 |
```

A última linha diz `0x + 0y = 5`, ou seja **`0 = 5`**. Absurdo.

**SI.** Geometricamente: retas **paralelas** (mesmo coeficiente angular, interceptos
diferentes).

### 33. Escalonamento `3×3`

```
| 1   1   1 |  6 |
| 2  -1   1 |  3 |
| 1   2  -1 |  2 |

L2 <- L2 - 2·L1      | 1   1   1 |  6 |
L3 <- L3 - L1        | 0  -3  -1 | -9 |
                     | 0   1  -2 | -4 |

L2 <- L2 + 3·L3      | 1   1   1 |   6 |     (combinando para eliminar y sem frações)
                     | 0   0  -7 | -21 |
                     | 0   1  -2 |  -4 |
```

Substituição retroativa:

```
-7z = -21          =>  z = 3
 y - 2(3) = -4     =>  y - 6 = -4   =>  y = 2
 x + 2 + 3 = 6     =>  x = 1
```

**SPD, solução `(x, y, z) = (1, 2, 3)`.**

**Conferência nas três equações originais:**

```
1 + 2 + 3      = 6   ✓
2(1) - 2 + 3   = 3   ✓
1 + 2(2) - 3   = 2   ✓
```

### 34. Classificação

```
| 1  2  -1 |  1 |
| 2  3   1 |  4 |
| 3  5   0 |  6 |

L2 <- L2 - 2·L1      | 1   2  -1 |  1 |
L3 <- L3 - 3·L1      | 0  -1   3 |  2 |
                     | 0  -1   3 |  3 |

L3 <- L3 - L2        | 1   2  -1 |  1 |
                     | 0  -1   3 |  2 |
                     | 0   0   0 |  1 |
```

A terceira linha diz **`0 = 1`**.

**SI — Sistema Impossível.**

> Repare o detalhe: as linhas 2 e 3 ficaram com **coeficientes idênticos**
> (`0 −1 3`) mas **termos independentes diferentes** (2 e 3). Duas equações que
> dizem coisas contraditórias sobre a mesma combinação de incógnitas. Se os termos
> independentes fossem iguais, seria SPI.

### 35. Solução geral

```
|  1   2  -1 |   3 |
|  2   4  -2 |   6 |
| -1  -2   1 |  -3 |

L2 <- L2 - 2·L1      | 1  2  -1 |  3 |
L3 <- L3 + L1        | 0  0   0 |  0 |
                     | 0  0   0 |  0 |
```

**Duas linhas inteiramente nulas.** As três equações eram a mesma: `L2 = 2·L1` e
`L3 = −L1`.

**SPI**, com **1 equação e 3 incógnitas** ⟹ **2 graus de liberdade**.

Isolando `x` na única equação que sobrou (`x + 2y − z = 3`):

```
x = 3 - 2y + z
```

**Solução geral:** `(x, y, z) = (3 − 2s + t, s, t)`, com `s, t ∈ ℝ`.

**Conferência com `s = 1, t = 2`:** `x = 3 − 2 + 2 = 3`, e
`3 + 2(1) − 2 = 3` ✓

### 36. Sistema homogêneo

```
| 1  2   3 |  0 |
| 2  5   7 |  0 |
| 3  7  10 |  0 |

L2 <- L2 - 2·L1      | 1  2  3 |  0 |
L3 <- L3 - 3·L1      | 0  1  1 |  0 |
                     | 0  1  1 |  0 |

L3 <- L3 - L2        | 1  2  3 |  0 |
                     | 0  1  1 |  0 |
                     | 0  0  0 |  0 |
```

Duas equações independentes, três incógnitas ⟹ **SPI** (1 grau de liberdade).

```
da 2ª equação:  y + z = 0        =>  y = -z
na 1ª equação:  x + 2(-z) + 3z = 0   =>  x + z = 0   =>  x = -z
```

**Solução geral:** `(x, y, z) = (−t, −t, t) = t·(−1, −1, 1)`, com `t ∈ ℝ`.

> **A solução trivial `(0,0,0)` está inclusa** (basta `t = 0`), como tem de estar em
> todo sistema homogêneo. Aqui existem infinitas outras além dela.
>
> Note ainda que **um sistema homogêneo nunca é SI**: a coluna de termos
> independentes é toda zero, então nenhuma operação elementar pode produzir
> `0 = k ≠ 0`.

### 37. Discussão em `k`

```
| 1  2  1 |  3 |
| 2  5  3 |  8 |
| 1  3  k |  5 |
```

**Passo 1 — o determinante decide.**

```
det = 1·(5k - 9) - 2·(2k - 3) + 1·(6 - 5)
    = 5k - 9 - 4k + 6 + 1
    = k - 2
```

**Passo 2 — `det ≠ 0` ⟹ SPD.**

```
k - 2 ≠ 0   =>   k ≠ 2   =>   SPD  (solução única)
```

**Passo 3 — o caso `k = 2` precisa ser investigado à mão.** `det = 0` só diz "não é
SPD"; não distingue SPI de SI. Escalonando com `k = 2`:

```
| 1  2  1 |  3 |
| 2  5  3 |  8 |
| 1  3  2 |  5 |

L2 <- L2 - 2·L1      | 1  2  1 |  3 |
L3 <- L3 - L1        | 0  1  1 |  2 |
                     | 0  1  1 |  2 |

L3 <- L3 - L2        | 1  2  1 |  3 |
                     | 0  1  1 |  2 |
                     | 0  0  0 |  0 |
```

Linha **nula inteira** (termo independente incluído) ⟹ **SPI**.

**Resposta:**

| `k` | Classificação |
| --- | --- |
| `k ≠ 2` | **SPD** |
| `k = 2` | **SPI** |
| — | **nunca SI** |

> As linhas 2 e 3 viraram a mesma equação `y + z = 2`, com o **mesmo** termo
> independente. Se o termo independente da terceira equação fosse outro (digamos 6
> em vez de 5), o mesmo `k = 2` daria **SI**. É por isso que o determinante sozinho
> não fecha a questão: ele enxerga só a matriz dos coeficientes, não a coluna dos
> termos independentes.

### 38. Verdadeiro ou falso

**a) Um sistema homogêneo pode ser SI.** **FALSO.**
Todo homogêneo admite a solução trivial `(0, 0, …, 0)` — basta substituir. Ter pelo
menos uma solução significa ser **possível**. Logo é SPD (só a trivial) ou SPI (a
trivial mais infinitas), **nunca SI**.

**b) 3 equações e 5 incógnitas pode ser SPD.** **FALSO.**
No máximo 3 pivôs para 5 incógnitas ⟹ pelo menos **2 variáveis livres** ⟹ infinitas
soluções. Se for possível, é SPI; se as equações se contradisserem, é SI. Solução
única é impossível. **Menos equações que incógnitas ⟹ nunca SPD.**

**c) `det(A) = 0` ⟹ `AX = B` é SI.** **FALSO.**
`det(A) = 0` só elimina o SPD. O sistema pode ser **SPI** — o exercício 37 com
`k = 2` é exatamente esse caso: `det = 0` e infinitas soluções. `det = 0` significa
"SPI **ou** SI", e só o escalonamento distingue.

**d) 4 equações e 2 incógnitas nunca pode ser SPD.** **FALSO.**
Mais equações que incógnitas **não** impede solução única — as equações extras podem
ser redundantes. Exemplo:

```
x + y = 3
x - y = 1
2x + 2y = 6      (= 2 × primeira)
3x - 3y = 3      (= 3 × segunda)
```

Solução única `(2, 1)` ⟹ **SPD**. A restrição real é a do item (b), no outro
sentido.

**e) `det(A) ≠ 0` ⟹ SPD para qualquer `B`.** **VERDADEIRO.**
Se `det(A) ≠ 0`, existe `A⁻¹`, e então `X = A⁻¹B` é solução — e é **única**, porque
multiplicar `AX = B` por `A⁻¹` à esquerda determina `X` completamente. Vale para
**qualquer** `B`, inclusive `B = 0` (aí a única solução é a trivial).

### 39. Problema aplicado

Sejam `x` = preço do SSD, `y` = preço do pente de RAM, `z` = preço da fonte.

```
 x + 2y +  z = 1400        (Kit 1)
2x +  y +  z = 1500        (Kit 2)
 x +  y + 2z = 1500        (Kit 3)
```

Escalonando a ampliada:

```
| 1  2  1 |  1400 |
| 2  1  1 |  1500 |
| 1  1  2 |  1500 |

L2 <- L2 - 2·L1      | 1   2   1 |  1400 |
L3 <- L3 - L1        | 0  -3  -1 | -1300 |
                     | 0  -1   1 |   100 |

L2 <- L2 - 3·L3      | 1   2   1 |  1400 |
                     | 0   0  -4 | -1600 |
                     | 0  -1   1 |   100 |
```

Substituição retroativa:

```
-4z = -1600        =>  z = 400
-y + 400 = 100     =>  -y = -300   =>  y = 300
x + 2(300) + 400 = 1400   =>  x + 1000 = 1400   =>  x = 400
```

**Resposta: SSD = R$ 400 · pente de RAM = R$ 300 · fonte = R$ 400.**

**Conferência nos três kits:**

```
Kit 1:  400 + 2(300) + 400  = 400 + 600 + 400  = 1400  ✓
Kit 2:  2(400) + 300 + 400  = 800 + 300 + 400  = 1500  ✓
Kit 3:  400 + 300 + 2(400)  = 400 + 300 + 800  = 1500  ✓
```

> `det(A) = −4 ≠ 0`, o que já garantia **antes de qualquer conta** que a solução
> seria única — os três kits trazem informação genuinamente independente sobre os
> preços. Se o determinante desse zero, algum kit seria combinação dos outros e os
> preços não ficariam determinados.

### 40. `A` é `3×3` com `det(A) = −5`

**a) `A` admite inversa?** **Sim.** Pela propriedade `A` é inversível **se e somente
se** `det(A) ≠ 0`. Aqui `−5 ≠ 0`.

**b) `det(A⁻¹)`?** **`−1/5`.** Porque `det(A⁻¹) = 1/det(A) = 1/(−5) = −1/5`.

**c) `AX = B` é SPD, SPI ou SI?** **SPD**, para qualquer `B`. Como `A⁻¹` existe,
`X = A⁻¹B` é solução e é única.

**d) E o homogêneo `AX = 0`?** **SPD também** — e a única solução é a **trivial**:

```
X = A⁻¹ · 0 = 0
```

> Este exercício fecha a cadeia da disciplina: **um único número** (`det = −5`)
> respondeu se a inversa existe, quanto vale o determinante dela, e como se
> classificam dois sistemas — sem escalonar nada.

---

## Gabarito rápido

| # | Resposta |
| --- | --- |
| 1 | `[[3,4,5],[5,6,7],[7,8,9]]` |
| 2 | `[[1,−2,3],[−2,4,−6],[3,−6,9]]` — **simétrica** |
| 3 | P: nula/diagonal · Q: diagonal (`5I₃`) · R: triangular superior · S: antissimétrica · T: simétrica · U: matriz linha |
| 4 | `x = 4`, `y = −4` |
| 5 | `x = −1`, `y = 6` |
| 6 | `a = 3`, `b = ±3`, `c = −3`, `d = −3` |
| 7 | `[[−5,−4,9],[−8,6,−4]]` |
| 8 | `(A+B)ᵀ = Aᵀ+Bᵀ = [[3,3],[−1,4],[2,1]]` |
| 9 | `AB` é 3×3 `[[0,6,5],[7,−3,1],[−4,12,8]]` · `BA` é 2×2 `[[2,8],[8,3]]` · `AB ≠ BA` |
| 10 | `(AB)ᵀ = BᵀAᵀ = [[0,7,−4],[6,−3,12],[5,1,8]]` |
| 11 | `A² = [[3,5],[−5,8]]` · `A³ = [[1,18],[−18,19]]` |
| 12 | `[[3,−1,2],[1,4,−1],[2,5,0]]·[x,y,z]ᵀ = [7,1,−3]ᵀ` |
| 13 | `[[2,−5],[−1,3]]` |
| 14 | `[[−11,2,2],[−4,0,1],[6,−1,−1]]` |
| 15 | Linha nula ao escalonar ⟹ não admite inversa |
| 16 | `(AB)⁻¹ = B⁻¹A⁻¹ = [[−1/3,1/3],[−7/3,4/3]]` · `A⁻¹B⁻¹` é **diferente** |
| 17 | `(x, y) = (3, 2)` |
| 18 | **É ortogonal** · `det(Q) = 1` |
| 19 | **É idempotente** · `A¹⁰⁰ = A` |
| 20 | **É nihilpotente**, índice `p = 3` |
| 21 | a) F · b) V · c) F (`±1`) · d) V · e) F |
| 22 | a) 26 · b) −2 · c) −14 |
| 23 | a) 18 · b) 0 · c) 12 |
| 24 | a) 0 (`L3 = L1+L2`) · b) 0 (linhas iguais) · c) 0 (coluna nula) · d) −24 (triangular) |
| 25 | **72** (`2³ · 9`) |
| 26 | **−57** |
| 27 | **18** |
| 28 | **−5** |
| 29 | a) 10 · 7 = 70 ✓ · b) `det(A⁻¹) = 1/10` ✓ |
| 30 | `[[3/5, −7/10], [−1/5, 2/5]]` |
| 31 | `[[−24,18,5],[20,−15,−4],[−5,4,1]]` |
| 32 | a) SPD `(1,2)` · b) SPI `(3−2t, t)` · c) SI |
| 33 | SPD, `(x,y,z) = (1,2,3)` |
| 34 | **SI** (`0 = 1`) |
| 35 | SPI, `(3−2s+t, s, t)` |
| 36 | SPI, `t·(−1,−1,1)` |
| 37 | `k ≠ 2` ⟹ SPD · `k = 2` ⟹ SPI · nunca SI |
| 38 | a) F · b) F · c) F · d) F · e) V |
| 39 | SSD R$ 400 · RAM R$ 300 · fonte R$ 400 |
| 40 | a) sim · b) `−1/5` · c) SPD · d) SPD, só a trivial |

---

## Onde cada erro aponta

| Se você errou… | Revise |
| --- | --- |
| 1–3 | Lei de formação e os 11 tipos — §1–§2 |
| 4–6 | Simétrica, antissimétrica, igualdade — §2–§3 |
| 7–8 | Adição, escalar, transposta — §4–§5 |
| 9–12 | Compatibilidade e `(AB)ᵀ = BᵀAᵀ` — §6 |
| 13–17 | Operações elementares: escrever `A` ao lado de `I` — §7 |
| 18–21 | Ortogonal, idempotente, nihilpotente — §8 |
| 22–25 | Sarrus e as 11 propriedades — §9–§10 |
| 26–28 | Triangulação e Laplace — §10.1, §11.2 |
| 29 | Binet e `det(A⁻¹)` — §10 |
| 30–31 | **Não esquecer de transpor a matriz dos cofatores** — §11.3 |
| 32–36 | Classificação pela escalonada — §12.3 |
| 37–38 | O determinante não distingue SPI de SI — §12.3 |
| 39–40 | A cadeia `det ≠ 0 <=> inversa <=> SPD` — §0 |
