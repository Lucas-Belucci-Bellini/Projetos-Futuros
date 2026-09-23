# Sistemas Digitais 2 — 50 exemplos resolvidos

Exemplos complementares seguindo os assuntos cobrados no Caderno de Estudo nº 2: álgebra booleana, DeMorgan e mapas de Karnaugh.

## 1. Absorção aplicada

**Exemplo:** `A+A·B'+A·B`.

**Resultado:** `A`.

## 2. Distributiva especial

**Exemplo:** `(A+B)(A+C)`.

**Resultado:** `A+BC`.

## 3. Complemento interno

**Exemplo:** `A(B+B')C`.

**Resultado:** `AC`.

## 4. Adjacência

**Exemplo:** `A'B+AB`.

**Resultado:** `B`.

## 5. Absorção em dois fatores

**Exemplo:** `(A+AB)(B+AB)`.

**Resultado:** `AB`.

## 6. Agrupamento

**Exemplo:** `ABC+ABC'`.

**Resultado:** `AB`.

## 7. Absorção com complemento

**Exemplo:** `A+A'B`.

**Resultado:** `A+B`.

## 8. DeMorgan simples

**Exemplo:** `(A+B')'`.

**Resultado:** `A'B`.

## 9. DeMorgan três variáveis

**Exemplo:** `(A'BC')'`.

**Resultado:** `A+B'+C`.

## 10. DeMorgan em produto

**Exemplo:** `((A+B)(A+C))'`.

**Resultado:** `A'(B'+C')`.

## 11. DeMorgan com soma externa

**Exemplo:** `((AB)+C)'`.

**Resultado:** `(A'+B')C'`.

## 12. Negação sucessiva

**Exemplo:** `(((A+B)')')'`.

**Resultado:** `A'B'`.

## 13. Mapa 2 var

**Exemplo:** Uns em 00,10.

**Resultado:** `B'`.

## 14. Mapa 2 var

**Exemplo:** Uns em 00,01.

**Resultado:** `A'`.

## 15. Mapa 2 var

**Exemplo:** Uns em 01,11.

**Resultado:** `B`.

## 16. Mapa 3 var

**Exemplo:** Uns em m0,m1,m2,m3.

**Resultado:** `A'`.

## 17. Mapa 3 var

**Exemplo:** Uns em m4,m5,m6,m7.

**Resultado:** `A`.

## 18. Mapa 3 var

**Exemplo:** Uns em m1,m3,m5,m7.

**Resultado:** `C`.

## 19. Isolados

**Exemplo:** Uns em m1,m2,m4.

**Resultado:** Nenhum grupo de 2; mantém a soma dos mintermos.

## 20. Mapa 4 var

**Exemplo:** Grupo de 8 com A=0.

**Resultado:** `A'`.

## 21. Mapa 4 var

**Exemplo:** Grupo de 4 com A=1,B=0.

**Resultado:** `AB'`.

## 22. Sobreposição

**Exemplo:** m6 pertence a dois grupos.

**Resultado:** É válida quando melhora a cobertura.

## 23. Wrap-around

**Exemplo:** Primeira e última coluna com 1.

**Resultado:** Podem formar grupo.

## 24. Célula já coberta

**Exemplo:** m7 em grupo menor e maior.

**Resultado:** Pode ser reutilizada.

## 25. Mintermo m0

**Exemplo:** ABC=000.

**Resultado:** `A'B'C'`.

## 26. Mintermo m7

**Exemplo:** ABC=111.

**Resultado:** `ABC`.

## 27. Mintermo m5

**Exemplo:** ABC=101.

**Resultado:** `AB'C`.

## 28. Mintermo m2

**Exemplo:** ABC=010.

**Resultado:** `A'BC'`.

## 29. XNOR

**Exemplo:** `BD+B'D'`.

**Resultado:** 1 quando B=D.

## 30. XOR

**Exemplo:** `B'D+BD'`.

**Resultado:** 1 quando B≠D.

## 31. Exatamente uma

**Exemplo:** F=1 em 001,010,100.

**Resultado:** `A'B'C+A'BC'+AB'C'`.

## 32. Maioria de 3

**Exemplo:** F=1 quando peso é 2 ou 3.

**Resultado:** `AB+AC+BC`.

## 33. A=0

**Exemplo:** F=1 sempre que A=0.

**Resultado:** Grupo de 8 → `A'`.

## 34. A=0 ou 1111

**Exemplo:** F=1 se A=0 ou A=B=C=D=1.

**Resultado:** `A'+BCD`.

## 35. Validação

**Exemplo:** `A+B'` em A=0,B=1.

**Resultado:** 0.

## 36. Validação

**Exemplo:** `A+B'` em A=1,B=1.

**Resultado:** 1.

## 37. Validação

**Exemplo:** `A'B'C+A'BC'+AB'C'` em 100.

**Resultado:** 1.

## 38. Validação

**Exemplo:** Mesma função em 111.

**Resultado:** 0.

## 39. Linha inteira

**Exemplo:** Quatro células de uma linha são 1.

**Resultado:** Grupo de 4.

## 40. Coluna inteira

**Exemplo:** Quatro células de uma coluna são 1.

**Resultado:** Grupo de 4.

## 41. Grupo de 2

**Exemplo:** Duas células diferem por uma variável.

**Resultado:** Essa variável desaparece.

## 42. Grupo de 4

**Exemplo:** Quatro células variam duas variáveis.

**Resultado:** Duas variáveis desaparecem.

## 43. Grupo de 8

**Exemplo:** Oito células variam três variáveis.

**Resultado:** Três variáveis desaparecem.

## 44. Função constante

**Exemplo:** Todas as 16 células em 1.

**Resultado:** `F=1`.

## 45. Sem simplificação

**Exemplo:** Cada 1 é isolado.

**Resultado:** Nenhum agrupamento permitido.

## 46. SOP para mapa

**Exemplo:** `A'B'C'+A'BC'+AB'C'`.

**Resultado:** Marque m0,m2,m4.

## 47. Mapa para SOP

**Exemplo:** Grupo com A=0,C=1.

**Resultado:** Termo `A'C`.

## 48. Lei no mapa

**Exemplo:** `A'B+AB`.

**Resultado:** Adjacência → `B`.

## 49. Comparação com tabela

**Exemplo:** Testar expressão candidata em todas as entradas.

**Resultado:** É válida quando reproduz a tabela.

## 50. Mintermos complementares

**Exemplo:** Separar os casos F=1.

**Resultado:** Cada linha com F=1 vira um mintermo.

