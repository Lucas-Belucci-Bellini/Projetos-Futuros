# Lista de Exercícios — Álgebra Booleana e Mapa de Karnaugh

**Disciplina:** Sistemas Digitais
**45 exercícios novos**, com resolução passo a passo.

> **Estes exercícios não são os dos slides.** Os dos slides já estão resolvidos em
> [`REVISAO-SD-BOOLEANA-KARNAUGH.md`](REVISAO-SD-BOOLEANA-KARNAUGH.md). Estes aqui
> são construídos do zero para exercitar cada técnica isoladamente e depois
> combinadas.

## Como usar

1. Resolva a **Parte 1** inteira sem olhar nada. Anote onde travou.
2. Só então abra a **Parte 2**. Cada resolução mostra **todos os passos**, não só a
   resposta.
3. O que errar, volte à seção correspondente da revisão (os links estão em cada
   bloco).

**Notação:** `A'` é "A barrado" (complemento). `(AB)'` é a barra longa sobre `AB`,
que **não** é `A'B'`. `Σm(...)` lista os mintermos (linhas com saída 1);
`ΠM(...)` lista os maxtermos (linhas com saída 0).

**Ordem dos bits:** em 3 variáveis, `A` é o bit mais significativo — o mintermo
`m5` é `101`, ou seja `A=1, B=0, C=1`. Em 4 variáveis, `m13` é `1101`
(`A=1, B=1, C=0, D=1`).

> **Sobre "a" resposta.** Vários destes problemas têm **mais de uma solução de
> custo mínimo**. Onde isso acontece a resolução lista todas. Se a sua for
> diferente mas tiver o mesmo número de termos e literais, **está certa** — confira
> pela tabela-verdade.

---

## Índice dos blocos

| Bloco | Exercícios | Assunto | Revisar em |
| --- | --- | --- | --- |
| A | 1–5 | Aritmética e identidades | §3–§4 |
| B | 6–15 | Simplificação algébrica | §5–§6 |
| C | 16–21 | Teoremas de DeMorgan | §8 |
| D | 22–24 | Tabela-verdade → SOP e POS | §9 |
| E | 25–29 | Mapa de Karnaugh, 3 variáveis | §13–§14 |
| F | 30–35 | Mapa de Karnaugh, 4 variáveis | §14–§15 |
| G | 36–39 | Maxtermos e solução POS | §16 |
| H | 40–42 | *Don't care* | §17 |
| I | 43–45 | Problemas completos | §10 |

---

# PARTE 1 — A LISTA

## Bloco A — Aritmética booleana e identidades

**1.** Calcule, justificando cada um pela identidade usada:

  a) `1 + 1 + 1 + 0`  b) `1 · 1 · 0`  c) `A + A' + B`  d) `(A · A') · B + 1`  e) `A · 1 + 0`

**2.** Simplifique, dizendo qual identidade permite cada passo:

  a) `ABC + 1`  b) `(A + B + C) · 0`  c) `AB + AB`  d) `A'' + A`

**3.** Quanto vale `A''''` (quatro complementos)? E `A'''` (três)? Justifique.

**4.** Prove que `A + A'B = A + B` de **duas** formas: por tabela-verdade e
algebricamente.

**5.** Reduza à forma mais simples: `AA' + BB + C · 1 + D · 0`

## Bloco B — Simplificação algébrica

Simplifique ao mínimo. Mostre cada passo e nomeie a regra usada.

**6.** `AB + AB'`
**7.** `A + AB + AB'C`
**8.** `(A + B)(A + B')`
**9.** `AB'C + ABC + A'BC`
**10.** `A'B'C + A'BC + AB'C + ABC`
**11.** `(A + B)(A + C)`
**12.** `AB + A'C + BC`
**13.** `A + A'B'`
**14.** `AB + A'B + AB'`
**15.** `(AB)'(A + B)`

## Bloco C — Teoremas de DeMorgan

Aplique DeMorgan até que **nenhuma barra cubra mais de uma variável**.
Lembre: **uma barra por passo**, começando pela mais externa.

**16.** `(A + B'C)'`
**17.** `((AB)' + C)'`
**18.** `(A'B + C)'`
**19.** `(A + B + C)'`
**20.** `((A + B)'(C + D)')'`
**21.** `((A' + B)(A + C'))'`

## Bloco D — Tabela-verdade → SOP e POS

**22. Somador completo.** Um somador completo recebe `A`, `B` e o vai-um de
entrada `C`, e produz a soma `S` e o vai-um de saída `Co`.

  a) Monte a tabela-verdade das duas saídas.
  b) Escreva o SOP de `S` e o de `Co`.
  c) Simplifique os dois. Um deles **não simplifica** — explique por quê.

**23.** Para `F(A,B,C) = Σm(1,3,4,5,6)`:
  a) Escreva o SOP canônico (um termo por mintermo).
  b) Simplifique.

**24. Qual forma escolher.** Para cada função abaixo, obtenha o **SOP mínimo** e o
**POS mínimo**, conte termos e literais de cada um, e diga qual compensa:

  a) `F(A,B,C,D) = Σm(2,4,6,11,13,15)`
  b) `F(A,B,C,D) = Σm(1,2,3,4,6,7,9,11,12,14)`

## Bloco E — Mapa de Karnaugh, 3 variáveis

Para cada função: **desenhe o mapa**, agrupe, e escreva o SOP mínimo.

**25.** `F(A,B,C) = Σm(0,1,2,5,7)`
**26.** `F(A,B,C) = Σm(1,2,3,4,5,7)`
**27.** `F(A,B,C) = Σm(0,2,4,5,6)`
**28.** `F(A,B,C) = Σm(3,4,5,6,7)`
**29.** `F(A,B,C) = Σm(1,2,4,7)` — *atenção: este tem uma surpresa.*

