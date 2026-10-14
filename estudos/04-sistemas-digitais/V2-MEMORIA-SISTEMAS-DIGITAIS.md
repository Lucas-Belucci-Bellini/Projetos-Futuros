# Sistemas Digitais — V2: Memória, Pegadinhas e Recuperação

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
| Achar que qualquer 1 gera 1 | Tabela errada | AND só vale 1 em entradas todas 1 | 00→0, 01→0, 10→0, 11→1 |
| Achar que tudo precisa ser 1 | Tabela errada | OR vale 1 se ao menos uma entrada for 1 | 00→0; restante →1 |
| Confundir diferença com soma lógica | Aceitar 11 | XOR = diferentes | 01 e 10 são 1 |
| Inverter XOR na cabeça | Saída errada | XNOR = iguais | 00 e 11 são 1 |
| Negar sem trocar operador | Expressão errada | Troque operador e negue cada variável | (AB)'=A'+B' |
| Usar ordem 00,10,01,11 | Vizinhança quebrada | Gray: 00,01,11,10 | Mesma regra em todas as questões |
| Achar que quatro 1s qualquer formam grupo | Grupo diagonal inválido | O bloco precisa ser retangular e adjacente | 2×2 é válido |
| Não procurar o maior grupo | Expressão maior | Comece pelo maior grupo válido | 8 células eliminam 3 variáveis |
| Inverter bit 0 errado | Termo errado | 0→complementado; 1→normal | 101→AB'C |
| Aplicar regra de mintermo | Sinais errados | 0→normal; 1→complementado | 101→(A'+B+C') |

## 1. Regras que você precisa reconhecer rápido

### 1. AND

**Erro:** Achar que qualquer 1 gera 1

**Consequência:** Tabela errada

**Regra:** AND só vale 1 em entradas todas 1

**Resultado:** 00→0, 01→0, 10→0, 11→1

### 2. OR

**Erro:** Achar que tudo precisa ser 1

**Consequência:** Tabela errada

**Regra:** OR vale 1 se ao menos uma entrada for 1

**Resultado:** 00→0; restante →1

### 3. XOR

**Erro:** Confundir diferença com soma lógica

**Consequência:** Aceitar 11

**Regra:** XOR = diferentes

**Resultado:** 01 e 10 são 1

### 4. XNOR

**Erro:** Inverter XOR na cabeça

**Consequência:** Saída errada

**Regra:** XNOR = iguais

**Resultado:** 00 e 11 são 1

### 5. DeMorgan

**Erro:** Negar sem trocar operador

**Consequência:** Expressão errada

**Regra:** Troque operador e negue cada variável

**Resultado:** (AB)'=A'+B'

### 6. K-map 3 var

**Erro:** Usar ordem 00,10,01,11

**Consequência:** Vizinhança quebrada

**Regra:** Gray: 00,01,11,10

**Resultado:** Mesma regra em todas as questões

### 7. Grupo de 4

**Erro:** Achar que quatro 1s qualquer formam grupo

**Consequência:** Grupo diagonal inválido

**Regra:** O bloco precisa ser retangular e adjacente

**Resultado:** 2×2 é válido

### 8. Grupo de 8

**Erro:** Não procurar o maior grupo

**Consequência:** Expressão maior

**Regra:** Comece pelo maior grupo válido

**Resultado:** 8 células eliminam 3 variáveis

### 9. Mintermo

**Erro:** Inverter bit 0 errado

**Consequência:** Termo errado

**Regra:** 0→complementado; 1→normal

**Resultado:** 101→AB'C

### 10. Maxtermo

**Erro:** Aplicar regra de mintermo

**Consequência:** Sinais errados

**Regra:** 0→normal; 1→complementado

**Resultado:** 101→(A'+B+C')

## 2. Revisão relâmpago antes da prova

- **AND:** AND só vale 1 em entradas todas 1 → 00→0, 01→0, 10→0, 11→1
- **OR:** OR vale 1 se ao menos uma entrada for 1 → 00→0; restante →1
- **XOR:** XOR = diferentes → 01 e 10 são 1
- **XNOR:** XNOR = iguais → 00 e 11 são 1
- **DeMorgan:** Troque operador e negue cada variável → (AB)'=A'+B'
- **K-map 3 var:** Gray: 00,01,11,10 → Mesma regra em todas as questões
- **Grupo de 4:** O bloco precisa ser retangular e adjacente → 2×2 é válido
- **Grupo de 8:** Comece pelo maior grupo válido → 8 células eliminam 3 variáveis
- **Mintermo:** 0→complementado; 1→normal → 101→AB'C
- **Maxtermo:** 0→normal; 1→complementado → 101→(A'+B+C')

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
