# Sistemas Digitais — 50 exemplos resolvidos

Este arquivo contém 50 exemplos derivados dos tópicos da revisão correspondente. Cada exemplo traz a situação e o resultado esperado.

## 1. Identidade OR

**Exemplo:** `A+0`.

**Resultado:** `A`.

## 2. Identidade AND

**Exemplo:** `A·1`.

**Resultado:** `A`.

## 3. Elemento nulo

**Exemplo:** `A+1` e `A·0`.

**Resultado:** `1` e `0`.

## 4. Idempotência

**Exemplo:** `A+A` e `A·A`.

**Resultado:** `A` nos dois casos.

## 5. Complemento

**Exemplo:** `A+A'`.

**Resultado:** `1`.

## 6. Complemento

**Exemplo:** `A·A'`.

**Resultado:** `0`.

## 7. Absorção

**Exemplo:** `A+A·B`.

**Resultado:** `A`.

## 8. Absorção

**Exemplo:** `A·(A+B)`.

**Resultado:** `A`.

## 9. Distributiva

**Exemplo:** `A·(B+C)`.

**Resultado:** `AB+AC`.

## 10. Distributiva invertida

**Exemplo:** `(A+B)(A+C)`.

**Resultado:** `A+BC`.

## 11. Adjacência

**Exemplo:** `A'B+AB`.

**Resultado:** `B`.

## 12. DeMorgan

**Exemplo:** `(A+B)'`.

**Resultado:** `A'B'`.

## 13. DeMorgan

**Exemplo:** `(AB)'`.

**Resultado:** `A'+B'`.

## 14. DeMorgan 3 var

**Exemplo:** `(A+B+C)'`.

**Resultado:** `A'B'C'`.

## 15. DeMorgan produto 3

**Exemplo:** `(ABC)'`.

**Resultado:** `A'+B'+C'`.

## 16. Dupla negação

**Exemplo:** `((A+B)')'`.

**Resultado:** `A+B`.

## 17. XOR

**Exemplo:** `A'B+AB'`.

**Resultado:** 1 quando entradas diferem.

## 18. XNOR

**Exemplo:** `AB+A'B'`.

**Resultado:** 1 quando entradas são iguais.

## 19. AND 3 entradas

**Exemplo:** A=B=C=1.

**Resultado:** F=1; em qualquer outro caso, 0.

## 20. OR 3 entradas

**Exemplo:** A,B,C.

**Resultado:** F=0 somente em 000.

## 21. Tabela AND

**Exemplo:** Entradas 00,01,10,11.

**Resultado:** Saídas 0,0,0,1.

## 22. Tabela OR

**Exemplo:** Entradas 00,01,10,11.

**Resultado:** Saídas 0,1,1,1.

## 23. Tabela XOR

**Exemplo:** Entradas 00,01,10,11.

**Resultado:** Saídas 0,1,1,0.

## 24. Tabela XNOR

**Exemplo:** Entradas 00,01,10,11.

**Resultado:** Saídas 1,0,0,1.

## 25. SOP 2 entradas

**Exemplo:** Uns em 01 e 10.

**Resultado:** `A'B+AB'`.

## 26. POS 2 entradas

**Exemplo:** Zeros em 00 e 11.

**Resultado:** `(A+B)(A'+B')`.

## 27. K-map 2 var

**Exemplo:** Uns na coluna B=1.

**Resultado:** `B`.

## 28. K-map 2 var

**Exemplo:** Uns na linha A=0.

**Resultado:** `A'`.

## 29. K-map 3 var

**Exemplo:** Uns m0,m1,m2,m3.

**Resultado:** `A'`.

## 30. K-map 3 var

**Exemplo:** Uns m3,m7,m2,m6.

**Resultado:** `C`.

## 31. K-map 3 var

**Exemplo:** Uns m4,m5,m6,m7.

**Resultado:** `A`.

## 32. K-map 4 var

**Exemplo:** Grupo de 8 com A=0.

**Resultado:** `A'`.

## 33. K-map 4 var

**Exemplo:** Grupo de 4 com A=1,B=0.

**Resultado:** `AB'`.

## 34. Adjacência horizontal

**Exemplo:** Duas células vizinhas na mesma linha.

**Resultado:** A variável que muda desaparece.

## 35. Adjacência vertical

**Exemplo:** Duas células vizinhas na mesma coluna.

**Resultado:** A variável que muda desaparece.

## 36. Adjacência de borda

**Exemplo:** Primeira e última coluna.

**Resultado:** Podem formar grupo.

## 37. Sobreposição

**Exemplo:** Um 1 em dois grupos.

**Resultado:** É válida.

## 38. Grupo de 1

**Exemplo:** Uma célula isolada.

**Resultado:** Nenhum literal é eliminado.

## 39. Grupo de 2

**Exemplo:** Duas células adjacentes.

**Resultado:** Um literal é eliminado.

## 40. Grupo de 4

**Exemplo:** Quatro células adjacentes.

**Resultado:** Dois literais são eliminados.

## 41. Grupo de 8

**Exemplo:** Oito células adjacentes.

**Resultado:** Três literais são eliminados.

## 42. Função constante 1

**Exemplo:** Todas as células em 1.

**Resultado:** `F=1`.

## 43. Função constante 0

**Exemplo:** Todas as células em 0.

**Resultado:** `F=0`.

## 44. Teste de simplificação

**Exemplo:** `A+A'B`, com A=1,B=0.

**Resultado:** Original e `A+B` dão 1.

## 45. Teste de simplificação

**Exemplo:** `A+A'B`, com A=0,B=1.

**Resultado:** Original e `A+B` dão 1.

## 46. Maioria 3

**Exemplo:** F=1 quando ao menos duas entradas são 1.

**Resultado:** `AB+AC+BC`.

## 47. Exatamente uma

**Exemplo:** F=1 quando exatamente uma entrada é 1.

**Resultado:** `A'B'C+A'BC'+AB'C'`.

## 48. Circuito combinacional

**Exemplo:** Saída depende apenas das entradas atuais.

**Resultado:** Não há memória/estado nesse modelo.

## 49. Mintermo

**Exemplo:** ABC=101.

**Resultado:** `AB'C`.

## 50. Maxtermo

**Exemplo:** ABC=101 com F=0.

**Resultado:** `(A'+B+C')`.

## Observação

Os exemplos complementam a revisão existente e foram organizados para que o resultado apareça imediatamente após cada exercício.