## Bloco F — Mapa de Karnaugh, 4 variáveis

**30.** `F = Σm(0,2,8,10)`
**31.** `F = Σm(0,1,2,3,8,9,10,11)`
**32.** `F = Σm(0,1,4,5,10,11,14,15)`
**33.** `F = Σm(0,2,5,7,8,10,13,15)`
**34.** `F = Σm(0,1,2,5,8,9,10)`
**35.** `F = Σm(2,3,4,5,10,11,12,13,14,15)`

## Bloco G — Maxtermos e solução POS

**36.** `F(A,B,C) = ΠM(0,2,4,6)` — escreva o POS mínimo.
**37.** `F(A,B,C) = ΠM(0,1,2,4)` — escreva o POS mínimo e o SOP mínimo. Compare.
**38.** `F(A,B,C,D)` vale 0 apenas em `m14` e `m15`. Escreva o POS.
**39.** Escreva em **POS** a função do exercício 33. Confirme que dá a mesma
tabela-verdade do SOP que você achou lá.

## Bloco H — *Don't care*

**40.** `F = Σm(1,3,7,11,15) + d(0,2,5)`
**41. Detector BCD.** Um dígito BCD (0 a 9) entra em `ABCD`. A saída vale 1 quando
o dígito é **maior ou igual a 5**. As combinações 10 a 15 **não existem** em BCD.
Escreva o SOP mínimo.
**42.** `F(A,B,C) = Σm(0,1,5) + d(2,3)`

## Bloco I — Problemas completos

**43.** Dado `Q = (A + B)(A' + C) + AB'`:
  a) Monte a tabela-verdade.
  b) Passe para o K-map e simplifique.
  c) Quantas portas o circuito original usa? E o simplificado?

**44. Detector de primo.** Projete um circuito que recebe um número binário de 3
bits (`A` = mais significativo) e produz 1 quando o número é **primo**.
  a) Tabela-verdade.  b) K-map e SOP mínimo.  c) Desenhe o circuito.

**45. Contagem de custo.** Para `F = Σm(0,1,2,5,8,9,10)` (exercício 34), calcule
o custo em portas do SOP canônico (um termo por mintermo) e do SOP mínimo.
Quantas entradas de porta você economizou?

---
---

# PARTE 2 — RESOLUÇÕES

## Bloco A

### 1.

| | Conta | Resultado | Identidade |
| --- | --- | --- | --- |
| a | `1 + 1 + 1 + 0` | **1** | `A + A = A` e `A + 0 = A`. Some dois a dois: `1+1=1`, `1+1=1`, `1+0=1` |
| b | `1 · 1 · 0` | **0** | `A · 0 = 0`. Qualquer produto com um 0 é 0 |
| c | `A + A' + B` | **1** | `A + A' = 1`, e depois `1 + B = 1` |
| d | `(A · A') · B + 1` | **1** | `A · A' = 0`, logo `0 · B = 0`, e `0 + 1 = 1` |
| e | `A · 1 + 0` | **A** | `A · 1 = A` e `A + 0 = A` |

> Em (c) e (d), a resposta é **1 sem depender de A nem de B**. É o efeito de
> `A + 1 = 1`: o 1 sobrepõe tudo.

### 2.

**a)** `ABC + 1 = 1`

Pela identidade `A + 1 = 1`. O truque é reconhecer que o **"A" da forma padrão
representa o termo inteiro `ABC`**. Não importa o que `ABC` valha — somado a 1, dá 1.

**b)** `(A + B + C) · 0 = 0` — pela identidade `A · 0 = 0`, com `A` = a soma inteira.

**c)** `AB + AB = AB` — pela identidade `A + A = A`, com `A` = `AB`.

**d)** `A'' + A = A + A = A` — primeiro o complemento duplo (`A'' = A`), depois
`A + A = A`.

### 3.

```
A''''  =  A     (quatro complementos = número PAR)
A'''   =  A'    (três complementos = número ÍMPAR)
```

**Por quê:** cada par de complementos se cancela (`A'' = A`). Com quatro, são dois
pares: volta ao original. Com três, cancela um par e sobra um: fica `A'`.
É a mesma lógica da negação nos reais: `−(−(−x)) = −x`.

### 4.

**Por tabela-verdade:**

| A | B | A' | A'B | A + A'B | A + B |
| --- | --- | --- | --- | --- | --- |
| 0 | 0 | 1 | 0 | **0** | **0** |
| 0 | 1 | 1 | 1 | **1** | **1** |
| 1 | 0 | 0 | 0 | **1** | **1** |
| 1 | 1 | 0 | 0 | **1** | **1** |

As duas últimas colunas são idênticas nas quatro linhas. Provado.

**Algebricamente:**

```
A + A'B  =  (A + A')(A + B)      distributiva ao contrário: X + YZ = (X+Y)(X+Z)
         =  1 · (A + B)          pois A + A' = 1
         =  A + B                pois 1 · X = X
```

### 5.

```
AA' + BB + C·1 + D·0
 = 0   + B  + C    + 0        (AA'=0 ; BB=B ; C·1=C ; D·0=0)
 = B + C
```

**Resposta: `B + C`.** `A` e `D` somem completamente da expressão.

---

## Bloco B

### 6. `AB + AB' = A`

```
AB + AB' = A(B + B')      fatorando A
         = A · 1          pois B + B' = 1
         = A
```

> Esta é a operação mais usada em toda a matéria, e é exatamente o que um
> agrupamento de duas células faz no K-map: elimina a variável que muda.

### 7. `A + AB + AB'C = A`

