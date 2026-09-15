# Sistemas Digitais — Revisão e Gabarito Comentado

> Baseado na prova **"Prova - Sistemas Digitais-4"** (15 questões, 6 páginas).
> Cada questão foi **resolvida do zero** e conferida contra as alternativas do
> enunciado original.

---

## Parte 1 — Álgebra Booleana

Duas únicas variáveis possíveis: **0** (falso, desligado) e **1** (verdadeiro,
ligado). Três operadores:

| Símbolo | Nome | Significado |
|---|---|---|
| `+` | OU (OR) | 1 se **qualquer** entrada for 1 |
| `·` | E (AND) | 1 só se **todas** forem 1 |
| `A'` | NÃO (NOT) | inverte o valor |

### As leis que você precisa saber de cor

| Lei | Soma (OU) | Produto (E) |
|---|---|---|
| **Identidade** | `A + 0 = A` | `A · 1 = A` |
| **Elemento nulo** | `A + 1 = 1` | `A · 0 = 0` |
| **Idempotência** | `A + A = A` | `A · A = A` |
| **Complemento** | `A + A' = 1` | `A · A' = 0` |
| **Absorção** | `A + AB = A` | `A(A + B) = A` |
| **Distributiva** | `A + BC = (A+B)(A+C)` | `A(B + C) = AB + AC` |

**Complemento duplo:** `(A')' = A`

### As duas que mais caem em prova

```
  A + A' = 1        ← "ou é, ou não é" — sempre verdadeiro
  A · A' = 0        ← "é e não é ao mesmo tempo" — impossível
```

Quase toda questão de simplificação do bloco 1 se resolve reconhecendo uma
dessas duas dentro da expressão.

### A distributiva "invertida" — a que engana

```
  (A + B)(A + C) = A + BC
```

Parece errada porque no Ensino Médio a distributiva só funciona com `·` sobre
`+`. Na álgebra booleana ela vale **nos dois sentidos**. É exatamente o que a
questão 2 cobra.

---

## Parte 2 — Teorema de DeMorgan

```
  (A · B)' = A' + B'     nega o produto  →  vira SOMA dos complementos
  (A + B)' = A' · B'     nega a soma     →  vira PRODUTO dos complementos
```

### Regra prática em 3 passos

1. **Quebre a barra** (remova a negação externa).
2. **Troque o operador**: `·` vira `+`, `+` vira `·`.
3. **Negue cada termo** de dentro.

Vale para qualquer quantidade de variáveis:

```
  (A · B · C)' = A' + B' + C'
  (A + B + C)' = A' · B' · C'
```

### Quando há negação dentro de negação

É o caso das questões 9 e 10. Aplique DeMorgan na camada **de fora** primeiro,
e lembre que `(X')' = X` cancela:

```
  ((A + B)' + C)'
  = ((A+B)')' · C'      ← DeMorgan na camada externa
  = (A + B) · C'        ← negação dupla cancela
```

---

## Parte 3 — Mapas de Karnaugh

Reorganiza a tabela-verdade numa grade onde **células vizinhas diferem em
apenas uma variável**. Isso deixa visível o que a álgebra esconde.

### A ordem das colunas NÃO é 00, 01, 10, 11

É **código Gray**: `00, 01, 11, 10`. Se você montar na ordem binária normal, os
agrupamentos saem errados. Este é o erro nº 1 em prova.

```
  Mapa de 3 variáveis (A nas linhas, BC nas colunas):

           BC=00   BC=01   BC=11   BC=10
   A=0  │   m0      m1      m3      m2
   A=1  │   m4      m5      m7      m6
```

### Regras de agrupamento

| Regra | Detalhe |
|---|---|
| Tamanho | Só potências de 2: **1, 2, 4, 8, 16** |
| Formato | Retângulos — nunca em L ou diagonal |
| Tamanho do grupo | **Sempre o maior possível** (grupo maior = termo menor) |
| Sobreposição | Grupos **podem** se sobrepor |
| Bordas | O mapa "dá a volta": a 1ª e a última coluna são vizinhas |
| Cobertura | Todo `1` precisa estar em **pelo menos um** grupo |

### Como ler o resultado de um grupo

Dentro do grupo, olhe cada variável:

- **Mudou de valor** (0 e 1 aparecem) → a variável **some** do termo
- **Ficou sempre 1** → entra como `A`
- **Ficou sempre 0** → entra como `A'`

