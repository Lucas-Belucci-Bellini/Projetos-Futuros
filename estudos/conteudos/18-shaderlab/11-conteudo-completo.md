# 11 — Conteúdo completo de ShaderLab

Este capítulo reúne a explicação principal da trilha em um único Markdown. Use-o junto dos módulos anteriores: leia uma seção, escreva o exemplo sem copiar e só depois consulte a documentação.

## 1. Modelo mental

ShaderLab declara a estrutura do shader; HLSL descreve operações executadas em muitos vértices e pixels.

A pergunta inicial deve ser: **qual problema estou resolvendo, quais dados entram, qual transformação acontece e qual resultado precisa sair?** Antes de adicionar bibliotecas, escreva uma versão pequena que prove a ideia.

## 2. Conceitos essenciais

shader; material; pass; propriedades; HLSL; vertex/fragment; texturas.

Para cada conceito, produza quatro registros:

1. uma definição com suas palavras;
2. um exemplo mínimo;
3. um erro que o conceito evita;
4. uma situação real em que ele é útil.

## 3. Exemplo guiado

Exemplo:
Shader "Custom/Color" { SubShader { Pass { } } }


Leia o exemplo em três passagens. Primeiro identifique entradas e saídas. Depois marque nomes, tipos e estruturas. Por fim, explique o que acontece quando a entrada é vazia, inválida ou maior que o esperado. Reescreva-o com nomes próprios e acrescente uma pequena variação.

## 4. Como estudar na prática

Comece com um arquivo mínimo e execute frequentemente. Faça uma mudança por vez. Quando algo falhar, copie a mensagem completa, formule uma hipótese e altere apenas o necessário para testar essa hipótese. No final, remova código experimental e deixe um caso de uso reproduzível.

Uma sessão de 60 minutos pode ser dividida assim:

- 10 minutos de recuperação ativa;
- 15 minutos de leitura e anotações;
- 25 minutos codando sem copiar;
- 10 minutos para teste, limpeza e registro.

## 5. Erros comuns

misturar espaços; custo de fragment; alpha incorreto; ignorar pipeline.

Para cada erro, crie deliberadamente um exemplo que falha, observe o sintoma e escreva a correção. A correção só conta como aprendida quando você consegue explicar a causa sem olhar a anotação.

## 6. Exercício principal

Implemente **shader de dissolve com parâmetro e máscara** em três versões:

- **Versão mínima:** funciona para um caso feliz.
- **Versão resistente:** valida entradas, trata falhas e registra diagnóstico.
- **Versão revisada:** separa responsabilidades, adiciona testes e documenta como executar.

Inclua pelo menos um caso normal, um caso limite e um caso inválido. Compare as versões: o que ficou mais claro, o que ficou mais lento e qual complexidade foi adicionada?

## 7. Checkpoint de explicação

Sem consultar, responda:

1. Qual é o modelo de execução desta tecnologia?
2. Quais dados e invariantes o programa precisa preservar?
3. Como você identifica uma falha de sintaxe, uma falha de lógica e uma falha de ambiente?
4. Qual decisão de design você tomou e qual alternativa descartou?
5. Como esse conhecimento aparece em um projeto real do seu GitHub?

Grave uma explicação de três minutos ou escreva-a aqui. Se não conseguir responder, volte ao conceito específico, não ao curso inteiro.

## 8. Entrega e revisão

A entrega deve conter código, instruções de execução, exemplo de entrada e saída, testes, limitações conhecidas e uma retrospectiva. Revise depois de 1 dia, 1 semana e 1 mês. Em cada revisão, altere ou reescreva uma parte sem consultar o capítulo.

**Estado atual:** __/5  
**Erro que mais se repete:**  
**Próxima prática:**  
