# Resultados — 50 exemplos de i++

Este arquivo complementa os 50 exemplos de `i++` fornecidos no material de estudo, colocando o resultado esperado junto de cada exemplo.

## 1–10 — Loops e contagens

1. **Contagem de 0 a 4** — Resultado: `0, 1, 2, 3, 4`.
2. **Contagem de 1 a 10** — Resultado: `1, 2, 3, 4, 5, 6, 7, 8, 9, 10`.
3. **Repetir mensagem 3 vezes** — Resultado: `A processar...` aparece 3 vezes.
4. **Somatório de 1 a 5** — Resultado: `15`.
5. **Tabuada do 5** — Resultado: `5, 10, 15, 20, 25, 30, 35, 40, 45, 50`.
6. **while de 0 a 4** — Resultado: `0, 1, 2, 3, 4`.
7. **do-while** — Resultado: `Passo 1`, `Passo 2`, `Passo 3`.
8. **Pares de 0 a 10** — Resultado: `0, 2, 4, 6, 8, 10`.
9. **Ímpares de 1 a 10** — Resultado: `1, 3, 5, 7, 9`.
10. **2 elevado a 4** — Resultado: `16`.

## 11–20 — Arrays, listas e Strings

11. **Percorrer array `{10,20,30,40}`** — Resultado: `10, 20, 30, 40`.
12. **Somar `{5,10,15}`** — Resultado: `30`.
13. **Maior de `{3,9,2,7}`** — Resultado: `9`.
14. **Menor de `{12,4,8,2}`** — Resultado: `2`.
15. **Preencher vetor de tamanho 5** — Resultado: `[1, 2, 3, 4, 5]`.
16. **Copiar `{1,2,3}`** — Resultado: `[1, 2, 3]`.
17. **Exibição invertida de `{10,20,30}`** — Resultado: `30, 20, 10`.
18. **`buffer[i++] = ...`** — Resultado: `buffer = [100, 200, 0]` e `i = 2`.
19. **Percorrer `JAVA`** — Resultado: `J`, `A`, `V`, `A`.
20. **Percorrer `List` de nomes** — Resultado: `Ana`, `Bruno`, `Carlos`.

## 21–30 — Contadores e condições

21. **Contar `a` em `banana`** — Resultado: `3`.
22. **Contar vogais em `programacao`** — Resultado: `5`.
23. **Contar positivos em `{-2,5,0,9,-1}`** — Resultado: `2`.
24. **Contar negativos em `{-2,5,0,9,-1}`** — Resultado: `2`.
25. **Tentativas de login** — Resultado máximo: `3`; se houver sucesso antes, pode terminar com `1`, `2` ou `3`.
26. **Divisores de 7** — Resultado: `2 divisores`; `7` é primo (`true`).
27. **Contar linhas de arquivo** — Resultado: igual ao número de linhas efetivamente lidas; depende do arquivo.
28. **Contar espaços em `Aprender algoritmos é essencial`** — Resultado: `3`.
29. **Contar pares em `{1,2,3,4,5,6}`** — Resultado: `3`.
30. **Notas >= 10 em `{12.5,8.0,15.0,9.5,18.0}`** — Resultado: `3 aprovados`.

## 31–40 — Comportamento do pós-incremento

31. **`int i=5; int a=i++;`** — Resultado: `a=5` e depois `i=6`.
32. **`int i=5; int b=++i;`** — Resultado: `b=6` e `i=6`.
33. **`System.out.println(i++)` com `i=10`** — Resultado: imprime `10`; depois `i=11`.
34. **`while (i++ < 3)` começando em 0** — Resultado: imprime `1, 2, 3`; valor final de `i = 4`.
35. **`(i++) * 10` com `i=2`** — Resultado: `resultado=20`; `i=3`.
36. **`i++` no corpo e no cabeçalho do `for`** — Resultado: `0, 2, 4, 6, 8`.
37. **`processarItem(i++)` com `i=1`** — Resultado: o método recebe `1`; depois `i=2`. A saída interna depende do método.
38. **`fila[ponteiro++]`** — Exemplo com 10, 20 e 30: `[10,20,30,0,0]`; ponteiro final `3`.
39. **Processar 10 posições** — Resultado: `processados=10` e `i=10`.
40. **`i++ + i++` começando em 0** — Resultado: `x=1` e `i=2`.

## 41–50 — Algoritmos clássicos

41. **Matriz `{{1,2},{3,4}}`** — Resultado: `1 2 3 4`.
42. **Triângulo de 4 linhas** — Resultado: `*`, `**`, `***`, `****`.
43. **Pesquisa de 30 em `{10,50,30,40}`** — Resultado: posição `2`.
44. **Bubble Sort de `{5,1,4,2}`** — Resultado: `[1, 2, 4, 5]`.
45. **Fatorial de 5** — Resultado: `120`.
46. **Fibonacci com n=6** — Resultado: `8`; sequência: `0,1,1,2,3,5,8`.
47. **Diagonal de `{{1,0},{0,1}}`** — Resultado: `1` e `1`.
48. **Comparar `teste` com `teste`** — Resultado: `true`.
49. **Intercalar `{1,3}` e `{2,4}`** — Resultado: `[1, 2, 3, 4]`; índice final `4`.
50. **Contador estático chamado 3 vezes** — Resultado: `Execução nº: 1`, `Execução nº: 2`, `Execução nº: 3`.

## Regra fundamental do `i++`

`i++` é pós-incremento: o valor atual é utilizado primeiro e `i` é incrementado depois.

Exemplo: `int i = 5; int x = i++;`

Resultado: `x = 5` e `i = 6`.

Comparação: `++i` é pré-incremento. `int i = 5; int x = ++i;` produz `x = 6` e `i = 6`.
