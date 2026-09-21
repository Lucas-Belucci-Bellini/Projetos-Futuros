# Álgebra Linear — 50 exemplos resolvidos

Este arquivo contém 50 exemplos derivados dos tópicos da revisão correspondente. Cada exemplo traz a situação e o resultado esperado.

## 1. Matriz e ordem

**Exemplo:** A=[[1,2,3],[4,5,6]] tem 2 linhas e 3 colunas.

**Resultado:** Ordem: 2×3.

## 2. Elemento de matriz

**Exemplo:** A=[[7,8],[9,10]]. Determine a_21.

**Resultado:** a_21 é 9 (linha 2, coluna 1).

## 3. Lei de formação

**Exemplo:** A=[a_ij], com a_ij=i+j, ordem 2×3.

**Resultado:** A=[[2,3,4],[3,4,5]].

## 4. Matriz quadrada

**Exemplo:** A=[[2,1],[0,3]].

**Resultado:** A é quadrada de ordem 2.

## 5. Matriz identidade

**Exemplo:** I_3.

**Resultado:** I_3=[[1,0,0],[0,1,0],[0,0,1]].

## 6. Matriz nula

**Exemplo:** M de ordem 2×3 com todos os elementos zero.

**Resultado:** M=[[0,0,0],[0,0,0]].

## 7. Matriz diagonal

**Exemplo:** D=diag(2,5,7).

**Resultado:** D=[[2,0,0],[0,5,0],[0,0,7]].

## 8. Matriz triangular superior

**Exemplo:** A=[[2,1,3],[0,4,5],[0,0,6]].

**Resultado:** A é triangular superior.

## 9. Matriz triangular inferior

**Exemplo:** A=[[2,0,0],[3,4,0],[5,6,7]].

**Resultado:** A é triangular inferior.

## 10. Matriz simétrica

**Exemplo:** A=[[1,4],[4,2]].

**Resultado:** A^T=A; portanto é simétrica.

## 11. Igualdade de matrizes

**Exemplo:** A=[[1,2],[3,4]] e B=[[1,2],[3,4]].

**Resultado:** A=B.

## 12. Transposta

**Exemplo:** A=[[1,2,3],[4,5,6]].

**Resultado:** A^T=[[1,4],[2,5],[3,6]].

## 13. Transposta duas vezes

**Exemplo:** A=[[2,7],[3,9]].

**Resultado:** (A^T)^T=A.

## 14. Soma de matrizes

**Exemplo:** A=[[1,2],[3,4]], B=[[5,6],[7,8]].

**Resultado:** A+B=[[6,8],[10,12]].

## 15. Subtração de matrizes

**Exemplo:** A=[[8,6],[4,2]], B=[[1,2],[3,4]].

**Resultado:** A-B=[[7,4],[1,-2]].

## 16. Multiplicação por escalar

**Exemplo:** 3·[[1,-2],[4,0]].

**Resultado:** [[3,-6],[12,0]].

## 17. Compatibilidade de produto

**Exemplo:** A é 2×3 e B é 3×4.

**Resultado:** AB é possível e terá ordem 2×4.

## 18. Produto 2×2

**Exemplo:** A=[[1,2],[3,4]], B=[[5,6],[7,8]].

**Resultado:** AB=[[19,22],[43,50]].

## 19. Produto matriz-vetor

**Exemplo:** A=[[2,1],[1,3]], x=[4,2]^T.

**Resultado:** Ax=[10,10]^T.

## 20. Não comutatividade

**Exemplo:** A=[[1,2],[0,1]], B=[[1,0],[3,1]].

**Resultado:** AB=[[7,2],[3,1]] e BA=[[1,2],[3,7]]; AB≠BA.

## 21. Matriz inversa 2×2

**Exemplo:** A=[[2,1],[1,1]].

**Resultado:** A^-1=[[1,-1],[-1,2]].

## 22. Verificação da inversa

**Exemplo:** A=[[2,1],[1,1]], A^-1=[[1,-1],[-1,2]].

