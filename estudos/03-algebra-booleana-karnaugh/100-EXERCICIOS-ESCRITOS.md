# Álgebra Booleana e Karnaugh — 50 Exercícios Escritos

> Resolva sem consultar. Depois compare o desenvolvimento com a solução-modelo.

## 1. Simplifique A+0.

**Solução:** A.

## 2. Simplifique A·1.

**Solução:** A.

## 3. Simplifique A+1.

**Solução:** 1.

## 4. Simplifique A·0.

**Solução:** 0.

## 5. Simplifique A+A.

**Solução:** A.

## 6. Simplifique A·A.

**Solução:** A.

## 7. Simplifique A+A'.

**Solução:** 1.

## 8. Simplifique A·A'.

**Solução:** 0.

## 9. Simplifique (A')'.

**Solução:** A.

## 10. Simplifique A+A·B.

**Solução:** A, pela absorção.

## 11. Simplifique A·(A+B).

**Solução:** A.

## 12. Simplifique A+A'B.

**Solução:** A+B.

## 13. Simplifique A'B+AB.

**Solução:** B.

## 14. Expanda A(B+C).

**Solução:** AB+AC.

## 15. Simplifique (A+B)(A+C).

**Solução:** A+BC.

## 16. Aplique DeMorgan a (A+B)'.

**Solução:** A'B'.

## 17. Aplique DeMorgan a (AB)'.

**Solução:** A'+B'.

## 18. Simplifique (A'BC')'.

**Solução:** A+B'+C.

## 19. Simplifique ((A+B)')'.

**Solução:** A+B.

## 20. Simplifique (((A+B)')')'.

**Solução:** A'B'.

## 21. Monte a SOP para F=1 em 001.

**Solução:** A'B'C.

## 22. Monte a SOP para F=1 em 101.

**Solução:** AB'C.

## 23. Monte o maxtermo para 000.

**Solução:** A+B+C.

## 24. Monte o maxtermo para 101.

**Solução:** A'+B+C'.

## 25. Escreva XOR de A e B.

**Solução:** A'B+AB'.

## 26. Escreva XNOR de A e B.

**Solução:** AB+A'B'.

## 27. Uma função vale 1 com pelo menos duas entradas em 1. Escreva a maioria de 3.

**Solução:** AB+AC+BC.

## 28. Uma função vale 1 com exatamente uma entrada em 1.

**Solução:** A'B'C+A'BC'+AB'C'.

## 29. K-map 2 variáveis com 1 em 00 e 01.

**Solução:** Grupo de 2 → A'.

## 30. K-map 2 variáveis com 1 em 00 e 10.

**Solução:** Grupo de 2 → B'.

## 31. K-map 2 variáveis com 1 em 01 e 11.

**Solução:** Grupo de 2 → B.

## 32. K-map 3 variáveis com 1 em m0,m1,m2,m3.

**Solução:** Grupo de 4 → A'.

## 33. K-map 3 variáveis com 1 em m4,m5,m6,m7.

**Solução:** Grupo de 4 → A.

## 34. K-map 3 variáveis com 1 em m1,m3,m5,m7.

**Solução:** Grupo de 4 → C.

## 35. Uns isolados em m1,m2,m4.

**Solução:** Não há agrupamento de 2; mantém os mintermos.

## 36. Grupo de 8 num K-map de 4 variáveis com A=0.

**Solução:** Termo A'.

## 37. Grupo de 4 com A=1 e B=0.

**Solução:** Termo AB'.

## 38. As primeiras e últimas colunas têm 1s.

**Solução:** Elas podem formar grupo por adjacência de borda.

## 39. Um 1 pertence a dois grupos.

**Solução:** A sobreposição é permitida.

## 40. Há um X adjacente a um grupo de 1.

**Solução:** O X pode ser usado se ampliar o grupo.

## 41. Há um X sem benefício.

**Solução:** Pode ser ignorado.

## 42. Qual é a ordem Gray de duas variáveis?

**Solução:** 00,01,11,10.

## 43. Por que grupo de 4 é melhor que grupo de 2?

**Solução:** Porque elimina duas variáveis em vez de uma.

## 44. O que acontece com uma variável que muda dentro do grupo?

**Solução:** Ela desaparece do termo.

## 45. Converta A'BC em mintermo de 3 variáveis.

**Solução:** m3.

## 46. Converta ABC em mintermo de 3 variáveis.

**Solução:** m7.

## 47. Converta A'B'C em mintermo de 3 variáveis.

**Solução:** m1.

## 48. Converta AB'C' em mintermo de 3 variáveis.

**Solução:** m4.

## 49. Faça a tabela XOR para 00,01,10,11.

**Solução:** 0,1,1,0.

## 50. Faça a tabela XNOR para 00,01,10,11.

**Solução:** 1,0,0,1.

## 51. Explique por que duas células diagonais não formam grupo.

**Solução:** Elas diferem em duas variáveis, não em uma.

## 52. Simplifique A+AB+AB'.

**Solução:** A, pois A absorve os dois termos.

