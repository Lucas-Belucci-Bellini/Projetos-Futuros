# Álgebra Booleana e Karnaugh — 50 exemplos resolvidos

Este arquivo contém 50 exemplos derivados dos tópicos da revisão correspondente. Cada exemplo traz a situação e o resultado esperado.

## 1. Identidade OR

**Exemplo:** `A+0`.

**Resultado:** `A`.

## 2. Identidade AND

**Exemplo:** `A·1`.

**Resultado:** `A`.

## 3. Elemento nulo OR

**Exemplo:** `A+1`.

**Resultado:** `1`.

## 4. Elemento nulo AND

**Exemplo:** `A·0`.

**Resultado:** `0`.

## 5. Idempotência OR

**Exemplo:** `A+A`.

**Resultado:** `A`.

## 6. Idempotência AND

**Exemplo:** `A·A`.

**Resultado:** `A`.

## 7. Complemento OR

**Exemplo:** `A+A'`.

**Resultado:** `1`.

## 8. Complemento AND

**Exemplo:** `A·A'`.

**Resultado:** `0`.

## 9. Involução

**Exemplo:** `(A')'`.

**Resultado:** `A`.

## 10. Absorção 1

**Exemplo:** `A+A·B`.

**Resultado:** `A`.

## 11. Absorção 2

**Exemplo:** `A·(A+B)`.

**Resultado:** `A`.

## 12. Absorção com complemento

**Exemplo:** `A+A'·B`.

**Resultado:** `A+B`.

## 13. Adjacência

**Exemplo:** `A'·B+A·B`.

**Resultado:** `B`.

## 14. Distributiva

**Exemplo:** `A·(B+C)`.

**Resultado:** `A·B+A·C`.

## 15. Distributiva especial

**Exemplo:** `(A+B)·(A+C)`.

**Resultado:** `A+B·C`.

## 16. DeMorgan OR

**Exemplo:** `(A+B)'`.

**Resultado:** `A'·B'`.

## 17. DeMorgan AND

**Exemplo:** `(A·B)'`.

**Resultado:** `A'+B'`.

## 18. DeMorgan com negadas

**Exemplo:** `(A'·B·C')'`.

**Resultado:** `A+B'+C`.

## 19. Negação dupla

**Exemplo:** `((A+B)')'`.

**Resultado:** `A+B`.

## 20. Três negações

**Exemplo:** `(((A+B)')')'`.

**Resultado:** `A'·B'`.

## 21. SOP simples

**Exemplo:** F=1 em 001 e 010.

**Resultado:** `A'B'C + A'BC'`.

## 22. POS simples

**Exemplo:** F=0 em 000 e 111.

**Resultado:** Produto dos dois maxtermos correspondentes.

## 23. AND

**Exemplo:** A=1 e B=1.

**Resultado:** `A·B`.

## 24. OR

**Exemplo:** A=1 ou B=1.

**Resultado:** `A+B`.

## 25. XOR

**Exemplo:** `A'B+AB'`.

**Resultado:** 1 quando A e B são diferentes.

## 26. XNOR

**Exemplo:** `AB+A'B'`.

**Resultado:** 1 quando A e B são iguais.

## 27. Maioria de 3

**Exemplo:** F=1 quando pelo menos duas entradas são 1.

**Resultado:** `AB+AC+BC`.

## 28. Exatamente uma

**Exemplo:** F=1 quando exatamente uma entrada é 1.

**Resultado:** `A'B'C+A'BC'+AB'C'`.

## 29. K-map 2 var

**Exemplo:** Uns em 00 e 01.

**Resultado:** Grupo de 2 → `A'`.

## 30. K-map 2 var

**Exemplo:** Uns em 00 e 10.

**Resultado:** Grupo de 2 → `B'`.

## 31. K-map 3 var

**Exemplo:** Uns em m0,m1,m2,m3.

**Resultado:** Grupo de 4 → `A'`.

## 32. K-map 3 var

**Exemplo:** Uns em m1,m3,m5,m7.

**Resultado:** Grupo de 4 → `C`.

## 33. K-map isolado

**Exemplo:** Uns em m0 e m7.

**Resultado:** Não há agrupamento entre eles.

## 34. Sobreposição

**Exemplo:** Um 1 pertence a dois grupos.

**Resultado:** Sobreposição é permitida.

## 35. K-map 4 var

**Exemplo:** Grupo de 8.

**Resultado:** Três variáveis são eliminadas.

## 36. Mapa todo em 1

**Exemplo:** Todas as 16 células são 1.

**Resultado:** `F=1`.

## 37. SOP → K-map

**Exemplo:** `A'B'C+A'BC`.

**Resultado:** Marque m1 e m3; agrupamento → `A'C`.

## 38. K-map → SOP

**Exemplo:** Grupo com A=1,B=0.

**Resultado:** Termo `AB'`.

## 39. Maxtermo 000

**Exemplo:** ABC=000 é uma linha com F=0.

**Resultado:** Maxtermo `(A+B+C)`.

## 40. POS por zeros

**Exemplo:** F=0 em 001 e 110.

**Resultado:** Produto dos maxtermos m1 e m6.

## 41. Don't care usado

**Exemplo:** X adjacente a grupo de 1.

**Resultado:** Pode ampliar o grupo se reduzir a expressão.

## 42. Don't care ignorado

**Exemplo:** X sem benefício na simplificação.

**Resultado:** Pode ser ignorado.

## 43. Código Gray

**Exemplo:** Ordem das colunas de 2 bits.

**Resultado:** `00,01,11,10`.

## 44. Borda do mapa

**Exemplo:** Primeira e última coluna com 1.

**Resultado:** São adjacentes.

## 45. Grupo de 2

**Exemplo:** Duas células adjacentes.

**Resultado:** Uma variável desaparece.

## 46. Grupo de 4

**Exemplo:** Quatro células adjacentes.

**Resultado:** Duas variáveis desaparecem.

## 47. Grupo de 8

**Exemplo:** Oito células adjacentes.

**Resultado:** Três variáveis desaparecem.

## 48. Validação

**Exemplo:** Teste A=1,B=0 na expressão `A+B`.

**Resultado:** Resultado 1.

## 49. Circuito AND

**Exemplo:** Duas entradas em AND.

**Resultado:** Saída 1 apenas em 11.

## 50. Circuito OR

**Exemplo:** Duas entradas em OR.

**Resultado:** Saída 0 apenas em 00.

## Observação

Os exemplos complementam a revisão existente e foram organizados para que o resultado apareça imediatamente após cada exercício.
