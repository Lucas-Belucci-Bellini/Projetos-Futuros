# Álgebra Linear — V5: Guia Direcionado ao Seu Exame

> Este guia foi montado para estudar a **forma de resolver** exatamente os três tipos de questão que aparecem na atividade de exame. Ele não traz as respostas finais da avaliação; ele mostra o procedimento que você deve executar no papel.

Fonte da atividade: Exame de Tópicos em Álgebra Linear — conteúdo de Matrizes, Determinantes e Sistemas Lineares. A atividade pede resolução justificada e desenvolvimento passo a passo.

---

# 0. O que a sua prova está cobrando

A atividade tem três blocos:

1. **Escalonamento de sistema linear**.
2. **Inversa de matriz por operações elementares**.
3. **Determinante usando propriedades**.

As próprias instruções pedem que a resolução seja justificada. Portanto, na folha, não escreva apenas o resultado: escreva a operação usada em cada transformação.

---

# 1. QUESTÃO 1 — ESCALONAMENTO

A questão apresenta um sistema com três incógnitas:

- x
- y
- z

Seu objetivo não é “adivinhar” x, y e z.

Seu objetivo é transformar o sistema até conseguir:

\[
x=\text{algum número},\qquad
y=\text{algum número},\qquad
z=\text{algum número}
\]

## 1.1. Primeiro passo: transformar em matriz aumentada

Pegue os coeficientes de x, y e z e coloque a igualdade separada:

\[
\left[
\begin{array}{ccc|c}
a&b&c&r\\
d&e&f&s\\
g&h&i&t
\end{array}
\right]
\]

No seu exame, use os coeficientes exatamente como aparecem no enunciado.

## 1.2. Segundo passo: escolher o primeiro pivô

O primeiro número da primeira linha é seu primeiro pivô.

Você quer zerar os números **abaixo dele**.

Pense:

\[
\begin{bmatrix}
\boxed{\text{pivô}}&*&*\\
?&*&*\\
?&*&*
\end{bmatrix}
\]

Seu objetivo é:

\[
\begin{bmatrix}
\boxed{\text{pivô}}&*&*\\
0&*&*\\
0&*&*
\end{bmatrix}
\]

## 1.3. Como escrever a operação

Você não deve escrever somente a matriz seguinte.

Escreva, por exemplo:

\[
L_2\leftarrow L_2-kL_1
\]

e faça a conta célula por célula.

Depois:

\[
L_3\leftarrow L_3-mL_1
\]

O valor de k ou m é escolhido para fazer o primeiro elemento virar zero.

## 1.4. Agora vá para a segunda coluna

Depois que os dois elementos abaixo do primeiro pivô virarem zero, procure o próximo pivô:

\[
\begin{bmatrix}
*&*&*\\
0&\boxed{\text{pivô}}&*\\
0&?&*
\end{bmatrix}
\]

Agora use a segunda linha para zerar o número abaixo do segundo pivô:

\[
L_3\leftarrow L_3-kL_2
\]

Seu sistema ficará com o formato:

\[
\begin{bmatrix}
*&*&*|*\\
0&*&*|*\\
0&0&*|*
\end{bmatrix}
\]

Isso é o **escalonamento**.

---

# 2. Como sair do escalonamento e descobrir as incógnitas

Agora faça o caminho contrário.

Comece pela última linha.

Se ela ficar parecida com:

\[
cz=r
\]

então:

\[
z=\frac{r}{c}
\]

Depois suba uma linha.

Se aparecer:

\[
by+cz=r
\]

você já conhece z.

Substitua o valor de z:

\[
by+c(\text{valor de z})=r
\]

e descubra y.

Depois vá para a primeira linha.

Essa etapa é chamada de **substituição retroativa**.

### Memória

**ESCALONA DE CIMA PARA BAIXO → RESOLVE DE BAIXO PARA CIMA.**

---

# 3. Como conferir a Questão 1

