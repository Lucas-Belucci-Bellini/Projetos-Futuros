# Sistemas Digitais 2 — V3 de Revisão para Exame

> Reforço baseado no Caderno de Estudo nº 2 e nas suas quatro pegadinhas centrais.

## O que esta lista quer testar

1. Absorção e distributiva booleana.
2. DeMorgan, inclusive com variáveis já complementadas.
3. Código Gray e agrupamentos de Karnaugh.
4. Reconhecimento de funções como XOR/XNOR, maioria e exatamente uma.

## As quatro pegadinhas do caderno

**Absorção aplicada várias vezes:** A+A·B'+A·B=A.

**Distributiva especial:** (A+B)(A+C)=A+BC.

**Adjacência:** A'B+AB=B.

**Absorção com complemento:** B+B'C=B+C.

## DeMorgan sem erro

Troque o operador e negue cada variável. Se uma variável já tiver complemento, a negação dupla cancela.

## Karnaugh

Use Gray 00,01,11,10. Procure primeiro o maior grupo válido. Grupos podem ter 1,2,4,8,... células. Primeira e última bordas são adjacentes. Sobreposição é permitida.

## Casos importantes do caderno

- m1,m2,m4 isolados → não inventar grupo.
- Grupo de 8 em 4 variáveis → sobra 1 variável.
- Funções que dependem de apenas duas variáveis podem ignorar as outras no resultado.
- Uma célula já coberta pode ser reutilizada se isso reduzir a expressão.

## Método de memória

**PEGADINHA → CONSEQUÊNCIA → REGRA → RESULTADO.**
