# Álgebra Booleana e Karnaugh — V3 de Revisão para Exame

> Versão reforçada a partir da revisão existente, com foco em reconhecimento rápido e pegadinhas.

## Método de memória

**PEGADINHA → CONSEQUÊNCIA → REGRA → EXEMPLO → RESULTADO.**

Quando der branco, tente descobrir primeiro qual erro a questão quer provocar.

## Fórmulas essenciais

- Identidade: A+0=A e A·1=A.
- Nulo: A+1=1 e A·0=0.
- Complemento: A+A'=1 e A·A'=0.
- Absorção: A+A·B=A e A·(A+B)=A.
- Distributiva booleana: (A+B)(A+C)=A+BC.
- DeMorgan: (A+B)'=A'B' e (AB)'=A'+B'.
- XOR: A'B+AB'.
- XNOR: AB+A'B'.
- SOP usa linhas com F=1.
- POS usa linhas com F=0.
- K-map usa código Gray: 00,01,11,10.
- Grupos válidos têm 1,2,4,8,... células.
- Bordas do K-map são adjacentes.
- Don't care pode ser usado somente quando ajuda.

## Pegadinhas que mais custam ponto

### 1. Booleano não é aritmética comum
A+B não produz 2. OR produz 1 quando pelo menos uma entrada é 1.

### 2. DeMorgan troca o operador
Ao negar uma soma, vira produto; ao negar um produto, vira soma.

### 3. SOP e POS usam regras diferentes
SOP: bit 0 → variável complementada; bit 1 → variável normal.
POS: bit 0 → variável normal; bit 1 → variável complementada.

### 4. K-map não usa ordem binária comum
Use Gray: 00,01,11,10.

### 5. Grupo maior costuma simplificar mais
Procure primeiro 8, depois 4, depois 2, depois 1.

### 6. Isolado não pode ser inventado
Se duas células não são adjacentes, não podem formar grupo.

## Checklist

- [ ] Sei aplicar as oito identidades.
- [ ] Sei DeMorgan sem trocar sinais errados.
- [ ] Sei diferenciar XOR de XNOR.
- [ ] Sei montar SOP e POS.
- [ ] Sei numerar células em Gray.
- [ ] Sei agrupar 1/2/4/8 células.
- [ ] Sei usar bordas e sobreposição.
- [ ] Sei quando ignorar um don't care.
