# Contrato inicial da API — v1

Base local: http://localhost:8080/api/v1. Respostas JSON em UTF-8. Datas em ISO 8601 com fuso horário. Valores monetários devem ser serializados como decimal em string para evitar arredondamento binário.

## GET /health
Resposta 200: {"status":"ok","service":"catalogo-api","version":"0.1.0"}

Não revelar variáveis de ambiente, segredos, stack traces ou detalhes internos.

## GET /products
Parâmetros opcionais:
- q: texto de busca.
- category_id: categoria.
- brand: marca.
- store_id: filtrar por loja com oferta publicável.
- min_price e max_price: limites não negativos.
- page: começa em 1; padrão 1.
- page_size: padrão 24, máximo 100.
- sort: relevance, name_asc, price_asc ou price_desc.

Resposta proposta:
{"data":[{"id":"product-id","name":"Nome verificado","brand":null,"category":{"id":"category-id","name":"Categoria"},"unit":"un","identifiers":[],"offers_count":0}],"pagination":{"page":1,"page_size":24,"total_items":0,"total_pages":0}}

## GET /products/{id}
Retorna detalhes canônicos e ofertas publicáveis, incluindo loja, modalidade, preço, disponibilidade, URL oficial e observed_at. Se não existir produto publicável, responder 404.

## GET /categories
Retorna categorias ativas em ordem hierárquica.

## GET /stores
Lista lojas verificadas. Filtros previstos: q, neighborhood, page e page_size.

## Erros
Formato: {"error":{"code":"INVALID_QUERY","message":"Parâmetro de busca inválido.","request_id":"id-da-requisicao"}}

Códigos previstos: INVALID_QUERY, NOT_FOUND, RATE_LIMITED, INTERNAL_ERROR e SERVICE_UNAVAILABLE. Mensagens públicas não devem incluir SQL, caminhos locais ou stack traces.

## Compatibilidade
- Prefixo /api/v1 para evitar quebrar clientes.
- Não alterar silenciosamente o significado de campos existentes.
- Campos opcionais ausentes ou nulos devem ser tratados pelo frontend.
- Paginação e filtros precisam ser validados no backend.
- Preço precisa de moeda e data de observação.
- A interface não pode definir aprovação de fontes.
- IDs são strings opacas; o cliente não deve inferir sua estrutura.
