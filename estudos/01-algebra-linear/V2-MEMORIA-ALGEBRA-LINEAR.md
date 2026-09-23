# Álgebra Linear — V2: Memória, Pegadinhas e Recuperação

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
| Confundir linhas com colunas | Escrever a ordem errada | Regra: linhas × colunas | 2 linhas e 3 colunas → 2×3 |
| Achar que muda os números e não a posição | Montar A^T igual a A sem motivo | Troque linha por coluna | A=[[1,2,3],[4,5,6]] → A^T=[[1,4],[2,5],[3,6]] |
| Somar matrizes de ordens incompatíveis | Tentar somar 2×3 com 3×2 | Só soma mesma ordem | 2×2 + 2×2 é permitido |
| Olhar só para a quantidade de linhas | Multiplicar 2×3 por 2×2 | Internas precisam coincidir | 2×3 · 3×4 → resultado 2×4 |
| Tentar inverter matriz com determinante zero | Fazer conta longa numa matriz não invertível | det(A)=0 → não existe inversa | [[1,2],[2,4]] não tem inversa |
| Somar as diagonais | Esquecer o sinal da subtração | ad-bc | [[3,4],[2,5]] → 15-8=7 |
| Misturar diagonais principais e secundárias | Resultado errado | Repita as duas primeiras colunas e faça + diagonais principais, - secundárias | Para uma 3×3, o resultado precisa vir dessa diferença |
| Errar sinais dos cofatores | Somar menores sem alternar sinais | Sinais + - + / - + - / + - + | Cofator usa (-1)^(i+j) |
| Achar que qualquer sistema tem solução única | Ignorar o determinante/linhas escalonadas | det≠0 em matriz quadrada → solução única | x+y=5; x-y=1 → (3,2) |
| Confundir contradição com infinitas soluções | Não reconhecer [0 0 | 3] | 0=3 é impossível | Sem solução |
| Parar ao encontrar uma equação dependente | Não perceber variável livre | Uma linha de dependência pode indicar infinitas soluções | x+y=2 e 2x+2y=4 |

## 1. Regras que você precisa reconhecer rápido

### 1. Matriz

**Erro:** Confundir linhas com colunas

**Consequência:** Escrever a ordem errada

**Regra:** Regra: linhas × colunas

**Resultado:** 2 linhas e 3 colunas → 2×3

### 2. Transposta

**Erro:** Achar que muda os números e não a posição

**Consequência:** Montar A^T igual a A sem motivo

**Regra:** Troque linha por coluna

**Resultado:** A=[[1,2,3],[4,5,6]] → A^T=[[1,4],[2,5],[3,6]]

### 3. Soma

**Erro:** Somar matrizes de ordens incompatíveis

**Consequência:** Tentar somar 2×3 com 3×2

**Regra:** Só soma mesma ordem

**Resultado:** 2×2 + 2×2 é permitido

### 4. Produto

**Erro:** Olhar só para a quantidade de linhas

**Consequência:** Multiplicar 2×3 por 2×2

**Regra:** Internas precisam coincidir

**Resultado:** 2×3 · 3×4 → resultado 2×4

### 5. Inversa

**Erro:** Tentar inverter matriz com determinante zero

**Consequência:** Fazer conta longa numa matriz não invertível

**Regra:** det(A)=0 → não existe inversa

**Resultado:** [[1,2],[2,4]] não tem inversa

### 6. Determinante 2×2

**Erro:** Somar as diagonais

**Consequência:** Esquecer o sinal da subtração

**Regra:** ad-bc

**Resultado:** [[3,4],[2,5]] → 15-8=7

### 7. Sarrus

**Erro:** Misturar diagonais principais e secundárias

**Consequência:** Resultado errado

**Regra:** Repita as duas primeiras colunas e faça + diagonais principais, - secundárias

**Resultado:** Para uma 3×3, o resultado precisa vir dessa diferença

### 8. Laplace

**Erro:** Errar sinais dos cofatores

**Consequência:** Somar menores sem alternar sinais

**Regra:** Sinais + - + / - + - / + - +

**Resultado:** Cofator usa (-1)^(i+j)

### 9. Sistema determinado

**Erro:** Achar que qualquer sistema tem solução única

**Consequência:** Ignorar o determinante/linhas escalonadas

**Regra:** det≠0 em matriz quadrada → solução única

**Resultado:** x+y=5; x-y=1 → (3,2)

### 10. Sistema impossível

**Erro:** Confundir contradição com infinitas soluções

**Consequência:** Não reconhecer [0 0 | 3]

**Regra:** 0=3 é impossível

**Resultado:** Sem solução

### 11. Sistema indeterminado

**Erro:** Parar ao encontrar uma equação dependente

**Consequência:** Não perceber variável livre

**Regra:** Uma linha de dependência pode indicar infinitas soluções

**Resultado:** x+y=2 e 2x+2y=4

## 2. Revisão relâmpago antes da prova

- **Matriz:** Regra: linhas × colunas → 2 linhas e 3 colunas → 2×3
- **Transposta:** Troque linha por coluna → A=[[1,2,3],[4,5,6]] → A^T=[[1,4],[2,5],[3,6]]
- **Soma:** Só soma mesma ordem → 2×2 + 2×2 é permitido
- **Produto:** Internas precisam coincidir → 2×3 · 3×4 → resultado 2×4
- **Inversa:** det(A)=0 → não existe inversa → [[1,2],[2,4]] não tem inversa
- **Determinante 2×2:** ad-bc → [[3,4],[2,5]] → 15-8=7
- **Sarrus:** Repita as duas primeiras colunas e faça + diagonais principais, - secundárias → Para uma 3×3, o resultado precisa vir dessa diferença
- **Laplace:** Sinais + - + / - + - / + - + → Cofator usa (-1)^(i+j)
- **Sistema determinado:** det≠0 em matriz quadrada → solução única → x+y=5; x-y=1 → (3,2)
- **Sistema impossível:** 0=3 é impossível → Sem solução

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
