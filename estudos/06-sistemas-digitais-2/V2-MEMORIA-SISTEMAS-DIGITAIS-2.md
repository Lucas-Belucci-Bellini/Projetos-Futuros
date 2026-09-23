# Sistemas Digitais 2 — V2: Memória e Pegadinhas

> Transformar as pegadinhas de álgebra booleana, DeMorgan e Karnaugh do caderno SD2 em gatilhos de recuperação rápida.

## Método “DEU MERDA”

**ERRO → CONSEQUÊNCIA → REGRA → RESULTADO.** Use um erro fictício de exercício como gatilho de memória. A meta é lembrar a regra, não se punir pelo erro.

## 1. Como estudar

1. Cubra a consequência e o resultado.
2. Leia o erro e tente explicar por que ele acontece.
3. Recupere a regra sem consultar.
4. Resolva o exemplo.
5. Confira o resultado.

### 1. Absorção

**Erro:** A+A·B

**Consequência:** Conta maior

**Regra:** A+A·B=A

**Resultado:** A

### 2. Distributiva especial

**Erro:** (A+B)(A+C)

**Consequência:** Usar álgebra comum

**Regra:** É identidade booleana

**Resultado:** A+BC

### 3. Complemento

**Erro:** A(B+B')C

**Consequência:** Carregar B na resposta

**Regra:** B+B'=1

**Resultado:** AC

### 4. Adjacência

**Erro:** A'B+AB

**Consequência:** Manter A

**Regra:** A'+A=1

**Resultado:** B

### 5. Absorção em fatores

**Erro:** (A+AB)(B+AB)

**Consequência:** Expandir tudo

**Regra:** Simplifique cada fator primeiro

**Resultado:** AB

### 6. Agrupamento

**Erro:** ABC+ABC'

**Consequência:** Manter C

**Regra:** C+C'=1

**Resultado:** AB

### 7. Absorção com complemento

**Erro:** A+A'B

**Consequência:** Achar que é A

**Regra:** A+A'B=A+B

**Resultado:** A+B

### 8. DeMorgan

**Erro:** (A+B')'

**Consequência:** Negar tudo sem trocar operador

**Regra:** Troque + por · e negue termos

**Resultado:** A'B

### 9. DeMorgan 3 variáveis

**Erro:** (A'BC')'

**Consequência:** Esquecer negação dupla

**Regra:** Cada variável inverte

**Resultado:** A+B'+C

### 10. DeMorgan em produto

**Erro:** ((A+B)(A+C))'

**Consequência:** Negar camada errada

**Regra:** Aplique de fora para dentro

**Resultado:** A'(B'+C')

### 11. DeMorgan externo

**Erro:** ((AB)+C)'

**Consequência:** Trocar só variáveis

**Regra:** Soma externa vira produto

**Resultado:** (A'+B')C'

### 12. Negação sucessiva

**Erro:** (((A+B)')')'

**Consequência:** Contar errado

**Regra:** Duas cancelam; sobra uma

**Resultado:** A'B'

### 13. K-map 2 var

**Erro:** Uns em 00,10

**Consequência:** Ignorar coluna

**Regra:** Agrupe as duas

**Resultado:** B'

### 14. K-map 2 var

**Erro:** Uns em 00,01

**Consequência:** Usar B

**Regra:** Linha A=0

**Resultado:** A'

### 15. K-map 2 var

**Erro:** Uns em 01,11

**Consequência:** Manter A

**Regra:** Coluna B=1

**Resultado:** B

### 16. K-map 3 var

**Erro:** m0,m1,m2,m3

**Consequência:** Fazer grupos aleatórios

**Regra:** Grupo de 4

**Resultado:** A'

### 17. K-map 3 var

**Erro:** m4,m5,m6,m7

**Consequência:** Não enxergar linha

**Regra:** Grupo de 4

**Resultado:** A

### 18. K-map 3 var

**Erro:** m1,m3,m5,m7

**Consequência:** Confundir variável

**Regra:** Grupo de 4

**Resultado:** C

### 19. Isolados

**Erro:** m1,m2,m4

**Consequência:** Forçar grupo

**Regra:** Se não são adjacentes, não agrupe

**Resultado:** Mantém mintermos

### 20. K-map 4 var

**Erro:** Grupo de 8 com A=0

**Consequência:** Usar só 4

**Regra:** Grupo maior é melhor

**Resultado:** A'

### 21. K-map 4 var

**Erro:** Grupo de 4 A=1,B=0

**Consequência:** Adicionar variável

**Regra:** Só permanecem variáveis fixas

**Resultado:** AB'

### 22. Sobreposição

**Erro:** m6 aparece em dois grupos

**Consequência:** Achar que é proibido

**Regra:** Sobreposição é permitida

**Resultado:** Pode reduzir a expressão

### 23. Wrap-around

**Erro:** Primeira e última coluna

**Consequência:** Achar que não encostam

**Regra:** K-map tem adjacência circular

**Resultado:** Grupo válido

### 24. Célula coberta

**Erro:** m7 já está em grupo

**Consequência:** Proibir reutilização

**Regra:** Pode reutilizar

**Resultado:** Grupo maior pode reduzir

### 25. Mintermo m0

**Erro:** ABC=000

**Consequência:** Inverter bit 0 errado

**Regra:** 0→complemento

**Resultado:** A'B'C'

### 26. Mintermo m7

**Erro:** ABC=111

**Consequência:** Complementar tudo

**Regra:** 1→normal

**Resultado:** ABC

### 27. Mintermo m5

**Erro:** ABC=101

**Consequência:** Errar B

**Regra:** 0→complemento

**Resultado:** AB'C

### 28. Mintermo m2

**Erro:** ABC=010

**Consequência:** Errar A/C

**Regra:** 0→complemento

**Resultado:** A'BC'

### 29. XNOR

**Erro:** BD+B'D'

**Consequência:** Confundir com XOR

**Regra:** Igual → 1

**Resultado:** 1 quando B=D

### 30. XOR

**Erro:** B'D+BD'

**Consequência:** Confundir com XNOR

**Regra:** Diferente → 1

**Resultado:** 1 quando B≠D

### 31. Exatamente uma

**Erro:** 001,010,100

**Consequência:** Usar maioria

**Regra:** Peso 1 apenas

**Resultado:** A'B'C+A'BC'+AB'C'

### 32. Maioria

**Erro:** Peso 2 ou 3

**Consequência:** Usar exatamente uma

**Regra:** AB+AC+BC

**Resultado:** Maioria de 3

### 33. A=0

**Erro:** F=1 para A=0

**Consequência:** Usar A

**Regra:** Grupo de 8

**Resultado:** A'

### 34. A=0 ou 1111

**Erro:** F=1 se A=0 ou tudo 1

**Consequência:** Escrever mintermos sem agrupar

**Regra:** Grupo de 8 + par

**Resultado:** A'+BCD

### 35. Validação

**Erro:** A+B' em 0,1

**Consequência:** Não testar

**Regra:** Substitua entradas

**Resultado:** 0

### 36. Validação

**Erro:** A+B' em 1,1

**Consequência:** Olhar só B

**Regra:** Substitua entradas

**Resultado:** 1

### 37. Validação

**Erro:** Exatamente uma em 100

**Consequência:** Achar que 100 é 0

**Regra:** Peso de 100=1

**Resultado:** 1

### 38. Validação

**Erro:** Exatamente uma em 111

**Consequência:** Achar que três contém uma

**Regra:** Peso=3

**Resultado:** 0

### 39. Linha inteira

**Erro:** 4 células da linha=1

**Consequência:** Fazer 2 grupos

**Regra:** Grupo de 4

**Resultado:** 1 termo

### 40. Coluna inteira

**Erro:** 4 células da coluna=1

**Consequência:** Ignorar coluna

**Regra:** Grupo de 4

**Resultado:** 1 termo

### 41. Grupo de 2

**Erro:** Duas células adjacentes

**Consequência:** Agrupar diagonal

**Regra:** Só vizinhas por uma variável

**Resultado:** 1 variável eliminada

### 42. Grupo de 4

**Erro:** Bloco 2×2

**Consequência:** Usar 3

**Regra:** Retângulo válido

**Resultado:** 2 variáveis eliminadas

### 43. Grupo de 8

**Erro:** Metade do mapa

**Consequência:** Não aproveitar

**Regra:** Grupo 8

**Resultado:** 3 variáveis eliminadas

### 44. Constante 1

**Erro:** Todas as 16 são 1

**Consequência:** Procurar termos

**Regra:** Função constante

**Resultado:** F=1

### 45. Sem simplificação

**Erro:** Todos os 1 isolados

**Consequência:** Forçar agrupamento

**Regra:** Mantém mintermos

**Resultado:** Sem redução

### 46. SOP para mapa

**Erro:** A'B'C'+A'BC'+AB'C'

**Consequência:** Marcar célula errada

**Regra:** 0→complemento,1→normal

**Resultado:** m0,m2,m4

### 47. Mapa para SOP

**Erro:** Grupo A=0,C=1

**Consequência:** Incluir B

**Regra:** B varia

**Resultado:** A'C

### 48. Lei no mapa

**Erro:** A'B+AB

**Consequência:** Escrever 2 termos

**Regra:** Adjacência elimina A

**Resultado:** B

### 49. Tabela final

**Erro:** Comparar expressão e tabela

**Consequência:** Conferir só uma linha

**Regra:** Teste todas as combinações quando necessário

**Resultado:** Mesma tabela → equivalente

### 50. Mintermos

**Erro:** F=1 em algumas linhas

**Consequência:** Esquecer uma linha

**Regra:** Cada F=1 gera mintermo

**Resultado:** SOP completa

## 2. Quando der branco na prova

Pare alguns segundos e pergunte: **qual é a pegadinha? qual regra elimina essa pegadinha? qual resultado sai dela?**. Reconstruir a resposta é mais útil do que tentar lembrar literalmente a página.

## 3. Revisão relâmpago

**Não decorar:** lembrar a cadeia **pegadinha → regra → exemplo → resultado**.