```
A + AB + AB'C
 = A + AB'C            pois A + AB = A  (absorção)
 = A                   absorção de novo, com "B'C" no papel de B
```

**Atalho:** todo termo que **contém `A` como fator** é absorvido por um `A` solto.
Aqui, `AB` e `AB'C` contêm `A`. Sobra `A`.

### 8. `(A + B)(A + B') = A`

```
(A + B)(A + B')
 = AA + AB' + BA + BB'      distributiva (todos contra todos)
 = A  + AB' + AB  + 0       pois AA = A e BB' = 0
 = A(1 + B' + B)            fatorando A
 = A · 1  =  A
```

**Pelo atalho POS:** `(A + X)(A + X') = A + XX' = A + 0 = A`.

### 9. `AB'C + ABC + A'BC = AC + BC`

```
AB'C + ABC + A'BC
 = AC(B' + B) + A'BC       fatorando AC dos dois primeiros
 = AC + A'BC               pois B' + B = 1
 = C(A + A'B)              fatorando C
 = C(A + B)                pela regra A + A'B = A + B
 = AC + BC
```

**Resposta: `AC + BC`** (ou, fatorada, `C(A + B)` — mesma coisa, e com **uma porta a
menos**).

### 10. `A'B'C + A'BC + AB'C + ABC = C`

```
A'B'C + A'BC + AB'C + ABC
 = A'C(B' + B) + AC(B' + B)      fatorando aos pares
 = A'C + AC                      pois B' + B = 1
 = C(A' + A)                     fatorando C
 = C
```

> Os quatro termos cobrem **todas** as combinações de `A` e `B` com `C = 1`. Então
> a saída só depende de `C`. No K-map isso seria um grupo de 4 células.

### 11. `(A + B)(A + C) = A + BC`

```
(A + B)(A + C)
 = AA + AC + BA + BC
 = A + AC + AB + BC        pois AA = A
 = A(1 + C + B) + BC       fatorando A dos três primeiros
 = A · 1 + BC
 = A + BC
```

É a **distributiva da soma sobre o produto** — existe no booleano e **não existe**
nos reais (`x + yz ≠ (x+y)(x+z)` para números).

### 12. `AB + A'C + BC = AB + A'C` — teorema do consenso

O termo `BC` é o **consenso** de `AB` e `A'C`: some-se a variável que aparece
complementada de um lado e direta do outro (`A`), e sobra `BC`. O consenso é sempre
redundante.

```
AB + A'C + BC
 = AB + A'C + BC(A + A')         pois A + A' = 1, pode multiplicar
 = AB + A'C + ABC + A'BC         distribuindo
 = (AB + ABC) + (A'C + A'BC)     reagrupando
 = AB(1 + C) + A'C(1 + B)        fatorando
 = AB + A'C
```

**Conferido por tabela-verdade:** as duas expressões dão a mesma saída nas 8 linhas.

### 13. `A + A'B' = A + B'`

Regra `A + A'X = A + X`, com `X = B'`. **Resposta: `A + B'`.**

### 14. `AB + A'B + AB' = A + B`

```
AB + A'B + AB'
 = B(A + A') + AB'        fatorando B dos dois primeiros
 = B + AB'                pois A + A' = 1
 = B + A                  pela regra X + X'Y = X + Y, com X = B e Y = A
 = A + B
```

### 15. `(AB)'(A + B) = A'B + AB'`

```
(AB)'(A + B)
 = (A' + B')(A + B)           DeMorgan na primeira barra
 = A'A + A'B + B'A + B'B      distribuindo
 = 0 + A'B + AB' + 0          pois A'A = 0 e B'B = 0
 = A'B + AB'
```

**Este é exatamente o XOR:** `A'B + AB' = A ⊕ B`. Faz sentido — a expressão original
diz "nem os dois ao mesmo tempo (`(AB)'`), mas pelo menos um (`A+B`)", que é a
definição de OU-exclusivo. **Uma única porta XOR** substitui o circuito inteiro.

---

## Bloco C — DeMorgan

### 16. `(A + B'C)' = A'B + A'C'`

```
(A + B'C)'
 = A' · (B'C)'          quebra a barra externa: + vira ·
 = A' · (B'' + C')      quebra a barra de B'C: · vira +
 = A' · (B + C')        complemento duplo: B'' = B
 = A'B + A'C'           distribuindo
```

### 17. `((AB)' + C)' = ABC'`

```
((AB)' + C)'
 = (AB)'' · C'          quebra a externa: + vira ·
 = AB · C'              complemento duplo
 = ABC'
```

### 18. `(A'B + C)' = AC' + B'C'`

```
(A'B + C)'
 = (A'B)' · C'          externa
 = (A'' + B') · C'      interna
 = (A + B') · C'        complemento duplo
 = AC' + B'C'
```

### 19. `(A + B + C)' = A'B'C'`

Uma só barra, quebrada em **dois lugares de uma vez** — o que é permitido, porque é
**uma única barra**:

```
(A + B + C)' = A' · B' · C' = A'B'C'
```

> O proibido é quebrar **duas barras diferentes** no mesmo passo. Quebrar **uma**
> barra em vários pontos, como aqui, é legítimo.

### 20. `((A + B)'(C + D)')' = A + B + C + D`

```
((A+B)'(C+D)')'
 = (A+B)'' + (C+D)''     quebra a barra mais externa: · vira +
 = (A+B) + (C+D)         complemento duplo nas duas
 = A + B + C + D
```

> Repare que **nenhuma barra interna foi quebrada** — elas foram **canceladas** pelo
> complemento duplo. Menos trabalho, e é por isso que se começa pela mais externa.

### 21. `((A' + B)(A + C'))' = AB' + A'C`

