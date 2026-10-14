# Algoritmos — V2: Memória e Pegadinhas

> Transformar teste de mesa, `i++`, erros, debugging e Javadoc em gatilhos de recuperação rápida.

## Método “DEU MERDA”

**ERRO → CONSEQUÊNCIA → REGRA → RESULTADO.** Use um erro fictício de exercício como gatilho de memória. A meta é lembrar a regra, não se punir pelo erro.

## 1. Como estudar

1. Cubra a consequência e o resultado.
2. Leia o erro e tente explicar por que ele acontece.
3. Recupere a regra sem consultar.
4. Resolva o exemplo.
5. Confira o resultado.

### 1. i++

**Erro:** Achar que aumenta de 2

**Consequência:** Confundir 0→2 com o incremento

**Regra:** i++ significa i=i+1

**Resultado:** 0→1→2→3

### 2. i+=2

**Erro:** Confundir com i++

**Consequência:** Sequência errada

**Regra:** i+=2 soma 2

**Resultado:** 0→2→4→6

### 3. Pós-incremento

**Erro:** Usar valor novo na mesma expressão

**Consequência:** Resultado errado

**Regra:** Usa primeiro, incrementa depois

**Resultado:** x=i++ com i=5 → x=5, i=6

### 4. Pré-incremento

**Erro:** Usar valor antigo

**Consequência:** Resultado errado

**Regra:** Incrementa primeiro

**Resultado:** x=++i com i=5 → x=6, i=6

### 5. for

**Erro:** Esquecer a condição

**Consequência:** Loop errado

**Regra:** Leia inicialização, condição e atualização

**Resultado:** for(i=0;i<5;i++) → 0..4

### 6. while

**Erro:** Ler como do-while

**Consequência:** Executar sem testar antes

**Regra:** while testa primeiro

**Resultado:** i=5; i<3 → 0 execuções

### 7. do-while

**Erro:** Achar que pode executar zero

**Consequência:** Resultado errado

**Regra:** Executa ao menos uma vez

**Resultado:** i=5 → corpo executa 1 vez

### 8. mod

**Erro:** Confundir resto

**Consequência:** Filtro errado

**Regra:** x%2==0 identifica pares

**Resultado:** 0,2,4,6...

### 9. Acumulador

**Erro:** Reiniciar total no loop

**Consequência:** Perder soma

**Regra:** Inicialize antes do loop

**Resultado:** 1+2+3+4+5=15

### 10. Busca linear

**Erro:** Confundir valor e índice

**Consequência:** Resposta deslocada

**Regra:** Memorize a posição

**Resultado:** 30 em [10,50,30] → índice 2

### 11. Maior/menor

**Erro:** Começar com 0

**Consequência:** Falhar com valores negativos

**Regra:** Use primeiro elemento como base

**Resultado:** [-4,-2,-8] → maior -2

### 12. Erro de sintaxe

**Erro:** Achar que todo erro é de execução

**Consequência:** Nem compila

**Regra:** Sintaxe é estrutura válida

**Resultado:** parêntese faltando → compilador reclama

### 13. Erro de execução

**Erro:** Chamar tudo de sintaxe

**Consequência:** Diagnóstico errado

**Regra:** Acontece durante execução

**Resultado:** índice inválido → exceção

### 14. Erro de lógica

**Erro:** Pensar que rodou = certo

**Consequência:** Aceitar resultado errado

**Regra:** A lógica pode estar incorreta

**Resultado:** dias<10 quando deveria ser >=10

### 15. Breakpoint

**Erro:** Parar sem saber por quê

**Consequência:** Observação inútil

**Regra:** Pare antes da instrução suspeita

**Resultado:** olhe variáveis e fluxo

### 16. Step Over

**Erro:** Entrar em todo método

**Consequência:** Depuração cansativa

**Regra:** Executa sem entrar

**Resultado:** F8

### 17. Step Into

**Erro:** Só querer avançar

**Consequência:** Entrar sem necessidade

**Regra:** Use para inspecionar método

**Resultado:** F7

### 18. Javadoc

**Erro:** Comentar cada linha

**Consequência:** Ruído

**Regra:** Documente intenção e contrato

**Resultado:** @param explica o parâmetro

### 19. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 20. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 21. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 22. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 23. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 24. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 25. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 26. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 27. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 28. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 29. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 30. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 31. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 32. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 33. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 34. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 35. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 36. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 37. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 38. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 39. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 40. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 41. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 42. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 43. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 44. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 45. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 46. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 47. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 48. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 49. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

### 50. Recuperação ativa

**Erro:** Ler a resposta antes de tentar

**Consequência:** Reconhecimento passivo

**Regra:** Tente lembrar primeiro e só depois confira

**Resultado:** Erro lembrado → regra recuperada → resposta reconstruída

## 2. Quando der branco na prova

Pare alguns segundos e pergunte: **qual é a pegadinha? qual regra elimina essa pegadinha? qual resultado sai dela?**. Reconstruir a resposta é mais útil do que tentar lembrar literalmente a página.

## 3. Revisão relâmpago

**Não decorar:** lembrar a cadeia **pegadinha → regra → exemplo → resultado**.