Um grupo de 2 elimina 1 variável. Um grupo de 4 elimina 2. Um grupo de 8
elimina 3.

---

# Gabarito comentado

## Bloco 1 — Álgebra Booleana (1 a 5)

### Questão 1 — Resposta: **A)** `A`

```
  (A + 0)(A + A')
  = A · 1            ← A+0=A (identidade) e A+A'=1 (complemento)
  = A                ← A·1=A (identidade)
```

> ⚠️ **Atenção — havia um erro na versão que você tinha.** A resolução que você
> me passou começava dizendo *"Alternativa C — 0"* e só depois se corrigia para
> A. O valor correto é **A**, e a alternativa A do enunciado é exatamente `A`.
> Marque **A**.

### Questão 2 — Resposta: **B)** `A`

```
  (A + B)(A + B')
  = A + (B · B')     ← distributiva invertida: (X+Y)(X+Z) = X + YZ
  = A + 0            ← B·B' = 0 (complemento)
  = A
```

Se você não lembrar da distributiva invertida, expanda tudo:
`AA + AB' + AB + BB'` → `A + AB' + AB + 0` → `A(1 + B' + B)` → `A · 1` = `A`.

### Questão 3 — Resposta: **A)** `A`

```
  AB + AB'
  = A(B + B')        ← fatora A
  = A · 1            ← B+B' = 1
  = A
```

### Questão 4 — Resposta: **C)** `1`

```
  (A + A')(B + B')
  = 1 · 1            ← cada parêntese é um complemento completo
  = 1
```

### Questão 5 — Resposta: **A)** `A`

```
  (A · 1) + (A' · 0)
  = A + 0            ← A·1=A  e  A'·0=0
  = A
```

> O `A'` é isca. Multiplicado por 0, some — não importa o que ele vale.

---

## Bloco 2 — DeMorgan (6 a 10)

### Questão 6 — Resposta: **C)** `A' + B'`

Aplicação direta: `(A · B)' = A' + B'`.

> A alternativa A (`A' · B'`) é a pegadinha: é o resultado de negar a **soma**,
> não o produto.

### Questão 7 — Resposta: **B)** `A' + B' + C'`

Mesma lei, estendida: `(A · B · C)' = A' + B' + C'`.

### Questão 8 — Resposta: **B)** `A' · B' · C`

```
  (A + B)' · C
  = (A' · B') · C    ← DeMorgan só no parêntese negado
  = A'B'C
```

> O `C` está **fora** da negação. Não se mexe nele.

### Questão 9 — Resposta: **E)** `(A + B) · C'`

```
  ((A + B)' + C)'
  = ((A+B)')' · C'   ← DeMorgan na camada de fora: soma vira produto
  = (A + B) · C'     ← negação dupla cancela
```

### Questão 10 — Resposta: **A)** `A · B + C'`

```
  ((A · B)' · C)'
  = ((A·B)')' + C'   ← DeMorgan externo: produto vira soma
  = AB + C'          ← negação dupla cancela
```

---

## Bloco 3 — Mapas de Karnaugh (11 a 15)

### Questão 11 — Resposta: **B)** `F = A' + B`

Tabela: `00→1, 01→1, 10→0, 11→1`

```
          B=0   B=1
   A=0  │  1     1        ← linha inteira = 1  →  grupo de 2  →  A'
   A=1  │  0     1
                 ↑
          coluna B=1 = 1  →  grupo de 2  →  B
```

Dois grupos de 2, sobrepondo na célula `A=0,B=1`:
**`F = A' + B`**

Conferência rápida: só `A=1,B=0` dá 0. Em `A'+B`: `A'=0` e `B=0` → `0`. ✓

### Questão 12 — Resposta: **C)** `F = A'B + C`

Mintermos com F=1: **m1, m2, m3, m5, m7**

```
           BC=00   BC=01   BC=11   BC=10
   A=0  │    0       1       1       1
   A=1  │    0       1       1       0
                 └───────────┘
              grupo de 4 (C=1)  →  C
```

- **Grupo de 4** — as colunas `BC=01` e `BC=11` inteiras (m1,m3,m5,m7).
  A some, B some, **C fica sempre 1** → termo `C`
