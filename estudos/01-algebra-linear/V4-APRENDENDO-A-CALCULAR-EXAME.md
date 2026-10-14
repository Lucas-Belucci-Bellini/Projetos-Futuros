# Álgebra Linear — V4: Aprendendo a Calcular do Zero

> Material de estudo baseado na Atividade de Exame enviada. O exame informa como conteúdo Matrizes, Determinantes e Sistemas Lineares e exige resolução justificada e passo a passo. fileciteturn173file0L4-L11

## 0. O que você precisa aprender

O exame tem três habilidades principais:

1. Resolver sistema linear por escalonamento.
2. Encontrar inversa por operações elementares.
3. Calcular determinantes usando propriedades.

Não tente decorar tudo. Aprenda a reconhecer o tipo de questão e aplicar um roteiro.

---

# PARTE 1 — SISTEMA LINEAR POR ESCALONAMENTO

A Questão 1 da atividade pede resolução por escalonamento. fileciteturn173file0L12-L18

## 1.1 Transforme o sistema em matriz aumentada

Exemplo:

~~~text
x + y + z = 6
2x - y + z = 3
x + 2y - z = 2
~~~

Matriz aumentada:

~~~text
1   1   1 |  6
2  -1   1 |  3
1   2  -1 |  2
~~~

Antes da barra: coeficientes.
Depois da barra: termos independentes.

## 1.2 As três operações de linha

Trocar linhas:

~~~text
L1 ↔ L2
~~~

Multiplicar uma linha por número diferente de zero:

~~~text
L1 → 2L1
~~~

Somar um múltiplo de uma linha a outra:

~~~text
L2 → L2 - 2L1
~~~

Essa terceira operação é a principal para zerar elementos.

## 1.3 O objetivo

Queremos chegar a algo assim:

~~~text
a  b  c | d
0  e  f | g
0  0  h | i
~~~

Depois resolvemos de baixo para cima.

### Frase de memória

**ZERA → ESCALONA → SOBE → CONFERE.**

## 1.4 Exemplo completo

Começamos:

~~~text
1  1  1 | 6
2 -1  1 | 3
1  2 -1 | 2
~~~

Para zerar o primeiro elemento da linha 2:

~~~text
L2 → L2 - 2L1
~~~

Resultado:

~~~text
1  1  1 |  6
0 -3 -1 | -9
1  2 -1 |  2
~~~

Agora linha 3:

~~~text
L3 → L3 - L1
~~~

Resultado:

~~~text
1  1  1 |  6
0 -3 -1 | -9
0  1 -2 | -4
~~~

Para facilitar o próximo pivô, troque as linhas 2 e 3:

~~~text
L2 ↔ L3
~~~

Fica:

~~~text
1  1  1 |  6
0  1 -2 | -4
0 -3 -1 | -9
~~~

Agora:

~~~text
L3 → L3 + 3L2
~~~

Resultado escalonado:

~~~text
1  1  1 |   6
0  1 -2 |  -4
0  0 -7 | -21
~~~

Comece pela última linha:

-7z = -21

z = 3

Segunda linha:

y - 2z = -4

y - 6 = -4

y = 2

Primeira linha:

x + y + z = 6

x + 2 + 3 = 6

x = 1

Resultado do exemplo:

~~~text
x = 1
y = 2
z = 3
~~~

## 1.5 Como saber se errou

Depois de encontrar x, y e z, substitua os três valores em TODAS as equações.

Se alguma equação não fechar, volte ao escalonamento.

## 1.6 Classificação

SPD = uma solução.

SPI = infinitas soluções.

SI = nenhuma solução.

Pegadinha:

~~~text
0  0  0 | 5
~~~

Isso representa 0 = 5. Portanto: SI.

Já:

~~~text
0  0  0 | 0
~~~

não é contradição e pode indicar variável livre e SPI.

---

# PARTE 2 — INVERSA POR OPERAÇÕES ELEMENTARES