Depois de encontrar x, y e z, não entregue imediatamente.

Pegue os valores e substitua nas **três equações originais**.

Exemplo genérico:

\[
2x+y=7
\]

Se você encontrou algum valor para x e y, substitua.

Faça isso nas três linhas.

### Se as três baterem:

sua solução é consistente com o sistema.

### Se uma não bater:

alguma operação foi feita errada.

---

# 4. Operações de linha que você pode usar

Na Questão 1, você pode:

### Trocar linhas

\[
L_1\leftrightarrow L_2
\]

### Multiplicar uma linha por número diferente de zero

\[
L_1\leftarrow kL_1
\]

### Somar múltiplo de uma linha a outra

\[
L_2\leftarrow L_2-kL_1
\]

Essas são as três operações elementares.

---

# 5. COMO ESCREVER A QUESTÃO 1 NA FOLHA

Use este modelo:

## Passo 1 — matriz aumentada

\[
\left[
\begin{array}{ccc|c}
&&&\\
&&&\\
&&&
\end{array}
\right]
\]

## Passo 2 — operação

\[
L_2\leftarrow L_2-kL_1
\]

Matriz nova:

\[
\left[
\begin{array}{ccc|c}
&&&\\
&&&\\
&&&
\end{array}
\right]
\]

## Passo 3 — operação

\[
L_3\leftarrow L_3-mL_1
\]

Matriz nova.

## Passo 4

Zere o elemento abaixo do segundo pivô.

## Passo 5

Faça substituição retroativa.

## Passo 6

Confira nas três equações originais.

---

# 6. QUESTÃO 2 — INVERSA POR OPERAÇÕES ELEMENTARES

Aqui você não vai tentar “adivinhar” a inversa.

O método é mecânico.

## 6.1. Monte a matriz aumentada

Pegue a matriz A da questão e coloque a identidade do mesmo tamanho ao lado:

\[
[A\mid I]
\]

Para uma matriz 3 × 3:

\[
\left[
\begin{array}{ccc|ccc}
*&*&*&1&0&0\\
*&*&*&0&1&0\\
*&*&*&0&0&1
\end{array}
\right]
\]

A matriz da esquerda é seu alvo.

Você quer transformá-la em:

\[
\left[
\begin{array}{ccc|ccc}
1&0&0&*&*&*\\
0&1&0&*&*&*\\
0&0&1&*&*&*
\end{array}
\right]
\]

Quando chegar aí:

\[
\text{lado direito}=A^{-1}
\]

---

# 7. Regra mais importante da Questão 2

As operações feitas na esquerda também são feitas na direita.

Se você fizer:

\[
L_2\leftarrow L_2-2L_1
\]

tem que aplicar a mesma operação à linha inteira da matriz aumentada.

Não altere apenas os números da esquerda.

### Memória

**OPERAÇÃO DE LINHA = OPERAÇÃO NA LINHA INTEIRA.**

---

# 8. Estratégia para a Questão 2

Você quer produzir:

### Primeiro pivô

\[
1\ 0\ 0
\]

### Segundo pivô

\[
0\ 1\ 0
\]

### Terceiro pivô

\[
0\ 0\ 1
\]

E também precisa zerar os números **acima** dos pivôs.

Por isso a inversa normalmente passa por duas fases:

**ESCALONAR → VOLTAR E ZERAR ACIMA**

---

# 9. Como saber que terminou a Questão 2

Você terminou somente quando a esquerda for exatamente:

\[
I=
\begin{bmatrix}
1&0&0\\
0&1&0\\
0&0&1
\end{bmatrix}
\]

Não basta ter zeros abaixo dos pivôs.

Confira também os zeros acima.

### Memória

**SE A ESQUERDA NÃO É I, A INVERSA AINDA NÃO ESTÁ PRONTA.**

---

# 10. Como conferir uma inversa

Depois que obter uma candidata a \(A^{-1}\), o conceito é:

\[
A\cdot A^{-1}=I
\]

Na prova, o método solicitado é por operações elementares, então a própria transformação

\[
[A|I]\rightarrow[I|A^{-1}]
\]

já é o desenvolvimento principal.

---

# 11. QUESTÃO 3 — DETERMINANTE POR PROPRIEDADES

Esta questão é a mais diferente.

Aqui você **não quer multiplicar as matrizes**.

A questão já fornece:

\[
\det(A)=4
\]

e

\[
\det(B)=-2
\]

O truque é transformar o determinante da expressão em uma conta numérica.

---

# 12. Propriedade da transposta

Quando aparecer:

\[
A^T
\]

use:

\[
\det(A^T)=\det(A)
\]

Então a transposta desaparece do problema do determinante.

---

# 13. Propriedade do produto

Quando aparecer:

\[
\det(XY)
\]

separe:

\[
\det(XY)=\det(X)\det(Y)
\]

Para vários fatores:

\[
\det(XYZ)=\det(X)\det(Y)\det(Z)
\]

---

# 14. Propriedade da potência

Quando aparecer:

\[
B^2
\]

pense:

\[
B^2=B\cdot B
\]

Portanto:

\[
\det(B^2)=\det(B)^2
\]

No seu caso, depois substitua o valor conhecido de \(\det(B)\).

---

# 15. Propriedade da inversa

Quando aparecer:

\[
A^{-1}
\]

use:

\[
\det(A^{-1})=\frac{1}{\det(A)}
\]

E quando aparecer:

\[
(B^2)^{-1}
\]

faça em duas etapas:

\[
\det((B^2)^{-1})
=
\frac{1}{\det(B^2)}
\]

Depois:

\[
\det(B^2)=\det(B)^2
\]

Então:

\[
\det((B^2)^{-1})
=
\frac{1}{\det(B)^2}
\]

---

# 16. O cuidado MAIS IMPORTANTE: o número 3

Na sua expressão existe:

\[
3\cdot A^T\cdot(B^2)^{-1}\cdot A^{-1}
\]

A matriz é 3 × 3.

Quando um número multiplica uma matriz 3 × 3:

\[
\det(kA)=k^3\det(A)
\]

Portanto, neste tipo de questão, o 3 **não entra simplesmente como 3**.

O tamanho da matriz vira o expoente.

### Memória

**3 × 3 → potência 3.**

---

# 17. Como atacar a sua Questão 3 no papel

Escreva primeiro:

\[
\det(M)=
\det\left(3\cdot A^T\cdot(B^2)^{-1}\cdot A^{-1}\right)
\]

Depois quebre a expressão:

\[
\det(M)
=
\det(3\cdot \text{parte})
\]

Como a matriz é 3 × 3:

\[
=3^3\cdot
\det(A^T)\cdot
\det((B^2)^{-1})\cdot
\det(A^{-1})
\]

Agora substitua cada pedaço por uma propriedade:

\[
\det(A^T)\rightarrow\det(A)
\]

\[
\det((B^2)^{-1})
\rightarrow
\frac{1}{\det(B)^2}
\]

\[
\det(A^{-1})
\rightarrow
\frac{1}{\det(A)}
\]

Agora você terá somente números e poderá fazer a conta.

**Aqui está o ponto-chave: o objetivo é chegar a uma expressão numérica.**

---

# 18. A sequência exata das propriedades da Questão 3

Quando escrever a justificativa, você pode pensar nesta ordem:

1. **Produto:** separa os determinantes.
2. **Escalar:** transforma o 3 em \(3^3\), porque a matriz é 3 × 3.
3. **Transposta:** \(\det(A^T)=\det(A)\).
4. **Inversa:** \(\det(A^{-1})=1/\det(A)\).
5. **Potência:** \(\det(B^2)=\det(B)^2\).
6. **Inversa novamente:** \(\det((B^2)^{-1})=1/\det(B)^2\).
7. Substitua \(\det(A)=4\) e \(\det(B)=-2\).
8. Faça a conta numérica.

