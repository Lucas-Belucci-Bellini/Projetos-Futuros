# Sistemas Digitais — V4: Aprendendo a Resolver e Calcular do Zero

> O objetivo é ligar porta lógica, expressão, tabela-verdade e Karnaugh em um único processo.

## 0. O mapa da matéria

**PORTA → EXPRESSÃO → TABELA → KARNAUGH → EXPRESSÃO SIMPLIFICADA**

Uma questão pode começar em qualquer uma dessas formas.

---

# 1. AND

AND só dá 1 quando todas as entradas são 1.

A·B

Tabela:

| A | B | A·B |
|---|---|---|
| 0 | 0 | 0 |
| 0 | 1 | 0 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |

Exemplo:

A=1, B=0

1·0=0.

---

# 2. OR

OR dá 1 quando pelo menos uma entrada é 1.

A+B

Exemplos:

0+0=0

0+1=1

1+0=1

1+1=1

---

# 3. NOT

NOT inverte:

0'=1

1'=0

---

# 4. XOR e XNOR

XOR = diferentes.

XNOR = iguais.

XOR:

A'B+AB'

XNOR:

AB+A'B'

---

# 5. Como calcular uma expressão por etapas

Exemplo:

F=(A+B)·C

A=0, B=1, C=1.

Passo 1:

A+B=0+1=1

Passo 2:

1·C=1·1=1

Resultado:

**F=1**

### Regra

**NÃO FAÇA A EXPRESSÃO TODA DE UMA VEZ. CALCULE OS BLOCOS.**

---

# 6. Preencha tabela-verdade pela quantidade de entradas

Duas variáveis:

2²=4 linhas.

Três:

2³=8.

Quatro:

2⁴=16.

Isso já diz quantas combinações você precisa escrever.

---

# 7. Exemplo de tabela

F=A·B

| A | B | F |
|---|---|---|
| 0 | 0 | 0 |
| 0 | 1 | 0 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |

Você calcula uma linha de cada vez.

---

# 8. Como criar expressão a partir da tabela

Suponha:

| A | B | F |
|---|---|---|
| 0 | 0 | 1 |
| 0 | 1 | 0 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

Para SOP, pegue os 1.

00 → A'B'

10 → AB'

F=A'B'+AB'

F=B'(A'+A)

F=B'

Resultado:

**B'**

---

# 9. DeMorgan aplicado a circuito

Regras:

(A+B)'=A'B'

(AB)'=A'+B'

Ao “empurrar” uma negação:
- OR vira AND;
- AND vira OR;
- cada entrada é negada.

Memória:

**A BOLHA ENTRA → A PORTA TROCA.**

---

# 10. Simplificação algébrica de circuito

Exemplo:

F=AB+AB'

F=A(B+B')

F=A·1

F=A

Resultado:

**F=A**

A entrada B desapareceu porque, no conjunto de possibilidades, B e B' cobriam todos os casos.

---

# 11. Karnaugh: como preparar

Código Gray:

00, 01, 11, 10

A ordem precisa ser Gray para que vizinhos diferenciem em uma variável.

---

# 12. Grupo de 2

Exemplo:

A'B+AB

A varia.

Resultado:

B

### Regra

**O QUE MUDA SOME.**

---

# 13. Grupo de 4

Exemplo:

A'B'C + A'BC + AB'C + ABC

C=1 em todos.

Resultado:

C.

---

# 14. Grupos válidos

Use:

1, 2, 4, 8, 16.

Não use 3, 5, 6 ou 7.

Procure primeiro o grupo maior.

---

# 15. Bordas e sobreposição

Primeira e última posições podem ser vizinhas.

Uma célula também pode estar em mais de um grupo.

Isso é permitido.

---

# 16. Função maioria

Maioria de três significa pelo menos duas entradas iguais a 1.

Casos:
011
101
110
111

Forma minimizada:

AB+AC+BC

Exemplo:

A=1, B=0, C=1

AB=0
AC=1
BC=0

F=1.

---

# 17. Exatamente uma

Exatamente uma entrada 1:

001
010
100

SOP:

A'B'C + A'BC' + AB'C'

Não confunda com maioria.

### Regra

**EXATAMENTE UMA ≠ PELO MENOS DUAS.**

---

# 18. Calculando circuito em etapas

Exemplo:

F=(A+B')·C

A=0, B=0, C=1.

Passo 1:
B'=1

Passo 2:
A+B'=0+1=1

Passo 3:
1·C=1·1=1

Resultado:

**F=1**

---

# 19. Tabela com colunas intermediárias

Para uma expressão maior:

F=(A+B')·C

Faça:

| A | B | C | B' | A+B' | F |
|---|---|---|---|---|---|

Isso evita tentar fazer cinco operações na mesma linha.

---

# 20. Ligação circuito ↔ expressão ↔ tabela

Se o circuito gera:

F=A+B'

você pode:
1. escrever a expressão;
2. construir a tabela;
3. marcar os 1;
4. montar o Karnaugh;
5. verificar a expressão simplificada.

Esse ciclo ajuda a encontrar erro.

---

# 21. Como validar uma alternativa

Você tem duas expressões candidatas.

Escolha uma entrada em que sabe o valor da função.

Exemplo:
A=0, B=1.

Se a função original dá 0 e a alternativa dá 1, a alternativa está errada.

Um único contraexemplo já elimina uma alternativa.

---

# 22. DEU MERDA

## XOR x XNOR

**ERRO:** confundir diferentes com iguais.

**CONSEQUÊNCIA:** tabela invertida.

**REGRA:** XOR=diferentes; XNOR=iguais.

## DeMorgan

**ERRO:** negar e esquecer de trocar a porta.

**CONSEQUÊNCIA:** expressão errada.

**REGRA:** + troca com ·.

## Karnaugh

**ERRO:** montar em ordem binária normal.

**CONSEQUÊNCIA:** adjacência falsa.

**REGRA:** Gray 00,01,11,10.

---

# 23. Resumo operacional

**AND:** tudo 1.

**OR:** pelo menos um 1.

**NOT:** inverte.

**XOR:** diferentes.

**XNOR:** iguais.

**Tabela:** 2^n linhas.

**SOP:** procura F=1.

**DeMorgan:** nega + troca operador.

**Karnaugh:** Gray + grupos em potência de 2.

**Grupo maior:** mais variáveis desaparecem.

---

# 24. Checklist

- [ ] Sei calcular cada porta.
- [ ] Sei calcular expressão por etapas.
- [ ] Sei montar tabela.
- [ ] Sei gerar SOP.
- [ ] Sei aplicar DeMorgan.
- [ ] Sei usar Karnaugh.
- [ ] Sei maioria.
- [ ] Sei exatamente uma.
- [ ] Sei validar uma resposta com contraexemplo.

> **Fórmula de sobrevivência:** quando o circuito parecer confuso, quebre em blocos e dê um nome para cada resultado intermediário.
