# Manual explicado de C#

Este arquivo é uma aula de consulta para **C#**. Use-o antes dos exemplos, durante a prática e na revisão. O objetivo é entender o código o suficiente para explicar, modificar, testar e aplicar em um trabalho acadêmico ou profissional.

## Como estudar cada código

1. Leia o título e diga qual problema o exemplo resolve.
2. Separe entrada, transformação, saída e efeitos colaterais.
3. Leia linha por linha: identifique palavras reservadas, nomes, operadores, delimitadores e valores.
4. Preveja o resultado antes de executar.
5. Execute o exemplo no ambiente correto.
6. Altere uma coisa por vez e registre a diferença.
7. Crie uma versão própria e explique a decisão tomada.

## Execução

Comando ou ambiente inicial:

    dotnet run

Registre a versão da ferramenta, o sistema operacional, a entrada, a saída e qualquer mensagem de erro. Isso torna o estudo reproduzível para você e para seus colegas.

## Sintaxe essencial

Tipos, classes e propriedades estruturam o programa; LINQ consulta coleções; `async/await` trabalha com tarefas.

Pergunte sempre: “qual parte cria dados?”, “qual parte transforma dados?”, “qual parte controla o fluxo?” e “qual parte produz a saída?”.

## Resultado e diagnóstico

O resultado pode ser texto no terminal, uma página renderizada, uma linha no banco, um arquivo, uma imagem, um target executado ou uma imagem/container. A ausência de texto no terminal não significa ausência de efeito.

Para diagnosticar: leia a mensagem exata, reduza o caso ao menor código que falha, confirme a versão e teste uma hipótese por vez.

## Erros comuns

Bloquear código assíncrono, não liberar recursos e ignorar valores nullable.

Em código profissional, valide entradas, trate falhas, evite segredos no código, escreva testes e documente decisões. Em atividades acadêmicas, anote hipótese, procedimento e resultado.

## Aplicações

- **Acadêmica:** algoritmos, relatórios, experimentos reproduzíveis, trabalhos e apresentações.
- **Profissional:** funções reutilizáveis, automação, APIs, dados, testes, build e manutenção.
- **Pessoal:** scripts, páginas, jogos, ferramentas, bancos de dados e laboratórios.

## Checklist de domínio

- [ ] Explico os tokens importantes sem copiar a solução.
- [ ] Prevejo a saída antes de executar.
- [ ] Modifico o exemplo sem quebrá-lo.
- [ ] Identifico dois erros comuns e sei investigá-los.
- [ ] Transformo o microexemplo em uma solução reutilizável.
- [ ] Consigo ensinar o exemplo a outra pessoa.

## Prática final

Escolha cinco exemplos desta trilha. Reescreva cada um, adicione uma validação, documente a saída e combine as ideias em um pequeno projeto ligado à faculdade, ao trabalho ou ao portfólio.

## Biblioteca detalhada

Consulte a [biblioteca de exemplos](./exemplos/README.md). Cada item contém código, leitura linha a linha, resultado esperado, conceitos, aplicação, erros, variação, exercício e checkpoint.