```
((A' + B)(A + C'))'
 = (A' + B)' + (A + C')'      externa: · vira +
 = (A'' · B') + (A' · C'')    cada interna: + vira ·
 = AB' + A'C                  complementos duplos
```

---

## Bloco D

### 22. Somador completo

**a) Tabela-verdade:**

| A | B | C | S (soma) | Co (vai-um) |
| --- | --- | --- | --- | --- |
| 0 | 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 | 0 |
| 0 | 1 | 0 | 1 | 0 |
| 0 | 1 | 1 | 0 | 1 |
| 1 | 0 | 0 | 1 | 0 |
| 1 | 0 | 1 | 0 | 1 |
| 1 | 1 | 0 | 0 | 1 |
| 1 | 1 | 1 | 1 | 1 |

`S` é 1 quando o **número de entradas em 1 é ímpar**. `Co` é 1 quando **duas ou
mais** entradas são 1.

**b) SOP canônico:**

```
S  = A'B'C + A'BC' + AB'C' + ABC       (mintermos 1, 2, 4, 7)
Co = A'BC + AB'C + ABC' + ABC          (mintermos 3, 5, 6, 7)
```

**c) Simplificação:**

**`Co` simplifica.** Reaproveitando `ABC` três vezes (`X + X = X`):

```
Co = (A'BC + ABC) + (AB'C + ABC) + (ABC' + ABC)
   = BC(A' + A) + AC(B' + B) + AB(C' + C)
   = BC + AC + AB
```

**`Co = AB + AC + BC`** — de 4 termos de 3 literais para 3 termos de 2.

**`S` NÃO simplifica.** No K-map, os mintermos 1, 2, 4, 7 formam um **tabuleiro de
xadrez**: nenhum par é adjacente (todos diferem em **duas** variáveis, não uma).
Sem adjacência, não há agrupamento. O SOP mínimo é o próprio canônico:
**4 termos, 12 literais**.

**A saída real** é escrever em XOR: **`S = A ⊕ B ⊕ C`**. Dois XOR de 2 entradas
resolvem o que 4 ANDs de 3 entradas + 1 OR de 4 entradas fariam. É o exemplo
clássico de função que a álgebra booleana e o K-map **não sabem** simplificar,
porque XOR não é operação booleana nativa.

### 23. `F(A,B,C) = Σm(1,3,4,5,6)`

**a) SOP canônico** (um termo por mintermo):

```
m1 = 001 -> A'B'C      m3 = 011 -> A'BC       m4 = 100 -> AB'C'
m5 = 101 -> AB'C       m6 = 110 -> ABC'

F = A'B'C + A'BC + AB'C' + AB'C + ABC'
```

**b) Simplificando:**

```
F = A'C(B' + B)  +  AB'(C' + C)  +  ABC'     agrupando aos pares
  = A'C + AB' + ABC'
  = A'C + A(B' + BC')                         fatorando A
  = A'C + A(B' + C')                          pois B' + BC' = B' + C'
  = A'C + AB' + AC'
```

**Resposta: `F = A'C + AC' + AB'`** — 3 termos, 6 literais.

**Solução alternativa de custo igual:** `F = B'C + A'C + AC'` (também 3 termos e 6
literais). As duas são mínimas.

### 24. Qual forma escolher

**a) `F = Σm(2,4,6,11,13,15)`** — 6 uns, 10 zeros.

| Forma | Expressão | Custo |
| --- | --- | --- |
| SOP mínimo | `A'CD' + A'BD' + ACD + ABD` | 4 termos, **12 literais** |
| POS mínimo | `(B + C)(A + D')(A' + D)` | 3 termos, **6 literais** |

**O POS vence, com metade dos literais.**

**b) `F = Σm(1,2,3,4,6,7,9,11,12,14)`** — 10 uns, 6 zeros.

| Forma | Expressão | Custo |
| --- | --- | --- |
| SOP mínimo | `B'D + BD' + A'C` | 3 termos, **6 literais** |
| POS mínimo | `(B+C+D)(B'+C+D')(A'+B+D)(A'+B'+D')` | 4 termos, **12 literais** |

**O SOP vence, com metade dos literais.**

**A conclusão que este par ensina:** não existe "o SOP é sempre melhor" nem "menos
uns ⟹ use POS". O que decide é **como os 1s e os 0s se agrupam no mapa**, não
quantos são. Em (a) os 6 uns estão espalhados e os 10 zeros formam blocos grandes;
em (b) é o contrário. **Na dúvida, faça os dois e conte os literais** — leva dois
minutos e pode cortar o circuito pela metade.

---

## Bloco E — K-map de 3 variáveis

### 25. `F = Σm(0,1,2,5,7)`

```
        BC
    \  00  01  11  10
      +---+---+---+---+
  A=0 | 1 | 1 | 0 | 1 |
      +---+---+---+---+
  A=1 | 0 | 1 | 1 | 0 |
      +---+---+---+---+
```

Grupos: `m1+m5` (coluna BC=01) → `B'C`; `m0+m2` (bordas da linha A=0, enrolando) →
`A'C'`; `m5+m7` (linha A=1, colunas 01 e 11) → `AC`.

**`F = B'C + A'C' + AC`** — 3 termos, 6 literais.

**Solução alternativa de mesmo custo:** `F = A'C' + A'B' + AC`.
(Trocando o grupo `m1+m5` por `m0+m1`.) As duas são mínimas.

### 26. `F = Σm(1,2,3,4,5,7)`

```
        BC
    \  00  01  11  10
      +---+---+---+---+
  A=0 | 0 | 1 | 1 | 1 |
      +---+---+---+---+
  A=1 | 1 | 1 | 1 | 0 |
      +---+---+---+---+
```

