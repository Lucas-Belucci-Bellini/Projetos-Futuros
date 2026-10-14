# Álgebra Linear — Revisão V3 para Exame

> Versão de reforço baseada na revisão completa existente no projeto. A ideia é diminuir a carga de memorização: primeiro identifique a pegadinha, depois aplique a regra e só então faça a conta.

## 1. O mapa mental da disciplina

A matéria pode ser enxergada como uma cadeia:

MATRIZES → PRODUTO → INVERSA → DETERMINANTE → SISTEMAS LINEARES

O fio principal é:

A quadrada
↓
det(A) ≠ 0 → A⁻¹ existe → AX=B tem solução única
det(A) = 0 → A⁻¹ não existe → AX=B não é SPD
　　　　　　　　　　　　　　　├→ SPI
　　　　　　　　　　　　　　　└→ SI

### Frase para lembrar

**Determinante diferente de zero → inversa existe → solução única.**

E o caminho inverso:

**determinante zero → sem inversa → nunca solução única.**

---

# PARTE I — MATRIZES

## 2. Ordem e posição

Uma matriz de ordem m × n tem:

- m linhas;
- n colunas;
- m·n elementos.

Para aᵢⱼ:

**primeiro índice = linha**

**segundo índice = coluna**

### Pegadinha

a₂₃ não é linha 3 coluna 2.

### Exemplo

A = [1 2 3; 4 5 6]

a₂₃ = 6 e a ordem é 2×3.

---

## 3. Tipos de matriz

| Tipo | Como reconhecer |
|---|---|
| Nula | todos os elementos são 0 |
| Linha | uma única linha |
| Coluna | uma única coluna |
| Quadrada | linhas = colunas |
| Diagonal | tudo fora da diagonal principal é 0 |
| Identidade | diagonal principal = 1 e restante = 0 |
| Triangular superior | zeros abaixo da diagonal |
| Triangular inferior | zeros acima da diagonal |
| Simétrica | Aᵀ=A |
| Antissimétrica | Aᵀ=-A; diagonal principal é 0 |
| Ortogonal | A⁻¹=Aᵀ |

### Pegadinha das triangulares

**Superior = zeros abaixo.**

**Inferior = zeros acima.**

O nome se refere à região em que ficam os elementos permitidos.

---

## 4. Igualdade

A=B somente quando:

1. A e B têm a mesma ordem;
2. todos os elementos correspondentes são iguais.

A aparência não precisa ser idêntica.

Exemplo:

√25 = 5  
sen(30°) = 1/2  
-3/4 = -0,75  
π⁰ = 1

Valores equivalentes produzem matrizes iguais.

---

## 5. Transposta

A transposta troca linhas por colunas.

A = [1 2 3; 4 5 6]

Aᵀ = [1 4; 2 5; 3 6]

Se A é m×n, Aᵀ é n×m.

### Regras fundamentais

(Aᵀ)ᵀ = A  
(A+B)ᵀ = Aᵀ+Bᵀ  
(αA)ᵀ = αAᵀ  
(AB)ᵀ = BᵀAᵀ

### Pegadinha nº 1

**Na transposta do produto, a ordem inverte.**

Não é AᵀBᵀ.

---

# PARTE II — OPERAÇÕES COM MATRIZES

## 6. Adição e subtração

Só podem ser realizadas entre matrizes de **mesma ordem**.

A soma é elemento por elemento.

### Pegadinha

2×3 + 3×2 não existe.

---

## 7. Multiplicação por escalar

Cada elemento recebe a multiplicação pelo escalar.

Exemplo:

3·[2 -1; 0 4] = [6 -3; 0 12]

---

## 8. Produto de matrizes

Se A é m×n e B é n×p, então AB existe e é m×p.

### Macete

**As dimensões do meio precisam ser iguais.**

(m×n) · (n×p) = m×p

O cálculo de cada elemento é:

**linha da primeira × coluna da segunda, somando os produtos.**

Exemplo:

A=[1 2; 3 4]  
B=[5 6; 7 8]

AB=[19 22; 43 50]

### Pegadinhas

Produto não é elemento por elemento.

Em geral, AB ≠ BA.

Pode acontecer de AB existir e BA não existir.

---

# PARTE III — INVERSA

## 9. Definição

A⁻¹ é a matriz que satisfaz:

AA⁻¹ = A⁻¹A = I

A matriz precisa ser quadrada.

### Teste decisivo

det(A) ≠ 0 → inversa existe

det(A) = 0 → inversa não existe

### Método [A | I]

[A | I] → [I | A⁻¹]

As mesmas operações elementares devem ser aplicadas aos dois lados.

### Pegadinha

Alterar apenas um dos lados destrói o método.

---

# PARTE IV — DETERMINANTES

## 10. Determinante de ordem 2

Para:

[a b; c d]

det(A) = ad - bc

### Memória

**principal − secundária.**

Exemplo:

[3 4; 2 5]

det = 3·5 - 4·2 = 7

### Pegadinha

Não use ad+bc.

---

## 11. Regra de Sarrus

Sarrus é usada para **matrizes 3×3**.

Procedimento:

1. repita as duas primeiras colunas;
2. some as três diagonais principais;
3. some as três diagonais secundárias;
4. subtraia secundárias das principais.

### Memória

**3×3 → Sarrus.**

### Pegadinha

Não use Sarrus para 4×4.

---

## 12. Propriedades dos determinantes

| Operação | Efeito |
|---|---|
| Trocar duas linhas | troca o sinal |
| Multiplicar uma linha por k | multiplica det por k |
| Somar a uma linha um múltiplo de outra | não altera det |
| Matriz triangular | produto da diagonal |
| Linha nula | det=0 |
| Linhas iguais/proporcionais | det=0 |
| det(AB) | det(A)·det(B) |
| det(Aᵀ) | det(A) |
| det(kA), para n×n | kⁿdet(A) |

### Pegadinha mais importante

**Uma linha multiplicada por k → ×k.**

**A matriz inteira multiplicada por k → ×kⁿ.**

---

## 13. Laplace

Menor complementar Mᵢⱼ: retire a linha i e a coluna j.

Cofator:

Cᵢⱼ = (-1)^(i+j)Mᵢⱼ

Sinais:

+ - + -
- + - +
+ - + -
- + - +

Desenvolvimento:

det(A) = Σ aᵢⱼCᵢⱼ

### Estratégia

Escolha a linha ou coluna com **mais zeros**.

Zero elimina o cofator correspondente.

---

## 14. Inversa por cofatores

A⁻¹ = (1/det A)·adj(A)

A adjunta é a **transposta da matriz de cofatores**.

### Pegadinha nº 2

Calcular os cofatores e esquecer de transpor.

---

# PARTE V — MATRIZES ESPECIAIS

## 15. Idempotente

A²=A

Consequentemente:

A³=A  
A⁴=A  
A¹⁰⁰=A

### Memória

**Idempotente: multiplica novamente e permanece igual.**

---

## 16. Nihilpotente

Existe p>0 tal que Aᵖ=0.

É possível ter:

A²=0

mesmo com A≠0.

### Pegadinha

Isso é possível para matrizes.

---

## 17. Ortogonal

A⁻¹=Aᵀ

Equivalente a:

AAᵀ=AᵀA=I

---

# PARTE VI — SISTEMAS LINEARES

## 18. Equação linear

Forma geral:

a₁x₁+a₂x₂+...+aₙxₙ=b

É linear quando:

- cada incógnita aparece na primeira potência;
- não existe produto entre incógnitas.

### Linear

2x+3y=5

x-y+z=0

### Não linear

xy=4

x²+y=7

---

## 19. Sistema homogêneo

O termo independente é zero.

Exemplo:

2x-y+z=0  
x+3y-z=0

Todo sistema homogêneo possui a solução trivial:

x=0, y=0, z=0

### Pegadinha

**Sistema homogêneo nunca é SI.**

Pode ser SPD ou SPI.

---

## 20. Forma matricial

Todo sistema linear pode ser representado como:

AX=B

onde:

A = matriz dos coeficientes

X = vetor das incógnitas

B = vetor dos termos independentes

---

## 21. Classificação

**SPD — Sistema Possível e Determinado**

Uma única solução.

**SPI — Sistema Possível e Indeterminado**

Infinitas soluções.

**SI — Sistema Impossível**

Nenhuma solução.

### O detector de SI

Se aparecer:

0 0 0 | k

com k≠0, temos:

0=k

Isso é impossível.

### Linha nula

0 0 0 | 0

