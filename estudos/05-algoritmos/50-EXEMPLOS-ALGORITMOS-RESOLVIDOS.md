# Algoritmos — 50 exemplos resolvidos

Exemplos complementares derivados da revisão de algoritmos, teste de mesa, erros, debugging e Javadoc.

## 1. Sequência simples

**Exemplo:** `int a=2; int b=3; int c=a+b;`.

**Resultado:** c=5.

## 2. Condicional simples

**Exemplo:** `if(7>5)`.

**Resultado:** Condição verdadeira.

## 3. if/else

**Exemplo:** `if(3%2==0) ... else ...`.

**Resultado:** Resultado: ímpar.

## 4. for básico

**Exemplo:** `for(int i=0;i<5;i++) print(i);`.

**Resultado:** 0 1 2 3 4.

## 5. while básico

**Exemplo:** `int i=0; while(i<3){print(i);i++;}`.

**Resultado:** 0 1 2.

## 6. do-while

**Exemplo:** `i=5; do{...}while(i<3);`.

**Resultado:** Executa uma vez.

## 7. Somatório

**Exemplo:** Somar 1 até 5.

**Resultado:** 15.

## 8. Contagem de pares

**Exemplo:** 0 até 10.

**Resultado:** 6 pares.

## 9. Positivos

**Exemplo:** {-2,5,0,9,-1}.

**Resultado:** 2 positivos.

## 10. Negativos

**Exemplo:** {-2,5,0,9,-1}.

**Resultado:** 2 negativos.

## 11. Teste de mesa

**Exemplo:** `a=1;b=3;d=a+b`.

**Resultado:** d=4.

## 12. Condicional em mesa

**Exemplo:** `a=1,b=3`, testar `a+b>=5`.

**Resultado:** Falso.

## 13. Acumulador

**Exemplo:** `total=0`, i=0,1,2, somar i+1.

**Resultado:** 6.

## 14. Módulo

**Exemplo:** Somar pares de 1 a 5.

**Resultado:** 6.

## 15. Loops aninhados

**Exemplo:** i=1..3; j=1..i; total+=j.

**Resultado:** 10.

## 16. Fatorial

**Exemplo:** 5!.

**Resultado:** 120.

## 17. Fibonacci

**Exemplo:** Índice 6, começando 0,1.

**Resultado:** 8.

## 18. Maior de array

**Exemplo:** {3,9,2,7}.

**Resultado:** 9.

## 19. Menor de array

**Exemplo:** {12,4,8,2}.

**Resultado:** 2.

## 20. Busca linear

**Exemplo:** 30 em {10,50,30,40}.

**Resultado:** Índice 2.

## 21. Cópia de array

**Exemplo:** {1,2,3} para outro.

**Resultado:** {1,2,3}.

## 22. Inversão por índice

**Exemplo:** {10,20,30}.

**Resultado:** 30 20 10.

## 23. String charAt

**Exemplo:** Percorrer `JAVA`.

**Resultado:** J A V A.

## 24. Contar vogais

**Exemplo:** `programacao`.

**Resultado:** 5.

## 25. Contar espaços

**Exemplo:** `Aprender algoritmos é essencial`.

**Resultado:** 3.

## 26. Bubble Sort

**Exemplo:** {5,1,4,2}.

**Resultado:** {1,2,4,5}.

## 27. Matriz 2×2

**Exemplo:** {{1,2},{3,4}}.

**Resultado:** 1 2 3 4.

## 28. Diagonal

**Exemplo:** {{1,0},{0,1}}.

**Resultado:** 1 e 1.

## 29. Strings iguais

**Exemplo:** `teste` versus `teste`.

**Resultado:** true.

## 30. Intercalação

**Exemplo:** {1,3} e {2,4}.

**Resultado:** {1,2,3,4}.

## 31. Pós-incremento

**Exemplo:** `int i=5; int x=i++;`.

**Resultado:** x=5, i=6.

## 32. Pré-incremento

**Exemplo:** `int i=5; int x=++i;`.

**Resultado:** x=6, i=6.

## 33. while com i++ na condição

**Exemplo:** `while(i++<3)` com i=0.

**Resultado:** Imprime 1,2,3; final 4.

## 34. i++ em expressão

**Exemplo:** `(i++)*10` com i=2.

**Resultado:** 20; i=3.

## 35. Dois incrementos

**Exemplo:** `i++` no corpo e no cabeçalho.

**Resultado:** 0,2,4,6,8.

## 36. Argumento com pós-incremento

**Exemplo:** `processarItem(i++)`, i=1.

**Resultado:** Método recebe 1; i vira 2.

## 37. Ponteiro de escrita

**Exemplo:** `fila[ponteiro++]=valor`.

**Resultado:** Usa o índice atual e depois avança.

## 38. Contador de chamadas

**Exemplo:** `chamadas++`.

**Resultado:** 1,2,3,... a cada chamada.

## 39. Erro de sintaxe

**Exemplo:** `if (x > 2 {`

**Resultado:** Não compila.

## 40. Erro de execução

**Exemplo:** Acessar índice 10 de vetor de tamanho 5.

**Resultado:** Pode lançar ArrayIndexOutOfBoundsException.

## 41. Erro de lógica

**Exemplo:** Usar `dias<10` quando a regra é `dias>=10`.

**Resultado:** Executa, mas produz resultado errado.

## 42. Breakpoint

**Exemplo:** Parar em uma linha.

**Resultado:** Execução fica suspensa no ponto.

## 43. Breakpoint condicional

**Exemplo:** Condição `i==50`.

**Resultado:** Pausa quando i atingir 50.

## 44. Step Over

**Exemplo:** Executar chamada sem entrar.

**Resultado:** Avança para próxima linha.

## 45. Step Into

**Exemplo:** Entrar em `calcularTotal()`.

**Resultado:** Mostra a execução interna.

## 46. Step Out

**Exemplo:** Voltar de um método chamado.

**Resultado:** Conclui o método e retorna ao chamador.

## 47. Evaluate Expression

**Exemplo:** Avaliar `a+b` durante uma pausa.

**Resultado:** Mostra o valor atual de `a+b`.

## 48. Javadoc @param

**Exemplo:** Documentar um parâmetro.

**Resultado:** Explica seu significado/contrato.

## 49. Javadoc @throws

**Exemplo:** Documentar uma exceção.

**Resultado:** Registra quando ela pode ser lançada.

## 50. Rastreamento de variável

**Exemplo:** `x=2; x=x*3; x=x+1;`.

**Resultado:** x termina em 7.

