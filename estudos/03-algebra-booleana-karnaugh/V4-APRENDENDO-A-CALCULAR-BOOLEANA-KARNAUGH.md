# Álgebra Booleana e Karnaugh — V4: Aprendendo a Calcular do Zero

> O objetivo aqui é aprender a fazer a conta. Primeiro identificamos a estrutura, depois aplicamos a lei, depois conferimos.

## 0. As três operações

- + = OR
- · = AND
- ' = NOT

Tabelas básicas:

OR:
0+0=0
0+1=1
1+0=1
1+1=1

AND:
0·0=0
0·1=0
1·0=0
1·1=1

NOT:
0'=1
1'=0

**Não trate 1+1 como 2.**

---

# 1. Método para simplificar

Use:

**IDENTIFIQUE → APLIQUE UMA LEI → SIMPLIFIQUE → REPITA → CONFIRA**

Não tente enxergar cinco passos de uma vez.

---

# 2. Leis essenciais com cálculo

### Identidade

A+0=A
A·1=A

Exemplo:
X+0 = X.

### Nulo

A+1=1
A·0=0

### Complemento

A+A'=1
A·A'=0

### Idempotência

A+A=A
A·A=A

### Absorção

A+A·B=A
A(A+B)=A

---

# 3. Absorção na prática

Exemplo:

A + A·B

Existe A sozinho.

Então:

A + A·B = A

Resultado:

**A**

Outro exemplo:

A + A·B' + A·B

A sozinho já absorve as outras partes.

Resultado:

**A**

---

# 4. Fatoração

Exemplo:

AB + AC

Aparece A nos dois termos.

Fatore:

A(B+C)

Resultado:

**A(B+C)**

---

# 5. Adjacência algébrica

Exemplo:

A'B + AB