Grupos: coluna `BC=01` + coluna `BC=11` = 4 células (`m1,m3,m5,m7`) → `C`;
`m2+m3` → `A'B`; `m4+m5` → `AB'`.

**`F = C + A'B + AB'`** — 3 termos, **5 literais** (o `C` sozinho custa 1).

> Repare que `A'B + AB'` é XOR. Então `F = C + (A ⊕ B)`.

### 27. `F = Σm(0,2,4,5,6)`

```
        BC
    \  00  01  11  10
      +---+---+---+---+
  A=0 | 1 | 0 | 0 | 1 |
      +---+---+---+---+
  A=1 | 1 | 1 | 0 | 1 |
      +---+---+---+---+
```

Grupo de 4: colunas `BC=00` e `BC=10` inteiras (`m0,m2,m4,m6`) → nelas `C=0` sempre
→ **`C'`**. Sobra `m5`, que agrupa com `m4` → `AB'`.

**`F = C' + AB'`** — 2 termos, 3 literais.

### 28. `F = Σm(3,4,5,6,7)`

```
        BC
    \  00  01  11  10
      +---+---+---+---+
  A=0 | 0 | 0 | 1 | 0 |
      +---+---+---+---+
  A=1 | 1 | 1 | 1 | 1 |
      +---+---+---+---+
```

A linha `A=1` inteira é um grupo de 4 → **`A`**. Sobra `m3` (`A'BC`), que agrupa com
`m7` → `BC`.

**`F = A + BC`** — 2 termos, 3 literais.

### 29. `F = Σm(1,2,4,7)` — a surpresa

```
        BC
    \  00  01  11  10
      +---+---+---+---+
  A=0 | 0 | 1 | 0 | 1 |
      +---+---+---+---+
  A=1 | 1 | 0 | 1 | 0 |
      +---+---+---+---+
```

**Nenhum par de 1s é adjacente.** É um tabuleiro de xadrez: todo 1 tem 0 em cima,
embaixo e dos dois lados. Não há grupo possível maior que 1 célula.

**`F = A'B'C + A'BC' + AB'C' + ABC`** — o SOP mínimo **é** o canônico: 4 termos, 12
literais. Não há o que simplificar.

**Mas há resposta melhor fora do SOP:** esta é a função **paridade ímpar**, ou seja
**`F = A ⊕ B ⊕ C`** — dois XOR de 2 entradas. É a mesma função `S` do somador
completo (exercício 22).

> **A lição:** quando o K-map dá xadrez, pare de tentar agrupar e verifique se é
> XOR. O mapa não erra — ele simplesmente não representa XOR.

---

## Bloco F — K-map de 4 variáveis

### 30. `F = Σm(0,2,8,10)` — os quatro cantos

```
         CD
  AB \  00  01  11  10
      +---+---+---+---+
   00 | 1 | 0 | 0 | 1 |
      +---+---+---+---+
   01 | 0 | 0 | 0 | 0 |
      +---+---+---+---+
   11 | 0 | 0 | 0 | 0 |
      +---+---+---+---+
   10 | 1 | 0 | 0 | 1 |
      +---+---+---+---+
```

"Dobre os cantos como um guardanapo": os quatro formam **um grupo de 4**.
Nas quatro células, `B = 0` e `D = 0`; `A` e `C` variam.

**`F = B'D'`** — 1 termo, 2 literais. Quatro mintermos viraram um produto de dois.

### 31. `F = Σm(0,1,2,3,8,9,10,11)`

```
         CD
  AB \  00  01  11  10
      +---+---+---+---+
   00 | 1 | 1 | 1 | 1 |
      +---+---+---+---+
   01 | 0 | 0 | 0 | 0 |
      +---+---+---+---+
   11 | 0 | 0 | 0 | 0 |
      +---+---+---+---+
   10 | 1 | 1 | 1 | 1 |
      +---+---+---+---+
```

As linhas `AB=00` e `AB=10` são adjacentes (enrolando topo/base) → **grupo de 8**.
Em todas as 8 células, `B = 0`.

**`F = B'`** — 1 termo, 1 literal. Oito mintermos viraram **uma única variável**.

### 32. `F = Σm(0,1,4,5,10,11,14,15)`

```
         CD
  AB \  00  01  11  10
      +---+---+---+---+
   00 | 1 | 1 | 0 | 0 |
      +---+---+---+---+
   01 | 1 | 1 | 0 | 0 |
      +---+---+---+---+
   11 | 0 | 0 | 1 | 1 |
      +---+---+---+---+
   10 | 0 | 0 | 1 | 1 |
      +---+---+---+---+
```

Bloco superior esquerdo (4 células): `A=0`, `C=0` → `A'C'`.
Bloco inferior direito (4 células): `A=1`, `C=1` → `AC`.

**`F = A'C' + AC`** — 2 termos, 4 literais. É o **XNOR de A e C**: `F = (A ⊕ C)'`.

### 33. `F = Σm(0,2,5,7,8,10,13,15)`

```
         CD
  AB \  00  01  11  10
      +---+---+---+---+
   00 | 1 | 0 | 0 | 1 |
      +---+---+---+---+
   01 | 0 | 1 | 1 | 0 |
      +---+---+---+---+
   11 | 0 | 1 | 1 | 0 |
      +---+---+---+---+
   10 | 1 | 0 | 0 | 1 |
      +---+---+---+---+
```

Os quatro cantos (`m0, m2, m8, m10`): `B=0, D=0` → `B'D'`.
O bloco central (`m5, m7, m13, m15`): `B=1, D=1` → `BD`.

**`F = B'D' + BD`** — 2 termos, 4 literais. **XNOR de B e D.**