- **Sobrou m2** (A=0,B=1,C=0). Agrupa com m3 (A=0,B=1,C=1):
  A sempre 0, B sempre 1, C muda → termo `A'B`

**`F = A'B + C`**

> **Por que não D (`B + C`)?** Testa em `A=1,B=1,C=0` (m6): `B+C` daria 1, mas
> a tabela diz **0**. Alternativa eliminada. Sempre teste a alternativa
> suspeita na linha em que a tabela dá 0 — é o jeito mais rápido de matar
> distratores.

### Questão 13 — Resposta: **E)** `F = A'D' + AD`

Não precisa nem desenhar o mapa — olhe o padrão da tabela:

| | quando `D=0` | quando `D=1` |
|---|---|---|
| **A=0** | F = 1 | F = 0 |
| **A=1** | F = 0 | F = 1 |

`B` e `C` **não alteram nada** — variam nas 16 linhas e a saída só acompanha
A e D. Logo:

**`F = A'D' + AD`**

> Isso é a função **XNOR** (equivalência): vale 1 quando A e D são iguais.
> Reconhecer esse padrão economiza o mapa inteiro de 4 variáveis.

### Questão 14 — Resposta: **B)** `F = B`

Saída 1 em `(A=0,B=1)` e `(A=1,B=1)`:

```
  F = A'B + AB
    = B(A' + A)      ← fatora B
    = B · 1
    = B
```

```
          B=0   B=1
   A=0  │  0     1     ← coluna B=1 inteira  →  grupo de 2  →  B
   A=1  │  0     1
```

A variável `A` muda dentro do grupo, então some. **`F = B`**

### Questão 15 — Resposta: **D)** `F = AB + AC + BC`

"Pelo menos duas variáveis iguais a 1" → mintermos **m3, m5, m6, m7**:

| A | B | C | F | por quê |
|---|---|---|---|---|
| 0 | 1 | 1 | 1 | B e C |
| 1 | 0 | 1 | 1 | A e C |
| 1 | 1 | 0 | 1 | A e B |
| 1 | 1 | 1 | 1 | as três |

Três grupos de 2, um para cada par:

**`F = AB + AC + BC`**

> Essa é a **função maioria** (*majority*), o núcleo de um somador completo
> (o bit de carry-out). Vale reconhecer de vista.

---

# Gabarito final

| Questão | Alternativa | Resultado |
|:---:|:---:|---|
| 1 | **A** | `A` |
| 2 | **B** | `A` |
| 3 | **A** | `A` |
| 4 | **C** | `1` |
| 5 | **A** | `A` |
| 6 | **C** | `A' + B'` |
| 7 | **B** | `A' + B' + C'` |
| 8 | **B** | `A'B'C` |
| 9 | **E** | `(A + B)C'` |
| 10 | **A** | `AB + C'` |
| 11 | **B** | `F = A' + B` |
| 12 | **C** | `F = A'B + C` |
| 13 | **E** | `F = A'D' + AD` |
| 14 | **B** | `F = B` |
| 15 | **D** | `F = AB + AC + BC` |

**Distribuição:** A ×5 · B ×5 · C ×3 · E ×2 · D ×1

---

## Os 6 erros que mais custam ponto

1. **Montar o mapa de Karnaugh na ordem binária** (`00,01,10,11`) em vez de
   Gray (`00,01,11,10`). Erra todos os agrupamentos.
2. **Confundir as duas leis de DeMorgan.** Memorize pelo formato: *a barra
   quebra e o sinal troca*.
3. **Aplicar DeMorgan em termo que está fora da negação** (questão 8).
4. **Esquecer que o mapa dá a volta** — 1ª e última coluna são adjacentes.
5. **Fazer grupos pequenos.** Grupo maior = termo com menos variáveis =
   expressão mais simples. Sempre busque o maior.
6. **Não testar a alternativa na linha em que F=0.** É o teste mais rápido
   para eliminar distratores (foi o que matou a alternativa D na questão 12).

## Como treinar em 20 minutos

1. Monte a tabela-verdade de `F = AB + AC + BC` e confirme que ela bate com a
   questão 15. Isso treina o caminho inverso, que também cai.
2. Aplique DeMorgan em `((A·B)' + (C+D)')'` — três camadas. Se acertar, o
   bloco 2 está resolvido.
3. Desenhe o mapa de 4 variáveis da questão 13 completo e confirme
   visualmente que B e C somem.
