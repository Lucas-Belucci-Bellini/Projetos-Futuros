# Sistemas Digitais 2 — V4: Aprendendo a Calcular do Zero

> Esta revisão segue as quatro áreas do caderno: simplificação booleana, DeMorgan, Karnaugh e reconhecimento de funções.

## 0. Roteiro

**EXPRESSÃO → LEI → SIMPLIFICAÇÃO → MAPA → CONFIRMAÇÃO**

---

# 1. Absorção

Exemplo:

A+AB

A já aparece sozinho.

Resultado:

A.

Agora:

A+AB'+AB

A sozinho absorve os dois termos.

Resultado:

A.

---

# 2. Distributiva especial

(A+B)(A+C)=A+BC

Exemplo:

(X+Y)(X+Z)

Resultado:

X+YZ.

Você também pode verificar expandindo:

X·X + X·Z + Y·X + Y·Z

X + XZ + XY + YZ

Depois absorção:

X+YZ.

---

# 3. Complemento interno

Exemplo:

A(B+B')C

Dentro do parêntese:

B+B'=1

Então:

A·1·C

Resultado:

AC.

---

# 4. Adjacência

A'B+AB

Fatore:

B(A'+A)

Depois:

B·1

Resultado:

B.

No Karnaugh, essas duas parcelas seriam duas células vizinhas e A desapareceria.

---

# 5. Absorção com complemento

Uma identidade importante:

A+A'B=A+B

Exemplo:

X+X'Y=X+Y.

Quando aparecer essa estrutura, não precisa expandir tudo.

---

# 6. DeMorgan

Regras:

(A+B)'=A'B'

(AB)'=A'+B'

Use:

**NEGA → TROCA O OPERADOR**

---

# 7. DeMorgan com variável já negada

Exemplo:

(A'+B+C')'

Troca soma por produto:

(A')'·B'·(C')'

Elimine as duplas:

AB'C

Resultado:

**AB'C**

---

# 8. DeMorgan em camadas

Exemplo:

((A+B)(A+C))'

Primeiro a camada externa:

(A+B)' + (A+C)'

Depois:

A'B' + A'C'

Por fim:

A'(B'+C').

### Regra

**DE FORA PARA DENTRO.**

---

# 9. Código Gray

Para Karnaugh:

00, 01, 11, 10

Não:

00, 01, 10, 11.

Escreva a ordem no papel antes de começar.

---

# 10. Como descobrir uma célula

Exemplo:

m6 em três variáveis.

6 = 110.

Logo:

A=1
B=1
C=0

Mintermo:

ABC'

---

# 11. Grupo de 2

Duas células vizinhas diferem em uma variável.

Exemplo:

A'B+AB

A varia.

Resultado:

B.

### Regra

**VARIÁVEL QUE MUDA → SOME.**

---

# 12. Grupo de 4

Exemplo:

A'B'C+A'BC+AB'C+ABC

C é constante em 1.

A e B variam.

Resultado:

C.

---

# 13. Grupo de 8

Se oito células de quatro variáveis têm A=0 e todas as outras mudam:

resultado:

A'.

Quanto maior o grupo, menos variáveis ficam no termo.

---

# 14. Isolados

Se 1 em m1,m2,m4 não forma grupos válidos de 2, não invente adjacência.

Mantenha os mintermos.

### Regra

**SEM ADJACÊNCIA = SEM GRUPO.**

---

# 15. Bordas

A primeira e a última coluna são adjacentes.

A primeira e a última linha também podem ser adjacentes.

O mapa permite wrap-around.

---

# 16. Sobreposição

Uma célula pode ser reutilizada em dois grupos.

Use sobreposição quando isso produzir termos melhores.

---

# 17. XOR

Diferentes:

A'B+AB'

Casos:

01 e 10.

---

# 18. XNOR

Iguais:

AB+A'B'

Casos:

00 e 11.

---

# 19. Exatamente uma

Para três variáveis:

001
010
100

SOP:

A'B'C + A'BC' + AB'C'

Esses 1 podem ficar isolados.

---

# 20. Maioria

“Maioria de 3” = pelo menos duas entradas 1.

Casos:

011
101
110
111

Resultado minimizado:

AB+AC+BC

### Pegadinha

Maioria não é exatamente uma.

---

# 21. Reconhecendo XNOR rapidamente

Se uma função vale 1 quando duas variáveis são iguais:

F=BD+B'D'

Você pode reconhecer XNOR imediatamente.

Se vale 1 quando são diferentes:

F=B'D+BD'

é XOR.

---

# 22. Como conferir com uma entrada

Considere:

F=BD+B'D'

Teste B=1 e D=1:

BD=1

B'D'=0

F=1

Agora B=1 e D=0:

BD=0

B'D'=0

F=0

Portanto a função aceita igualdade.

---

# 23. Como simplificar expressão grande

Exemplo:

A+A'B+A'C

Primeiro fature:

A+A'(B+C)

Agora reconheça:

A+A'X=A+X

Logo:

A+B+C.

### Regra

**PROCURE ESTRUTURA ANTES DE EXPANDIR TUDO.**

---

# 24. DEU MERDA

## Gray errado

**ERRO:** usar ordem binária normal.

**CONSEQUÊNCIA:** mapa com vizinhanças falsas.

**REGRA:** 00,01,11,10.

## Negação dupla esquecida

**ERRO:** (A')' continua A'.

**CONSEQUÊNCIA:** variável errada.

**REGRA:** (A')'=A.

## Maioria x exatamente uma

**ERRO:** usar AB+AC+BC em “exatamente uma”.

**CONSEQUÊNCIA:** aceita casos com duas ou três entradas 1.

**REGRA:** exatamente uma = 001,010,100.

---

# 25. Roteiro completo de Karnaugh

1. escreva Gray;
2. marque os 1;
3. considere don't cares apenas se ajudarem;
4. procure 8;
5. depois 4;
6. depois 2;
7. depois 1;
8. aceite sobreposição;
9. aceite bordas;
10. descubra o que ficou constante;
11. escreva os termos;
12. confira.

---

# 26. Checklist

- [ ] Sei absorção.
- [ ] Sei distributiva.
- [ ] Sei complemento.
- [ ] Sei adjacência.
- [ ] Sei absorção com complemento.
- [ ] Sei DeMorgan.
- [ ] Sei negação dupla.
- [ ] Sei Gray.
- [ ] Sei mintermos.
- [ ] Sei grupos 1/2/4/8.
- [ ] Sei isolados.
- [ ] Sei bordas.
- [ ] Sei sobreposição.
- [ ] Sei XOR/XNOR.
- [ ] Sei maioria.
- [ ] Sei exatamente uma.

> **Fórmula de sobrevivência:** quando travar, simplifique a parte que você reconhece e deixe o Karnaugh resolver o restante.