### 34. `F = Σm(0,1,2,5,8,9,10)`

```
         CD
  AB \  00  01  11  10
      +---+---+---+---+
   00 | 1 | 1 | 0 | 1 |
      +---+---+---+---+
   01 | 0 | 1 | 0 | 0 |
      +---+---+---+---+
   11 | 0 | 0 | 0 | 0 |
      +---+---+---+---+
   10 | 1 | 1 | 0 | 1 |
      +---+---+---+---+
```

Grupos:
- `m0, m2, m8, m10` (os quatro cantos) → `B'D'`
- `m0, m1, m8, m9` (colunas CD=00 e CD=01 nas linhas AB=00 e AB=10) → `B'C'`
- `m1, m5` (coluna CD=01, linhas AB=00 e AB=01) → `A'C'D`

**`F = B'D' + B'C' + A'C'D`** — 3 termos, 7 literais.

> Note que `m0` e `m1` foram **reutilizados** em mais de um grupo. Isso é desejável
> — é o que permite que os grupos fiquem grandes.

### 35. `F = Σm(2,3,4,5,10,11,12,13,14,15)`

```
         CD
  AB \  00  01  11  10
      +---+---+---+---+
   00 | 0 | 0 | 1 | 1 |
      +---+---+---+---+
   01 | 1 | 1 | 0 | 0 |
      +---+---+---+---+
   11 | 1 | 1 | 1 | 1 |
      +---+---+---+---+
   10 | 0 | 0 | 1 | 1 |
      +---+---+---+---+
```

Grupos de 4:
- `m2, m3, m10, m11` (colunas CD=11 e CD=10, linhas AB=00 e AB=10) → `B'C`
- `m4, m5, m12, m13` (colunas CD=00 e CD=01, linhas AB=01 e AB=11) → `BC'`
- falta cobrir `m14, m15`: agrupam com `m10, m11` → `AC`, **ou** com `m12, m13` → `AB`

**Duas soluções de custo mínimo, ambas com 3 termos e 6 literais:**

```
F = B'C + BC' + AC        ou        F = B'C + BC' + AB
```

Qualquer uma está correta.

---

## Bloco G — Maxtermos e POS

### 36. `F(A,B,C) = ΠM(0,2,4,6)`

`F` vale **0** nos mintermos 0, 2, 4, 6 — ou seja, vale **1** em 1, 3, 5, 7.

Mapeando os **zeros** e agrupando: `m0, m2, m4, m6` formam um grupo de 4 no qual
`C = 0` em todas.

- Valor binário do grupo: `C = 0`
- **Complementando:** `C = 1` → termo soma `C`

**`F = C`** — um único termo. (E, de fato, os mintermos 1, 3, 5, 7 são exatamente
aqueles em que `C = 1`.)

### 37. `F(A,B,C) = ΠM(0,1,2,4)`

Zeros em `m0 (000)`, `m1 (001)`, `m2 (010)`, `m4 (100)`. Uns em 3, 5, 6, 7.

**Agrupando os ZEROS:**
- `m0 + m1` → `A=0, B=0` → complementando: `(A + B)`
- `m0 + m2` → `A=0, C=0` → complementando: `(A + C)`
- `m0 + m4` → `B=0, C=0` → complementando: `(B + C)`

**POS: `F = (A + B)(A + C)(B + C)`** — 3 termos, 6 literais.

**SOP** (agrupando os uns `m3, m5, m6, m7`): **`F = AB + AC + BC`** — 3 termos, 6
literais.

**Comparação:** custo idêntico. Esta é a **função maioria** (saída 1 quando pelo
menos duas entradas são 1) — a mesma do incinerador "dois de três" e do vai-um do
somador. As duas formas são igualmente boas aqui; a escolha vira questão de qual
tipo de porta está disponível.

### 38. `F(A,B,C,D)` vale 0 só em `m14` e `m15`

`m14 = 1110`, `m15 = 1111`. Agrupando os dois zeros: `A=1, B=1, C=1`, `D` varia.

- Valor do grupo: `A=1, B=1, C=1`
- **Complementando:** `A=0, B=0, C=0` → termo soma `(A' + B' + C')`

**`F = A' + B' + C'`** — um único termo soma, 3 literais.

**Conferência por DeMorgan:** `F' = ABC`, logo `F = (ABC)' = A' + B' + C'` ✓

### 39. Exercício 33 em POS

`F = Σm(0,2,5,7,8,10,13,15)`. Os **zeros** estão em
`m1, m3, m4, m6, m9, m11, m12, m14`.

```
         CD                      (mapa dos ZEROS)
  AB \  00  01  11  10
      +---+---+---+---+
   00 | . | 0 | 0 | . |
      +---+---+---+---+
   01 | 0 | . | . | 0 |
      +---+---+---+---+
   11 | 0 | . | . | 0 |
      +---+---+---+---+
   10 | . | 0 | 0 | . |
      +---+---+---+---+
```

Dois grupos de 4:
- `m1, m3, m9, m11`: `B=0, D=1` → complementando → `(B + D')`
- `m4, m6, m12, m14`: `B=1, D=0` → complementando → `(B' + D)`

**POS: `F = (B + D')(B' + D)`** — 2 termos, 4 literais.

**Comparando com o SOP do exercício 33** (`B'D' + BD`): mesmo custo — 2 termos, 4
literais. Coerente, já que a função é o XNOR de `B` e `D`, simétrico nas duas
formas.

**Conferência:** `(B + D')(B' + D) = BB' + BD + D'B' + D'D = 0 + BD + B'D' + 0 = BD + B'D'` ✓
Bate com o SOP.

---

## Bloco H — *Don't care*

