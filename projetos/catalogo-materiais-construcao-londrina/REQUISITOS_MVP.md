# Requisitos do MVP

## Objetivo
Publicar uma primeira versão utilizável do catálogo local, com dados rastreáveis e sem afirmar cobertura total das lojas ou preços em tempo real.

## Perfis
- **Visitante:** busca produtos, filtra resultados, compara ofertas e abre a fonte original.
- **Responsável pelo catálogo:** revisa importações, corrige categorias e resolve duplicatas.
- **Representante de loja (futuro):** solicita correção por canal verificável.

## Requisitos funcionais
- **RF-01 Busca:** pesquisar por nome, marca, código e termos relevantes. Diferenças simples de caixa e espaços não devem impedir resultados, mas produtos distintos não podem ser fundidos.
- **RF-02 Categorias:** navegar pela taxonomia hierárquica.
- **RF-03 Filtros:** loja, categoria, marca, faixa de preço quando disponível, unidade e disponibilidade declarada.
- **RF-04 Detalhe:** mostrar nome, categoria, marca/código quando conhecidos, unidade, variante e fonte. Omitir dados desconhecidos.
- **RF-05 Comparação:** comparar ofertas do mesmo produto/variante ou explicar diferenças. Mostrar loja, preço, modalidade, condições, data e link.
- **RF-06 Atualidade:** exibir data de verificação e sinalizar ofertas antigas conforme política configurada.
- **RF-07 Loja:** exibir nome, site, endereço e contato somente quando confirmados.
- **RF-08 Correções:** permitir relatar dados incorretos; o relato não altera automaticamente o catálogo.
- **RF-09 Erros:** explicar ausência de resultados, indisponibilidade da fonte e preço não publicado.
- **RF-10 Acessibilidade:** busca e filtros funcionam em celular e desktop, com teclado, rótulos claros e contraste adequado.

## Requisitos não funcionais
- **RNF-01 Rastreabilidade:** cada produto/oferta publicado tem fonte e data.
- **RNF-02 Integridade:** importações repetidas não criam duplicatas.
- **RNF-03 Segurança:** validar entradas e tratar conteúdo importado como dado não confiável.
- **RNF-04 Desempenho:** busca paginada e índices adequados; nunca carregar dezenas de milhares de resultados de uma vez.
- **RNF-05 Privacidade:** coletar o mínimo de dados pessoais necessário.
- **RNF-06 Transparência:** indicar que o projeto é independente e não afiliado às lojas, salvo acordo explícito.
- **RNF-07 Manutenção:** registrar falhas, versão do importador e histórico dos lotes.

## Critérios de aceite
- Busca e filtros funcionam com um conjunto real de produtos.
- Produto e oferta são entidades separadas.
- Toda oferta publicada tem fonte, data e modalidade.
- Campos desconhecidos são mostrados como não informados.
- Duplicatas suspeitas não são fundidas automaticamente.
- Há testes para busca, filtros, importação, validação e expiração.
- Interface funciona em celular.
- Existe uma página explicando fontes, limites e correção de dados.

## Fora do primeiro MVP
Checkout, pagamentos, garantia de estoque em tempo real, promessa de menor preço de toda a cidade, conta obrigatória para pesquisar, rastreamento individual de visitantes e importação em massa de fontes não autorizadas.
