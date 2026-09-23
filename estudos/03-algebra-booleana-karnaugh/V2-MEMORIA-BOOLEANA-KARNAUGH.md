# Álgebra Booleana e Karnaugh — V2: Memória, Pegadinhas e Recuperação

> **Objetivo desta versão:** estudar para a prova sem depender de lembrar tudo de uma vez. Cada regra é transformada em uma sequência de memória: **ERRO → CONSEQUÊNCIA → REGRA → RESULTADO**.

## Como estudar este MD

1. Leia somente a **Regra**.
2. Cubra o Resultado e tente lembrar.
3. Leia o **Erro** como uma pegadinha que você não quer repetir.
4. Faça o exemplo sem olhar.
5. Só então confira o resultado.

> **Frase de segurança:** “Eu não preciso lembrar tudo. Preciso reconhecer a pegadinha e aplicar a regra.”

## Modo “DEU MERDA”

A ideia é criar uma memória forte a partir de um **erro fictício de exercício**, não de culpa real. O cérebro encontra o erro → percebe a consequência → recupera a regra → chega ao resultado.

| ERRO | CONSEQUÊNCIA | REGRA | RESULTADO |
|---|---|---|---|
| Tratar + como soma normal | Tentar calcular 1+1=2 | Booleano usa regras próprias | A+0=A |
| Esquecer que A e A' cobrem os dois estados | Não enxergar simplificação | A+A'=1 e A·A'=0 | A·A'=0 |
| Expandir tudo sem necessidade | Conta longa | A+A·B=A | O termo menor absorve o maior |
| Achar que a forma `(A+B)(A+C)` funciona como álgebra comum | Chegar a resultado errado | `(A+B)(A+C)=A+BC` | É uma identidade booleana |
| Só negar variáveis | Manter o operador original | Troque +↔· e negue cada termo | (A+B)'=A'B' |
| Continuar negando | Inverter demais | Duas negações se cancelam | (A')'=A |
| Confundir com OR | Aceitar 11 como saída 1 | XOR vale 1 quando são diferentes | A'B+AB' |
| Confundir com XOR | Inverter a tabela | XNOR vale 1 quando são iguais | AB+A'B' |
| Montar termo pelo valor errado | Variável com complemento invertido | Na linha F=1, bit 0 vira variável negada e bit 1 normal | 001 → A'B'C |
| Usar regra de SOP nos zeros | Sinal errado no maxtermo | Na linha F=0, bit 0 vira variável normal e bit 1 complementada | 101 → (A'+B+C') |
| Colocar colunas em ordem binária | Perder adjacência | Use Gray 00,01,11,10 | Bordas também podem ser vizinhas |
| Criar grupo de 3 células | Grupo inválido | Tamanhos válidos: 1,2,4,8,16 | Grupo maior elimina mais variáveis |

## 1. Regras que você precisa reconhecer rápido

### 1. Identidade

**Erro:** Tratar + como soma normal

**Consequência:** Tentar calcular 1+1=2

**Regra:** Booleano usa regras próprias

**Resultado:** A+0=A

### 2. Complemento

**Erro:** Esquecer que A e A' cobrem os dois estados

**Consequência:** Não enxergar simplificação

**Regra:** A+A'=1 e A·A'=0

**Resultado:** A·A'=0

### 3. Absorção

**Erro:** Expandir tudo sem necessidade

**Consequência:** Conta longa

**Regra:** A+A·B=A

**Resultado:** O termo menor absorve o maior

### 4. Distributiva especial

**Erro:** Achar que a forma `(A+B)(A+C)` funciona como álgebra comum

**Consequência:** Chegar a resultado errado

**Regra:** `(A+B)(A+C)=A+BC`

**Resultado:** É uma identidade booleana

### 5. DeMorgan

**Erro:** Só negar variáveis

**Consequência:** Manter o operador original

**Regra:** Troque +↔· e negue cada termo

**Resultado:** (A+B)'=A'B'

### 6. Dupla negação

**Erro:** Continuar negando

**Consequência:** Inverter demais

**Regra:** Duas negações se cancelam

**Resultado:** (A')'=A

### 7. XOR

**Erro:** Confundir com OR

**Consequência:** Aceitar 11 como saída 1

**Regra:** XOR vale 1 quando são diferentes

**Resultado:** A'B+AB'

### 8. XNOR

**Erro:** Confundir com XOR

**Consequência:** Inverter a tabela

**Regra:** XNOR vale 1 quando são iguais

**Resultado:** AB+A'B'

### 9. SOP

**Erro:** Montar termo pelo valor errado

**Consequência:** Variável com complemento invertido

**Regra:** Na linha F=1, bit 0 vira variável negada e bit 1 normal

**Resultado:** 001 → A'B'C

### 10. POS

**Erro:** Usar regra de SOP nos zeros

**Consequência:** Sinal errado no maxtermo

**Regra:** Na linha F=0, bit 0 vira variável normal e bit 1 complementada

**Resultado:** 101 → (A'+B+C')

### 11. K-map

**Erro:** Colocar colunas em ordem binária

**Consequência:** Perder adjacência

**Regra:** Use Gray 00,01,11,10

**Resultado:** Bordas também podem ser vizinhas

### 12. Grupo

**Erro:** Criar grupo de 3 células

**Consequência:** Grupo inválido

**Regra:** Tamanhos válidos: 1,2,4,8,16

**Resultado:** Grupo maior elimina mais variáveis

### 13. Isolado

**Erro:** Forçar agrupamento

**Consequência:** Mudar a função sem perceber

**Regra:** Se não é adjacente, permanece termo

**Resultado:** m1 e m7 não formam par

### 14. Don't care

**Erro:** Achar que X sempre precisa entrar

**Consequência:** Grupo desnecessário

**Regra:** Use X só quando ajuda

**Resultado:** X é opcional

### 15. Borda

**Erro:** Achar que primeira e última não encostam

**Consequência:** Perder simplificação

**Regra:** K-map tem adjacência circular

**Resultado:** Coluna 00 pode agrupar com 10

## 2. Revisão relâmpago antes da prova

- **Identidade:** Booleano usa regras próprias → A+0=A
- **Complemento:** A+A'=1 e A·A'=0 → A·A'=0
- **Absorção:** A+A·B=A → O termo menor absorve o maior
- **Distributiva especial:** `(A+B)(A+C)=A+BC` → É uma identidade booleana
- **DeMorgan:** Troque +↔· e negue cada termo → (A+B)'=A'B'
- **Dupla negação:** Duas negações se cancelam → (A')'=A
- **XOR:** XOR vale 1 quando são diferentes → A'B+AB'
- **XNOR:** XNOR vale 1 quando são iguais → AB+A'B'
- **SOP:** Na linha F=1, bit 0 vira variável negada e bit 1 normal → 001 → A'B'C
- **POS:** Na linha F=0, bit 0 vira variável normal e bit 1 complementada → 101 → (A'+B+C')

## 3. Método de recuperação ativa

Quando bater o branco, não releia a matéria inteira. Faça a sequência:

**1. O que estou tentando descobrir?**

**2. Qual é a pegadinha mais provável?**

**3. Qual regra mata essa pegadinha?**

**4. Qual resultado essa regra produz?**

**5. Consigo explicar em uma frase?**

## 4. Sessões curtas

Use blocos de 15–25 minutos. Em cada bloco, escolha poucos itens, tente lembrar sem consultar e marque os erros. No próximo bloco, revise principalmente os que você errou.

## 5. Regra contra o esquecimento

Não tente decorar a página inteira. Transforme cada conteúdo em uma associação curta:

**PEGADINHA → REGRA → EXEMPLO → RESULTADO.**

Isso permite reconstruir a resposta mesmo quando a memória da frase original desaparece.