### 40. `F = Σm(1,3,7,11,15) + d(0,2,5)`

```
         CD
  AB \  00  01  11  10
      +---+---+---+---+
   00 | X | 1 | 1 | X |
      +---+---+---+---+
   01 | 0 | X | 1 | 0 |
      +---+---+---+---+
   11 | 0 | 0 | 1 | 0 |
      +---+---+---+---+
   10 | 0 | 0 | 1 | 0 |
      +---+---+---+---+
```

A coluna `CD=11` inteira (`m3, m7, m11, m15`) é um grupo de 4 → `CD`.
Falta cobrir `m1`. Usando os X:

- `m0, m1, m2, m3` (linha AB=00 inteira, com X em `m0` e `m2`) → `A'B'`, **ou**
- `m1, m3, m5, m7` (colunas CD=01 e CD=11, linhas AB=00 e AB=01, com X em `m5`) → `A'D`

**Duas soluções mínimas, ambas com 2 termos e 4 literais:**

```
F = CD + A'B'        ou        F = CD + A'D
```

> O X em `m0`/`m2`/`m5` **nunca precisou virar 1 sozinho** — ele só entrou para
> deixar um grupo maior. Note que o X em `m5` é usado numa solução e ignorado na
> outra. É exatamente o comportamento esperado: **inclua o X só se ele aumentar o
> grupo.**

### 41. Detector BCD ≥ 5

Dígitos 5 a 9 → `m5, m6, m7, m8, m9` são 1. Os dígitos 0 a 4 são 0. As combinações
10 a 15 **não existem em BCD** → `d(10,11,12,13,14,15)`.

```
         CD
  AB \  00  01  11  10
      +---+---+---+---+
   00 | 0 | 0 | 0 | 0 |
      +---+---+---+---+
   01 | 0 | 1 | 1 | 1 |
      +---+---+---+---+
   11 | X | X | X | X |
      +---+---+---+---+
   10 | 1 | 1 | X | X |
      +---+---+---+---+
```

Grupos:
- Linhas `AB=11` e `AB=10` inteiras (8 células: `m8, m9` reais + 6 X) → `A`
- `m5, m7, m13, m15` (com dois X) → `BD`
- `m6, m7, m14, m15` (com dois X) → `BC`

**`F = A + BD + BC`** — 3 termos, 5 literais.

> Sem os *don't care*, a mesma função exigiria bem mais termos. **Os seis X
> economizaram metade do circuito** — e eles "custam" nada, porque essas entradas
> jamais aparecem num dígito BCD válido.

### 42. `F(A,B,C) = Σm(0,1,5) + d(2,3)`

```
        BC
    \  00  01  11  10
      +---+---+---+---+
  A=0 | 1 | 1 | X | X |
      +---+---+---+---+
  A=1 | 0 | 1 | 0 | 0 |
      +---+---+---+---+
```

- A linha `A=0` inteira (`m0, m1` reais + `m2, m3` como X) → grupo de 4 → **`A'`**
- `m1 + m5` (coluna `BC=01`) → **`B'C`**

**`F = A' + B'C`** — 2 termos, 3 literais.

---

## Bloco I — Problemas completos

### 43. `Q = (A + B)(A' + C) + AB'`

**a) Tabela-verdade.** Avalie parte por parte:

| A | B | C | A+B | A'+C | (A+B)(A'+C) | AB' | **Q** |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | 0 | 0 | 0 | 1 | 0 | 0 | **0** |
| 0 | 0 | 1 | 0 | 1 | 0 | 0 | **0** |
| 0 | 1 | 0 | 1 | 1 | 1 | 0 | **1** |
| 0 | 1 | 1 | 1 | 1 | 1 | 0 | **1** |
| 1 | 0 | 0 | 1 | 0 | 0 | 1 | **1** |
| 1 | 0 | 1 | 1 | 1 | 1 | 1 | **1** |
| 1 | 1 | 0 | 1 | 0 | 0 | 0 | **0** |
| 1 | 1 | 1 | 1 | 1 | 1 | 0 | **1** |

`Q = Σm(2,3,4,5,7)`

**b) K-map:**

```
        BC
    \  00  01  11  10
      +---+---+---+---+
  A=0 | 0 | 0 | 1 | 1 |
      +---+---+---+---+
  A=1 | 1 | 1 | 1 | 0 |
      +---+---+---+---+
```

Grupos: `m2+m3` → `A'B`; `m4+m5` → `AB'`; `m5+m7` → `AC`.

**`Q = A'B + AB' + AC`** — 3 termos, 6 literais.
**Alternativa de mesmo custo:** `Q = A'B + AB' + BC`.

> `A'B + AB'` é XOR, então também vale **`Q = (A ⊕ B) + AC`**.

**c) Contagem de portas:**

| Circuito | Portas |
| --- | --- |
| Original `(A+B)(A'+C) + AB'` | 2 OR(2) + 2 AND(2) + 1 OR(2) + 2 NOT = **7** |
| Simplificado `A'B + AB' + AC` | 3 AND(2) + 1 OR(3) + 2 NOT = **6** |
| Com XOR: `(A ⊕ B) + AC` | 1 XOR + 1 AND(2) + 1 OR(2) = **3** |

A forma com XOR usa **menos da metade** das portas da original.

### 44. Detector de primo em 3 bits

**a) Tabela-verdade.** Primos entre 0 e 7: **2, 3, 5, 7**.
(0 e 1 não são primos por definição; 4 e 6 são pares maiores que 2.)