**Resultado:** AA^-1=I_2.

## 23. Inversa e determinante

**Exemplo:** det(A)=5.

**Resultado:** det(A^-1)=1/5.

## 24. Condição de invertibilidade

**Exemplo:** A=[[1,2],[2,4]].

**Resultado:** det(A)=0; A não é invertível.

## 25. Troca de linhas

**Exemplo:** Trocar L1 e L2 em A.

**Resultado:** O determinante troca de sinal.

## 26. Escalonamento

**Exemplo:** [[1,2],[2,4]] → L2←L2-2L1.

**Resultado:** [[1,2],[0,0]].

## 27. Potência de matriz

**Exemplo:** A=I_2.

**Resultado:** A^5=I_2.

## 28. Matriz idempotente

**Exemplo:** A=[[1,0],[0,0]].

**Resultado:** A^2=A.

## 29. Matriz nilpotente

**Exemplo:** N=[[0,1],[0,0]].

**Resultado:** N^2=0.

## 30. Matriz ortogonal

**Exemplo:** Q=[[0,1],[-1,0]].

**Resultado:** Q^TQ=I_2.

## 31. Matriz periódica

**Exemplo:** A=diag(1,-1).

**Resultado:** A^2=I_2; as potências se repetem.

## 32. Determinante 2×2

**Exemplo:** A=[[3,4],[2,5]].

**Resultado:** det(A)=7.

## 33. Determinante 2×2 negativo

**Exemplo:** A=[[1,4],[3,2]].

**Resultado:** det(A)=-10.

## 34. Sarrus 3×3

**Exemplo:** A=[[1,2,3],[0,1,4],[5,6,0]].

**Resultado:** det(A)=1.

## 35. Determinante da identidade

**Exemplo:** I_4.

**Resultado:** det(I_4)=1.

## 36. Linha duplicada

**Exemplo:** A=[[1,2],[1,2]].

**Resultado:** det(A)=0.

## 37. Linha nula

**Exemplo:** A=[[1,2],[0,0]].

**Resultado:** det(A)=0.

## 38. Escalar no determinante

**Exemplo:** A é 3×3 e B=2A.

**Resultado:** det(B)=8det(A).

## 39. Laplace

**Exemplo:** A=[[2,1,0],[3,4,5],[1,0,2]], expansão pela 1ª linha.

**Resultado:** det(A)=10.

## 40. Menor complementar

**Exemplo:** A=[[1,2],[3,4]], M_11.

**Resultado:** M_11=4.

## 41. Cofator

**Exemplo:** A=[[1,2],[3,4]], C_12.

**Resultado:** C_12=-3.

## 42. Sistema 2×2

**Exemplo:** x+y=5; x-y=1.

**Resultado:** x=3, y=2.

## 43. Substituição

**Exemplo:** 2x+y=7; y=3.

**Resultado:** x=2.

## 44. Sistema indeterminado

**Exemplo:** x+y=2; 2x+2y=4.

**Resultado:** Infinitas soluções.

## 45. Sistema impossível

**Exemplo:** x+y=2; x+y=5.

**Resultado:** Sem solução.

## 46. Sistema determinado

**Exemplo:** x+y=5; x-y=1.

**Resultado:** Única solução: (3,2).

## 47. Forma matricial

**Exemplo:** 2x+3y=7; x-y=1.

**Resultado:** A=[[2,3],[1,-1]], X=[x,y]^T, B=[7,1]^T.

## 48. Escalonamento 3×3

**Exemplo:** x+y+z=6; y+z=4; z=2.

**Resultado:** z=2, y=2, x=2.

## 49. Classificação por determinante

**Exemplo:** Matriz quadrada com det(A)≠0.

**Resultado:** Sistema associado tem solução única.

## 50. Contradição na escalonação

**Exemplo:** Surge a linha [0 0 | 3].

**Resultado:** Sistema impossível.

## Observação

Os exemplos complementam a revisão existente e foram organizados para que o resultado apareça imediatamente após cada exercício.