A Questão 2 pede encontrar a inversa por operações elementares. fileciteturn173file0L19-L24

## 2.1 A ideia

Queremos transformar A em I.

Começamos com:

~~~text
[ A | I ]
~~~

E queremos chegar a:

~~~text
[ I | A⁻¹ ]
~~~

### Frase de memória

**ESQUERDA VIRA IDENTIDADE; DIREITA VIRA INVERSA.**

## 2.2 Exemplo 2×2

Comece:

~~~text
1  2 | 1  0
3  4 | 0  1
~~~

Zere o 3:

~~~text
L2 → L2 - 3L1
~~~

Resultado:

~~~text
1  2 |  1  0
0 -2 | -3  1
~~~

Transforme -2 em 1:

~~~text
L2 → (-1/2)L2
~~~

Resultado:

~~~text
1 2 |    1     0
0 1 |  3/2  -1/2
~~~

Agora zere o 2 da primeira linha:

~~~text
L1 → L1 - 2L2
~~~

Resultado:

~~~text
1 0 | -2    1
0 1 | 3/2 -1/2
~~~

Então a direita é A⁻¹:

~~~text
A⁻¹ = [ -2      1   ]
       [  3/2  -1/2 ]
~~~

## 2.3 A regra que mais causa erro

Se você fizer uma operação na esquerda, faça a MESMA operação na direita.

Errado:

~~~text
mudei A, mas deixei I igual
~~~

Certo:

~~~text
[ A | I ]
mesma operação dos dois lados
↓
[ I | A⁻¹ ]
~~~

## 2.4 Como conferir

Faça:

~~~text
A · A⁻¹
~~~

Se aparecer a identidade, a inversa está correta.

---

# PARTE 3 — DETERMINANTES POR PROPRIEDADES

A Questão 3 pede exclusivamente propriedades dos determinantes e solicita que cada propriedade utilizada seja indicada. fileciteturn173file0L27-L31 fileciteturn173file0L39-L39

## 3.1 Primeiro: aprenda o tradutor

| Você vê | Transforme em |
|---|---|
| Aᵀ | det(Aᵀ)=det(A) |
| AB | det(AB)=det(A)·det(B) |
| B² | det(B²)=det(B)² |
| A⁻¹ | det(A⁻¹)=1/det(A) |
| kA em matriz n×n | det(kA)=kⁿdet(A) |

Memorize essa tabela em vez de tentar decorar uma página inteira.

## 3.2 Transposta

Se:

det(A)=4

então:

det(Aᵀ)=4

**Propriedade:** det(Aᵀ)=det(A).

## 3.3 Produto

Se:

det(A)=4
det(B)=3

então:

det(AB)=4·3=12

**Propriedade:** det(AB)=det(A)·det(B).

## 3.4 Potência

Se det(B)=2:

det(B²)=2²=4

**Propriedade:** det(B²)=det(B)².

## 3.5 Inversa

Se det(A)=4:

det(A⁻¹)=1/4

**Propriedade:** det(A⁻¹)=1/det(A).

### Frase de memória

**INVERTEU → VIRA 1 SOBRE.**

## 3.6 Escalar

Se A é 3×3:

det(3A)=3³det(A)=27det(A)

### Pegadinha

Uma linha multiplicada por 3 não é a mesma coisa que a matriz inteira multiplicada por 3.

Uma linha:

det novo = 3·det antigo

Matriz inteira 3×3:

det novo = 3³·det antigo

---

# PARTE 4 — COMO RESOLVER UMA EXPRESSÃO GRANDE

Imagine uma expressão do mesmo formato da Questão 3:

~~~text
M = 2 · Aᵀ · (B²)⁻¹ · A⁻¹
~~~

Suponha, apenas como exemplo de treino:

det(A)=5
det(B)=2

Você NÃO calcula a matriz M.

Você escreve:

~~~text
det(M)
= det(2 · Aᵀ · (B²)⁻¹ · A⁻¹)
~~~

Como estamos trabalhando com uma matriz de ordem 3, o escalar 2 produz:

~~~text
det(M) = 2³ · det(Aᵀ) · det((B²)⁻¹) · det(A⁻¹)
~~~

Agora traduza cada pedaço:

Transposta:

~~~text
det(Aᵀ)=det(A)=5
~~~

Potência:

~~~text
det(B²)=det(B)²=2²=4
~~~

Inversa da potência:

~~~text
det((B²)⁻¹)=1/4
~~~

Inversa de A:

~~~text
det(A⁻¹)=1/5
~~~

Então sobra uma expressão só com números:

~~~text
det(M)=8·5·(1/4)·(1/5)
~~~

Agora faça a aritmética.

### O segredo

**Transforme a expressão inteira em números antes de calcular o resultado final.**

---

# PARTE 5 — COMO ESCREVER A RESOLUÇÃO NA PROVA

O exame exige justificativa e desenvolvimento passo a passo. fileciteturn173file0L10-L11

## Para escalonamento

Escreva a operação:

~~~text
L2 → L2 - 2L1
~~~

Depois mostre a matriz resultante.

Não pule diretamente para o resultado.

## Para inversa

Mostre:

~~~text
[A | I]
↓
operações elementares
↓
[I | A⁻¹]
~~~

## Para determinante

Escreva o nome da propriedade:

**Propriedade da transposta:**

det(Aᵀ)=det(A)

Depois:

**Propriedade do produto:**

det(AB)=det(A)·det(B)

Depois continue.

---

# PARTE 6 — ERROS QUE VOCÊ VAI EVITAR

## Erro 1 — Escalonar só metade

Se fizer uma operação de linha, todos os elementos daquela linha precisam participar da conta.

## Erro 2 — Inversa só de um lado

[A|I] precisa receber as mesmas operações dos dois lados.

## Erro 3 — Confundir det(kA)

Pergunte: o número multiplicou UMA linha ou a MATRIZ INTEIRA?

## Erro 4 — Calcular matriz quando só pedem determinante

Se aparecem transposta, potência, inversa e produto, tente primeiro as propriedades.

## Erro 5 — Não conferir

Em sistema, substitua a solução nas equações.

Na inversa, multiplique A pela inversa encontrada.

---

# PARTE 7 — ROTEIROS DE 10 SEGUNDOS

## Vi um sistema

**Matriz aumentada → zerar abaixo → escalonar → subir → conferir.**

## Vi uma inversa

**[A|I] → esquerda vira I → direita é A⁻¹ → conferir.**

## Vi determinante com A, B, transposta, potência ou inversa

**Separar produtos → transposta → potência → inversa → escalar → aritmética.**

---

# PARTE 8 — MINI CHECKLIST

- [ ] Sei montar matriz aumentada.
- [ ] Sei fazer L2 → L2 - kL1.
- [ ] Sei resolver de baixo para cima.
- [ ] Sei reconhecer 0 = número diferente de zero como SI.
- [ ] Sei montar [A|I].
- [ ] Sei que as operações precisam ser feitas dos dois lados.
- [ ] Sei conferir A·A⁻¹=I.
- [ ] Sei det(Aᵀ)=det(A).
- [ ] Sei det(AB)=det(A)det(B).
- [ ] Sei det(B²)=det(B)².
- [ ] Sei det(A⁻¹)=1/det(A).
- [ ] Sei det(kA)=kⁿdet(A).
- [ ] Sei escrever explicitamente qual propriedade usei.

# Regra final

Quando bater o branco, não tente lembrar a matéria inteira.

Pergunte apenas:

**É SISTEMA? → ESCALONAMENTO.**

**É INVERSA? → [A|I].**

**É DETERMINANTE COM OPERAÇÕES? → PROPRIEDADES.**

Depois faça uma operação por vez.