---

# 19. O que NÃO fazer

## Questão 1

Não tente achar x, y e z “no chute”.

Use escalonamento.

## Questão 2

Não tente calcular a inversa por fórmula de cabeça.

Use:

\[
[A|I]\rightarrow[I|A^{-1}]
\]

## Questão 3

Não tente calcular \(A\), \(B\), \(A^T\) ou \(B^2\) individualmente.

A questão fornece justamente os determinantes para você trabalhar pelas propriedades.

---

# 20. DEU MERDA — versão para esta prova

## Sistema

**ERRO:** fazer uma operação somente em um número.

**CONSEQUÊNCIA:** a matriz deixa de representar o mesmo sistema.

**REGRA:** operação de linha vale para a linha inteira.

---

## Inversa

**ERRO:** mexer só na matriz da esquerda.

**CONSEQUÊNCIA:** o lado direito deixa de representar a inversa.

**REGRA:** toda operação da matriz aumentada vale nos dois lados.

---

## Determinante

**ERRO:** tratar \(\det(3A)\) como \(3\det(A)\) em uma matriz 3 × 3.

**CONSEQUÊNCIA:** fator numérico errado.

**REGRA:**

\[
\det(3A)=3^3\det(A)
\]

---

## Deixar o resultado sem justificativa

**ERRO:** colocar somente o número final.

**CONSEQUÊNCIA:** perde a parte do desenvolvimento exigida pela atividade.

**REGRA:** escreva a propriedade ou operação usada em cada etapa.

---

# 21. COLA MENTAL DE 30 SEGUNDOS

## Q1 — Sistema

**MATRIZ AUMENTADA**

↓  

**ZERAR ABAIXO**

↓

**ESCALONAR**

↓

**SUBSTITUIÇÃO DE BAIXO PARA CIMA**

↓

**CONFERIR**

---

## Q2 — Inversa

\[
[A|I]
\]

↓

**TRANSFORMAR ESQUERDA EM I**

↓

\[
[I|A^{-1}]
\]

---

## Q3 — Determinante

**SEPARAR PRODUTO**

↓

**ESCALAR**

↓

**TRANSPOSTA**

↓

**POTÊNCIA**

↓

**INVERSA**

↓

**SUBSTITUIR DET(A) E DET(B)**

↓

**CONTA NUMÉRICA**

---

# 22. Checklist antes de entregar

### Questão 1

- [ ] Montei a matriz aumentada.
- [ ] Escrevi cada operação de linha.
- [ ] Zerei abaixo dos pivôs.
- [ ] Fiz a substituição retroativa.
- [ ] Testei x, y e z nas equações originais.

### Questão 2

- [ ] Montei [A|I].
- [ ] Toda operação foi feita na linha inteira.
- [ ] A esquerda virou exatamente I.
- [ ] Copiei a direita como \(A^{-1}\).

### Questão 3

- [ ] Separei o produto.
- [ ] Lembrei que é 3 × 3.
- [ ] Usei transposta corretamente.
- [ ] Usei potência corretamente.
- [ ] Usei inversa corretamente.
- [ ] Substituí \(\det(A)=4\).
- [ ] Substituí \(\det(B)=-2\).
- [ ] Mostrei as propriedades.
- [ ] Só então fiz a conta final.

---

# 23. Regra final

Quando você olhar para a prova e pensar:

> “Eu não faço ideia de como começar.”

não procure a resposta.

Procure **o tipo de questão**:

**Q1 = ESCALONAMENTO**

**Q2 = [A|I]**

**Q3 = PROPRIEDADES DO DETERMINANTE**

Depois faça somente o primeiro passo.

**Primeiro passo → segundo passo → terceiro passo.**

É assim que você resolve uma questão grande sem precisar saber tudo de uma vez.