| A | B | C | nº | primo? |
| --- | --- | --- | --- | --- |
| 0 | 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 | 0 |
| 0 | 1 | 0 | 2 | **1** |
| 0 | 1 | 1 | 3 | **1** |
| 1 | 0 | 0 | 4 | 0 |
| 1 | 0 | 1 | 5 | **1** |
| 1 | 1 | 0 | 6 | 0 |
| 1 | 1 | 1 | 7 | **1** |

`F = Σm(2,3,5,7)`

**b) K-map:**

```
        BC
    \  00  01  11  10
      +---+---+---+---+
  A=0 | 0 | 0 | 1 | 1 |
      +---+---+---+---+
  A=1 | 0 | 1 | 1 | 0 |
      +---+---+---+---+
```

Grupos: `m2 + m3` (linha A=0, colunas 11 e 10) → `A=0, B=1` → `A'B`;
`m5 + m7` (linha A=1, colunas 01 e 11) → `A=1, C=1` → `AC`.

**`F = A'B + AC`** — 2 termos, 4 literais.

**c) Circuito:** 2 inversores (para `A'`), 2 portas AND de 2 entradas, 1 porta OR de
2 entradas.

```
  A ──┬──[NOT]──┐
      │         ├──[AND]──┐
  B ──┼─────────┘         ├──[OR]── F
      │                   │
      └─────────┐         │
  C ────────────┼──[AND]──┘
                │
                (A e C)
```

### 45. Contagem de custo para `F = Σm(0,1,2,5,8,9,10)`

**SOP canônico** — 7 mintermos, cada um um AND de 4 entradas, tudo num OR de 7:

| | Portas | Entradas totais |
| --- | --- | --- |
| AND de 4 entradas | 7 | 28 |
| OR de 7 entradas | 1 | 7 |
| NOT | 4 | 4 |
| **Total** | **12 portas** | **39 entradas** |

**SOP mínimo** `F = B'D' + B'C' + A'C'D` (do exercício 34):

| | Portas | Entradas totais |
| --- | --- | --- |
| AND de 2 entradas | 2 | 4 |
| AND de 3 entradas | 1 | 3 |
| OR de 3 entradas | 1 | 3 |
| NOT | 4 | 4 |
| **Total** | **8 portas** | **14 entradas** |

**Economia: 4 portas e 25 entradas de porta** — uma redução de **64%** no número de
entradas. Isso é menos silício, menos consumo, menos atraso de propagação e maior
confiabilidade: exatamente o que a definição de "menor custo" do K-map persegue.

---

## Gabarito rápido

Para conferir sem ler a resolução inteira.

| # | Resposta |
| --- | --- |
| 1 | a) 1 · b) 0 · c) 1 · d) 1 · e) A |
| 2 | a) 1 · b) 0 · c) AB · d) A |
| 3 | `A'''' = A` · `A''' = A'` |
| 4 | — (demonstração) |
| 5 | `B + C` |
| 6 | `A` |
| 7 | `A` |
| 8 | `A` |
| 9 | `AC + BC` |
| 10 | `C` |
| 11 | `A + BC` |
| 12 | `AB + A'C` |
| 13 | `A + B'` |
| 14 | `A + B` |
| 15 | `A'B + AB'` (= `A ⊕ B`) |
| 16 | `A'B + A'C'` |
| 17 | `ABC'` |
| 18 | `AC' + B'C'` |
| 19 | `A'B'C'` |
| 20 | `A + B + C + D` |
| 21 | `AB' + A'C` |
| 22 | `S` não simplifica (= `A ⊕ B ⊕ C`) · `Co = AB + AC + BC` |
| 23 | `A'C + AC' + AB'` **ou** `B'C + A'C + AC'` |
| 24 | a) POS vence: `(B+C)(A+D')(A'+D)`, 6 lit. vs 12 · b) SOP vence: `B'D + BD' + A'C`, 6 lit. vs 12 |
| 25 | `B'C + A'C' + AC` **ou** `A'C' + A'B' + AC` |
| 26 | `C + A'B + AB'` |
| 27 | `C' + AB'` |
| 28 | `A + BC` |
| 29 | Não simplifica: `A'B'C + A'BC' + AB'C' + ABC` (= `A ⊕ B ⊕ C`) |
| 30 | `B'D'` |
| 31 | `B'` |
| 32 | `A'C' + AC` |
| 33 | `B'D' + BD` |
| 34 | `B'D' + B'C' + A'C'D` |
| 35 | `B'C + BC' + AC` **ou** `B'C + BC' + AB` |
| 36 | `C` |
| 37 | POS `(A+B)(A+C)(B+C)` · SOP `AB + AC + BC` — custo igual |
| 38 | `A' + B' + C'` |
| 39 | `(B + D')(B' + D)` |
| 40 | `CD + A'B'` **ou** `CD + A'D` |
| 41 | `A + BD + BC` |
| 42 | `A' + B'C` |
| 43 | `A'B + AB' + AC` **ou** `A'B + AB' + BC` · 7 portas → 6 (ou 3 com XOR) |
| 44 | `A'B + AC` |
| 45 | 12 portas / 39 entradas → 8 portas / 14 entradas (−64%) |

---

## Onde cada erro aponta

| Se você errou… | Revise |
| --- | --- |
| 1–5 | As 8 identidades — §4 da revisão |
| 6–15 | Regras exclusivas e fatoração — §6 |
| 16–21 | Quebra de barra, uma por vez — §8.3 |
| 22–24 | SOP vs POS: 1s ou 0s — §9 |
| 25–29 | Código Gray e agrupamento — §13–§14 |
| 30–35 | Bordas e cantos: o mapa enrola — §15 |
| 36–39 | No POS o endereço é **complementado** — §16.1 |
| 40–42 | X só entra se **aumentar** o grupo — §17 |
| 43–45 | O fluxo completo — §10 |