não é contradição. Pode indicar variável livre e SPI.

---

# PARTE VII — ESCALONAMENTO

## 22. Operações elementares

1. trocar duas linhas;
2. multiplicar uma linha por escalar não nulo;
3. somar a uma linha um múltiplo de outra.

As três aparecem em:

- sistemas;
- determinantes;
- inversa.

### Associação de memória

**Uma técnica, três assuntos.**

---

## 23. Forma escalonada

Os pivôs avançam para a direita à medida que descemos pelas linhas.

Exemplo:

[2 3 1; 0 4 5; 0 0 7]

está escalonada.

### Pegadinha

A quantidade de zeros iniciais precisa aumentar, mas não precisa aumentar exatamente de um em um.

---

# PARTE VIII — ALGORITMOS MENTAIS PARA A PROVA

## 24. Questão de matriz

Pergunte:

1. Qual é a ordem?
2. Essa operação é permitida?
3. Se for produto, as dimensões internas batem?
4. Se for determinante, a matriz é quadrada?
5. Se for inversa, det é diferente de zero?

---

## 25. Questão de determinante

**2×2 → ad-bc**

**3×3 → Sarrus**

**Muitos zeros → Laplace**

**Triangular → produto da diagonal**

**Troca de linhas → muda sinal**

**Multiplicação de uma linha → acompanha fator**

---

## 26. Questão de sistemas

1. É linear?
2. É homogêneo?
3. Escalone.
4. Apareceu 0=k com k≠0? → SI.
5. Existe variável livre? → SPI.
6. Todas as incógnitas ficaram determinadas? → SPD.

---

# PARTE IX — MEMÓRIA POR PEGADINHAS

## 27. Método ERRO → CONSEQUÊNCIA → REGRA → RESULTADO

A memória não precisa guardar uma página inteira. Ela precisa conseguir reconstruir a regra.

### Transposta

**Erro:** usar AᵀBᵀ.

**Consequência:** ordem errada.

**Regra:** (AB)ᵀ=BᵀAᵀ.

**Resultado:** a ordem dos fatores inverte.

### Determinante 2×2

**Erro:** fazer ad+bc.

**Consequência:** sinal errado.

**Regra:** ad-bc.

**Resultado:** [3 4; 2 5] → 7.

### Sistemas

**Erro:** chamar 0=7 de SPI.

**Consequência:** classificação errada.

**Regra:** 0=k, k≠0 → SI.

**Resultado:** nenhuma solução.

### det(kA)

**Erro:** usar só k.

**Consequência:** esquecer a dimensão.

**Regra:** para matriz n×n, det(kA)=kⁿdet(A).

**Resultado:** em 3×3, det(2A)=8det(A).

---

# PARTE X — CHECKLIST DE EXAME

### Matrizes
- [ ] Primeiro índice é linha.
- [ ] Segundo índice é coluna.
- [ ] Ordem = linhas × colunas.
- [ ] Transposta inverte a ordem.
- [ ] Soma exige mesma ordem.
- [ ] Produto exige dimensões internas iguais.
- [ ] AB pode ser diferente de BA.
- [ ] (AB)ᵀ=BᵀAᵀ.
- [ ] [A|I]→[I|A⁻¹].

### Determinantes
- [ ] Só existem para matrizes quadradas.
- [ ] 2×2 → ad-bc.
- [ ] 3×3 → Sarrus.
- [ ] Triangular → produto da diagonal.
- [ ] Trocar linhas → sinal muda.
- [ ] Multiplicar uma linha → ×k.
- [ ] Somar múltiplo de outra → não muda.
- [ ] det(kA)=kⁿdet(A).
- [ ] det(AB)=det(A)det(B).
- [ ] det(A)≠0 → inversa existe.
- [ ] adj(A) = transposta dos cofatores.

### Sistemas
- [ ] Reconheço linearidade.
- [ ] Homogêneo nunca é SI.
- [ ] 0=k, k≠0 → SI.
- [ ] Linha nula pode indicar SPI.
- [ ] Sei escalonar.
- [ ] Sei resolver de baixo para cima.

# Regra final

Quando der branco, não tente lembrar a página inteira.

Pergunte:

**“Qual pegadinha essa questão está tentando me fazer cometer?”**

Depois recupere a regra que elimina a pegadinha e reconstrua o resultado.