Passo 1:
B(A'+A)

Passo 2:
B·1

Passo 3:
B

Resultado:

**B**

A variável A desapareceu porque os casos A=0 e A=1 foram cobertos.

Essa é a mesma ideia usada no Karnaugh.

---

# 6. Distributiva especial

Uma identidade importante:

(A+B)(A+C)=A+BC

Exemplo:

(X+Y)(X+Z)

Resultado:

X+YZ

Se quiser verificar expandindo:

X·X + X·Z + Y·X + Y·Z

X + XZ + XY + YZ

Pela absorção:

X + YZ

---

# 7. DeMorgan: faça por camadas

Regras:

(A+B)'=A'B'

(AB)'=A'+B'

Use a frase:

**NEGA → TROCA O OPERADOR**

Exemplo:

(A+B')'

A negação transforma a soma em produto:

A'·(B')'

Depois a negação dupla:

(B')'=B

Resultado:

**A'B**

---

# 8. DeMorgan em várias camadas

Exemplo:

((A+B)(A+C))'

Comece pela camada externa:

(A+B)' + (A+C)'

Agora cada soma:

A'B' + A'C'

Fatore:

A'(B'+C')

Resultado:

**A'(B'+C')**

### Regra

**DE FORA PARA DENTRO.**

---

# 9. XOR e XNOR

XOR = diferentes:

A'B + AB'

Casos verdadeiros:
01 e 10.

XNOR = iguais:

AB + A'B'

Casos verdadeiros:
00 e 11.

Memória:

**XOR = DIFERENTE**

**XNOR = IGUAL**

---

# 10. SOP: como montar a partir da tabela

SOP procura as linhas em que F=1.

Exemplo:

| A | B | F |
|---|---|---|
| 0 | 0 | 1 |
| 0 | 1 | 0 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

Pegue 00:
A=0 → A'
B=0 → B'
Termo: A'B'

Pegue 10:
A=1 → A
B=0 → B'
Termo: AB'

Então:

F=A'B'+AB'

Fatorando:

F=B'(A'+A)

F=B'

Resultado:

**F=B'**

---

# 11. POS: procure F=0

POS segue o caminho inverso: comece nas linhas em que F=0.

Regra para um maxtermo:
- bit 0 → variável normal;
- bit 1 → variável complementada.

O principal para prova é lembrar:

**SOP → 1**

**POS → 0**

---

# 12. Mintermo: como calcular a posição

Exemplo:

m5 em três variáveis.

5 em binário:

101

Então:

A=1
B=0
C=1

Mintermo:

AB'C

Outro exemplo:

m2:

010

Logo:

A'BC'

---

# 13. Karnaugh: código Gray

Para quatro posições use:

00, 01, 11, 10

Não use:

00, 01, 10, 11

O Gray garante que posições vizinhas diferem em apenas uma variável.

---

# 14. Grupo de 2

Se duas células são vizinhas, uma variável muda.

Exemplo:

A'B + AB

A muda.

Logo A desaparece.

Resultado:

B

### Regra visual

**VARIÁVEL QUE MUDA → SOME.**

---

# 15. Grupo de 4

Exemplo:

A'B'C + A'BC + AB'C + ABC

Em todos os termos:

C=1

A e B variam.

Logo:

F=C

Resultado:

**C**

---

# 16. Grupo de 8

Em mapa de quatro variáveis, se oito células têm A=0 e as demais variam:

Resultado:

A'

Quanto maior o grupo, mais variáveis podem desaparecer.

---

# 17. Grupos permitidos

Use somente quantidades:

1, 2, 4, 8, 16.

Não crie grupos de 3, 5, 6 ou 7.

---

# 18. Bordas

Primeira e última coluna são adjacentes.

Primeira e última linha também podem ser adjacentes.

O mapa “dá a volta”.

---

# 19. Sobreposição

Uma célula pode aparecer em mais de um grupo.

Isso é permitido quando melhora a cobertura.

### Regra

**NÃO É “USAR CADA 1 UMA VEZ”. É “COBRIR TODOS OS 1 COM GRUPOS VÁLIDOS E SIMPLES”.**

---

# 20. Don't care

Um X pode ser tratado como 0 ou 1 quando isso ajudar.

Não é obrigatório usar.

Regra:

**USE SOMENTE SE MELHORAR A SIMPLIFICAÇÃO.**

---

# 21. Como fazer uma questão completa de Karnaugh

Use:

**MARQUE → AGRUPE 8 → AGRUPE 4 → AGRUPE 2 → AGRUPE 1 → ESCREVA OS CONSTANTES → SOME → CONFIRA**

Exemplo:
uns em m0,m1,m2,m3.

Essas quatro células correspondem a A=0.

Logo:

F=A'

---

# 22. Expressão para Karnaugh

Exemplo:

A'B'C' + A'BC' + AB'C

Converta cada termo:

A'B'C' → 000 → m0

A'BC' → 010 → m2

AB'C → 101 → m5

Marque m0, m2 e m5.

Só depois procure agrupamentos.

---

# 23. Como conferir

Se encontrou:

F=A+B'

teste uma entrada da tabela, por exemplo:

A=0, B=1

F=0+0=0

Compare com a função original.

Faça pelo menos um teste de F=0 e um de F=1.

---

# 24. DEU MERDA

## Erro: usar aritmética comum

**ERRO:** 1+1=2.

**CONSEQUÊNCIA:** expressão inteira errada.

**REGRA:** em OR, 1+1=1.

## Erro: DeMorgan incompleto

**ERRO:** negar as variáveis sem trocar + e ·.

**CONSEQUÊNCIA:** expressão não equivalente.

**REGRA:** negue termos e troque o operador.

## Erro: Gray errado

**ERRO:** 00,01,10,11.

**CONSEQUÊNCIA:** agrupamentos incorretos.

**REGRA:** 00,01,11,10.

---

# 25. Resumo de bolso

**Complemento:** A+A'=1 e A·A'=0.

**Absorção:** A+A·B=A.

**Adjacência:** A'B+AB=B.

**DeMorgan:** nega + troca operador.

**XOR:** diferentes.

**XNOR:** iguais.

**SOP:** F=1.

**POS:** F=0.

**Karnaugh:** Gray.

**Grupo:** 1/2/4/8/16.

**Maior grupo:** geralmente simplifica mais.

**Borda:** pode ser vizinha.

**Sobreposição:** permitida.

---

# 26. Checklist

- [ ] Sei as operações básicas.
- [ ] Sei absorção e fatoração.
- [ ] Sei complemento e adjacência.
- [ ] Sei distributiva especial.
- [ ] Sei DeMorgan.
- [ ] Sei XOR e XNOR.
- [ ] Sei montar SOP/POS.
- [ ] Sei converter mintermos.
- [ ] Sei Gray.
- [ ] Sei agrupar 1/2/4/8.
- [ ] Sei bordas e sobreposição.
- [ ] Sei conferir.

> **Fórmula de sobrevivência:** reduza uma etapa por vez; depois confira com um valor de entrada.
